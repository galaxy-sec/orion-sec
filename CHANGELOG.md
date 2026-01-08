# Changelog

## [0.3.2] - 2026-01-08
### Added
- 增加 `SecReason` 与 `OrionSecReason` 的定义，提供 `SensitiveMsg`/`NoPermission`/`Deception`/`UnAuthenticated` 等场景化错误类型。
- 覆盖 `SecReason` 与 `SecError`/`SecResult` 的序列化、显示与转换测试用例，确保错误码与信息表现一致。
- `load.rs` 测试新增 `with_temp_home` 与 `HomeGuard`，统一封装临时 HOME 目录的创建与回收。

### Fixed
- 修复 `load_sec_dict_by_*` 测试直接操作全局 HOME 导致的竞态，确保在并行运行时不会互相覆盖环境变量。

## [0.3.0]
### Added
- 新增 `load_secfile_by` 函数测试用例，覆盖 YAML/TOML 加载、键名大写转换、空文件处理等场景

### Changed
- 重构：将硬编码字符串提取为常量，提升代码可维护性
  - `load.rs`: 新增 `SEC_PREFIX`、`SEC_VALUE_FILE_NAME`、`GALAXY_DOT_DIR`、`DEFAULT_FALLBACK_DIR` 常量
  - `sec.rs`: 新增 `SECRET_MASK` 常量用于密码掩码显示

## [0.2.0] - 2025-09-21
### Added
- 扩充 README 与 AGENTS，提供快速上手、命令与贡献说明。
- 新增 `CHANGELOG.md`，明确版本演进记录。

### Changed
- `ValueGetter::value_get` 现在按引用遍历，避免克隆整棵对象树。
- `Cargo.toml` 补充包元数据，提升 crates.io 展示质量。

### Fixed
- 修复 `SecValueType::to_sec`/`to_nor` 无法正确切换布尔密级的缺陷。
- 将跨类型比较改为返回 `None`，避免 `partial_cmp` 意外 panic。
- 更正 `SecReason::SensitiveMsg` 提示文本的拼写。
- 更新 `load::galaxy_dot_path`，在缺失 HOME 时回退并输出警告。

## [0.1.0]
### Added
- 初始发布，提供 `SecValue` 类型及 YAML 安全加载流程。
- 支持 `orion-variate` 的 `ValueType` 映射与 `UpperKey` 键规范。
- 提供基础 CI、Fmt、Clippy、Security Audit 与覆盖率流程。
