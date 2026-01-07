# Changelog

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
