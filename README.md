# yiirs

[<img alt="crates.io" src="https://img.shields.io/crates/v/yiirs.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/yiirs)
[<img alt="Apache 2.0 license" src="http://img.shields.io/badge/license-Apache%202.0-brightgreen.svg?style=for-the-badge" height="20">](http://opensource.org/licenses/apache2.0)

Rust Web开发脚手架，支持 `salvo` 和 `axum` 框架，并同时支持创建「单应用」和「多应用」

## 安装

```shell
cargo install yiirs
```

## 特点

- ORM使用 [sea-orm](https://github.com/SeaQL/sea-orm)
- Redis使用 [redis-rs](https://github.com/redis-rs/redis-rs)
- 日志使用 [tracing](https://github.com/tokio-rs/tracing)
- 配置使用 [config-rs](https://github.com/mehcode/config-rs)
- 命令行使用 [clap](https://github.com/clap-rs/clap)
- 异步运行时使用 [tokio](https://github.com/tokio-rs/tokio)
- 参数验证器使用 [validator](https://github.com/Keats/validator)
- 包含 Trace、认证、请求日志、Panic捕获 中间价
- 简单好用的 API Result 统一输出方式

## 创建项目

#### 单应用

```shell
yiirs new --name=demo # salvo
yiirs new --name=demo --axum # axum
.
├── Cargo.toml
├── Dockerfile
├── config.toml
└── src
    ├── app
    │   ├── api
    │   ├── cmd
    │   ├── middleware
    │   ├── router
    │   ├── service
    │   └── main.rs
    └── internal
```

#### 多应用

```shell
# http
yiirs new --name=demo --app=foo --app=bar # salvo
yiirs new --name=demo --app=foo --app=bar --axum # axum
.
├── Cargo.toml
├── foo.dockerfile
├── bar.dockerfile
├── foo_config.toml
├── bar_config.toml
└── src
    ├── app
    │   ├── foo
    │   │   ├── api
    │   │   ├── cmd
    │   │   ├── middleware
    │   │   ├── router
    │   │   ├── service
    │   │   └── main.rs
    │   ├── bar
    │   │   ├── ...
    │   │   └── main.rs
    └── internal
```

## 创建应用

```shell
# 多应用项目适用，需在项目根目录执行（即：Cargo.toml所在目录）
yiirs app --name=foo --name=bar # 创建salvo应用
yiirs app --name=foo --name=bar --axum # 创建axum应用
.
├── Cargo.toml
├── foo.dockerfile
├── bar.dockerfile
├── foo_config.toml
├── bar_config.toml
└── src
    ├── app
    │   ├── foo
    │   │   ├── ...
    │   │   └── main.rs
    │   ├── bar
    │   │   ├── ...
    │   │   └── main.rs
    └── internal
```

**Enjoy 😊**
