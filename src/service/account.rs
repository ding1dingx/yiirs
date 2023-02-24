use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use chrono::prelude::*;
use sea_orm::{
    ColumnTrait, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    config::db,
    entity::{account, prelude::*},
    result::{
        rejection::IRejection,
        response::{ApiErr, ApiOK, Result},
    },
    util::{
        self,
        auth::{Identity, Role},
        hash::{Algo, Hash},
    },
};

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ParamsCreate {
    #[validate(length(min = 1, message = "用户名必填"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码必填"))]
    pub password: String,
    #[validate(length(min = 1, message = "实名必填"))]
    pub realname: String,
}

pub async fn create(
    Extension(identity): Extension<Identity>,
    WithRejection(Json(params), _): IRejection<Json<ParamsCreate>>,
) -> Result<ApiOK<()>> {
    if !identity.is_role(Role::Super) {
        return Err(ApiErr::ErrPerm(None));
    }

    if let Err(err) = params.validate() {
        return Err(ApiErr::ErrParams(Some(err.to_string())));
    }

    match Account::find()
        .filter(account::Column::Username.eq(params.username.clone()))
        .count(db::get())
        .await
    {
        Err(err) => {
            tracing::error!(error = ?err, "err find account");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => {
            if v > 0 {
                return Err(ApiErr::ErrPerm(Some("该用户名已被使用".to_string())));
            }
        }
    }

    let salt = util::nonce(16);
    let pass = format!("{}{}", params.password, salt);

    let now = chrono::Local::now().timestamp();

    let model = account::ActiveModel {
        username: Set(params.username),
        password: Set(Hash(Algo::MD5).from_string(pass)),
        salt: Set(salt),
        role: Set(1),
        realname: Set(params.realname),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    if let Err(err) = Account::insert(model).exec(db::get()).await {
        tracing::error!(error = ?err, "err insert account");

        return Err(ApiErr::ErrSystem(None));
    }

    Ok(ApiOK(None))
}

#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub id: u64,
    pub username: String,
    pub realname: String,
    pub login_at: i64,
    pub login_at_str: String,
    pub created_at: i64,
    pub created_at_str: String,
}

pub async fn info(
    Extension(identity): Extension<Identity>,
    Path(account_id): Path<u64>,
) -> Result<ApiOK<RespInfo>> {
    if !identity.is_role(Role::Super) {
        return Err(ApiErr::ErrPerm(None));
    }

    let model = match Account::find_by_id(account_id).one(db::get()).await {
        Err(err) => {
            tracing::error!(error = ?err, "err find account");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => match v {
            None => return Err(ApiErr::ErrNotFound(Some("账号不存在".to_string()))),
            Some(model) => model,
        },
    };

    let mut resp = RespInfo {
        id: model.id,
        username: model.username,
        realname: model.realname,
        login_at: model.login_at,
        login_at_str: String::from(""),
        created_at: model.created_at,
        created_at_str: String::from(""),
    };

    if let Some(v) = Local.timestamp_opt(model.created_at, 0).single() {
        resp.created_at_str = v.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    if model.login_at > 0 {
        if let Some(v) = Local.timestamp_opt(model.login_at, 0).single() {
            resp.login_at_str = v.format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }

    Ok(ApiOK(Some(resp)))
}

#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

pub async fn list(
    Extension(identity): Extension<Identity>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<ApiOK<RespList>> {
    if !identity.is_role(Role::Super) {
        return Err(ApiErr::ErrPerm(None));
    }

    let mut builder = Account::find();

    if let Some(username) = query.get("username") {
        if username.len() > 0 {
            builder = builder.filter(account::Column::Username.eq(username.to_owned()));
        }
    }

    let (offset, limit) = util::query_page(&query);

    let mut total: i64 = 0;

    // 仅在第一页计算数量
    if offset == 0 {
        total = match builder
            .clone()
            .select_only()
            .column_as(account::Column::Id.count(), "count")
            .into_tuple::<i64>()
            .one(db::get())
            .await
        {
            Err(err) => {
                tracing::error!(error = ?err, "err count account");
                return Err(ApiErr::ErrSystem(None));
            }
            Ok(v) => v.unwrap_or_default(),
        }
    }

    let models = match builder
        .order_by(account::Column::Id, Order::Desc)
        .offset(offset)
        .limit(limit)
        .all(db::get())
        .await
    {
        Err(err) => {
            tracing::error!(error = ?err, "err find account");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(v) => v,
    };

    let mut resp = RespList {
        total,
        list: (Vec::with_capacity(models.len())),
    };

    for model in models {
        let mut info = RespInfo {
            id: model.id,
            username: model.username,
            realname: model.realname,
            login_at: model.login_at,
            login_at_str: String::from(""),
            created_at: model.created_at,
            created_at_str: String::from(""),
        };

        if let Some(v) = Local.timestamp_opt(model.created_at, 0).single() {
            info.created_at_str = v.format("%Y-%m-%d %H:%M:%S").to_string()
        }

        if model.login_at > 0 {
            if let Some(v) = Local.timestamp_opt(model.login_at, 0).single() {
                info.login_at_str = v.format("%Y-%m-%d %H:%M:%S").to_string()
            }
        }

        resp.list.push(info);
    }

    Ok(ApiOK(Some(resp)))
}
