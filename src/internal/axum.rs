use std::vec;

use tera::Tera;

pub fn global() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        (
            "Cargo.toml",
            include_str!("../../template/axum/Cargo.yiirs"),
        ),
        (
            ".dockerignore",
            include_str!("../../template/dockerignore.yiirs"),
        ),
        (".gitignore", include_str!("../../template/gitignore.yiirs")),
        (
            "README.md",
            include_str!("../../template/axum/README.yiirs"),
        ),
    ])
    .unwrap();
    tera
}

pub fn docker() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![(
        "Dockerfile",
        include_str!("../../template/Dockerfile.yiirs"),
    )])
    .unwrap();
    tera
}

pub fn other() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        ("dockerun.sh", include_str!("../../template/dockerun.yiirs")),
        ("config.toml", include_str!("../../template/config.yiirs")),
    ])
    .unwrap();
    tera
}

pub fn internal() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // lib.rs
        (
            "lib.rs",
            include_str!("../../template/axum/internal/lib.yiirs"),
        ),
        // core
        (
            "core/mod.rs",
            include_str!("../../template/axum/internal/core/mod.yiirs"),
        ),
        (
            "core/cache.rs",
            include_str!("../../template/axum/internal/core/cache.yiirs"),
        ),
        (
            "core/config.rs",
            include_str!("../../template/axum/internal/core/config.yiirs"),
        ),
        (
            "core/db.rs",
            include_str!("../../template/axum/internal/core/db.yiirs"),
        ),
        (
            "core/logger.rs",
            include_str!("../../template/axum/internal/core/logger.yiirs"),
        ),
        // middleware
        (
            "middleware/mod.rs",
            include_str!("../../template/axum/internal/middleware/mod.yiirs"),
        ),
        (
            "middleware/catch_panic.rs",
            include_str!("../../template/axum/internal/middleware/catch_panic.yiirs"),
        ),
        (
            "middleware/log.rs",
            include_str!("../../template/axum/internal/middleware/log.yiirs"),
        ),
        (
            "middleware/trace.rs",
            include_str!("../../template/axum/internal/middleware/trace.yiirs"),
        ),
        // result
        (
            "result/mod.rs",
            include_str!("../../template/axum/internal/result/mod.yiirs"),
        ),
        (
            "result/code.rs",
            include_str!("../../template/axum/internal/result/code.yiirs"),
        ),
        (
            "result/rejection.rs",
            include_str!("../../template/axum/internal/result/rejection.yiirs"),
        ),
        (
            "result/reply.rs",
            include_str!("../../template/axum/internal/result/reply.yiirs"),
        ),
        // util
        (
            "util/mod.rs",
            include_str!("../../template/axum/internal/util/mod.yiirs"),
        ),
        (
            "util/helper.rs",
            include_str!("../../template/axum/internal/util/helper.yiirs"),
        ),
        (
            "util/identity.rs",
            include_str!("../../template/axum/internal/util/identity.yiirs"),
        ),
    ])
    .unwrap();
    tera
}

pub fn app() -> Tera {
    let mut tera = Tera::default();
    // 使用 include_str! 宏将模板文件嵌入到二进制文件中
    tera.add_raw_templates(vec![
        // main.rs
        (
            "main.rs",
            include_str!("../../template/axum/app/main.yiirs"),
        ),
        // api
        (
            "api/mod.rs",
            include_str!("../../template/axum/app/api/mod.yiirs"),
        ),
        (
            "api/greeter.rs",
            include_str!("../../template/axum/app/api/greeter.yiirs"),
        ),
        // cmd
        (
            "cmd/mod.rs",
            include_str!("../../template/axum/app/cmd/mod.yiirs"),
        ),
        (
            "cmd/hello.rs",
            include_str!("../../template/axum/app/cmd/hello.yiirs"),
        ),
        (
            "cmd/serve.rs",
            include_str!("../../template/axum/app/cmd/serve.yiirs"),
        ),
        // middleware
        (
            "middleware/mod.rs",
            include_str!("../../template/axum/app/middleware/mod.yiirs"),
        ),
        (
            "middleware/auth.rs",
            include_str!("../../template/axum/app/middleware/auth.yiirs"),
        ),
        // router
        (
            "router/mod.rs",
            include_str!("../../template/axum/app/router/mod.yiirs"),
        ),
        (
            "router/route.rs",
            include_str!("../../template/axum/app/router/route.yiirs"),
        ),
        // service
        (
            "service/mod.rs",
            include_str!("../../template/axum/app/service/mod.yiirs"),
        ),
        (
            "service/greeter.rs",
            include_str!("../../template/axum/app/service/greeter.yiirs"),
        ),
    ])
    .unwrap();
    tera
}
