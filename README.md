#orion-sec

[![CI](https://github.com/galaxy-sec/orion-sec/workflows/CI/badge.svg)](https://github.com/galaxy-sec/orion-sec/actions)
[![Coverage Status](https://codecov.io/gh/galaxy-sec/orion-sec/branch/main/graph/badge.svg)](https://codecov.io/gh/galaxy-sec/orion-sec)
[![crates.io](https://img.shields.io/crates/v/orion-sec.svg)](https://crates.io/crates/orion-sec)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 简介

`orion-sec` 提供面向 Galaxy 平台的安全配置加载与脱敏工具，帮助代理或服务在保留数据结构的同时区分明文与敏感字段。核心模块围绕 `SecValue<T>` 与 `SecValueType` 类型构建，可在读取 YAML 或环境变量后动态切换字段的明文或加密状态。

## 核心特性

- 统一的密文标记：通过 `SecValue<T>` 在同一结构中表示敏感与非敏感数值。
- 与 `orion-variate` 无缝协作：支持 `EnvDict`、`ValueType` 及 `UpperKey` 样式。
- 配置加载即脱敏：`load_secfile` 自动从 `~/.galaxy/sec_value.yml` 或自定义路径构造安全对象。
- 灵活的路径读取：`ValueGetter` trait 支持点语法与数组索引（如 `A[0].B`）。

## 安装与集成

```bash
cargo add orion-sec
```

或者在 `Cargo.toml` 中手动加入：

```toml
[dependencies]
orion-sec = "0.2"
```

## 快速上手

```rust
use orion_sec::{load_sec_dict, SecValueType, ValueGetter};

fn main() -> orion_sec::SecResult<()> {
    let dict = load_sec_dict()?; // 自动加载并去除 SEC_ 前缀
    if let Some(SecValueType::String(db_pass)) = dict.value_get("database.credentials.password") {
        println!("Password masked? {}", db_pass.is_secret());
    }
    Ok(())
}
```

如需测试不同路径，可设置 `GAL_SEC_FILE_PATH=/custom/sec.yml` 指向替代文件。

## 常用命令

- `cargo fmt --all`：统一格式化。
- `cargo clippy --all-targets --all-features -- -D warnings`：静态检查。
- `cargo test --all-features -- --test-threads=1`：运行核心测试用例。

## 贡献指南

提交 PR 前请确保通过格式化、Clippy、测试与（如适用）`cargo llvm-cov`。详细要求见 `AGENTS.md`。

## 许可证

本项目采用 [MIT License](LICENSE)。
