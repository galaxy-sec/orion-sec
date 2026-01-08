use std::{env, path::PathBuf};

use log::{info, warn};
use orion_conf::{TomlIO, Yamlable};
use orion_error::{ErrorOwe, ErrorWith};
use orion_variate::vars::UpperKey;
use orion_variate::vars::{EnvDict, ValueDict};

use crate::{
    error::SecResult,
    sec::{NoSecConv, SecFrom, SecValueObj, SecValueType},
};

const SEC_PREFIX: &str = "SEC_";
const SEC_VALUE_FILE_NAME: &str = "sec_value.yml";
const GALAXY_DOT_DIR: &str = ".galaxy";
const DEFAULT_FALLBACK_DIR: &str = "./";

pub fn load_sec_dict() -> SecResult<EnvDict> {
    let space = load_secfile()?;
    let mut dict = EnvDict::new();
    for (k, v) in space.no_sec() {
        dict.insert(k, v);
    }
    Ok(dict)
}

pub fn load_sec_dict_by(dot_name: &str, file_name: &str, fmt: SecFileFmt) -> SecResult<EnvDict> {
    let sec_file = dot_path(dot_name).join(file_name);
    let space = load_secfile_by(sec_file, fmt)?;
    let mut dict = EnvDict::new();
    for (k, v) in space.no_sec() {
        dict.insert(k, v);
    }
    Ok(dict)
}

pub fn load_secfile() -> SecResult<SecValueObj> {
    let default = sec_value_galaxy_path();
    load_secfile_by(default, SecFileFmt::Yaml)
}

pub fn load_galaxy_secfile() -> SecResult<SecValueObj> {
    let default = sec_value_galaxy_path();
    load_secfile_by(default, SecFileFmt::Yaml)
}
pub enum SecFileFmt {
    Yaml,
    Toml,
}

pub fn load_secfile_by(sec_file: PathBuf, fmt: SecFileFmt) -> SecResult<SecValueObj> {
    let mut vars_dict = SecValueObj::new();
    if sec_file.exists() {
        let dict = match fmt {
            SecFileFmt::Yaml => ValueDict::load_yaml(&sec_file)
                .owe_logic()
                .with(&sec_file)?,
            SecFileFmt::Toml => ValueDict::load_toml(&sec_file)
                .owe_logic()
                .with(&sec_file)?,
        };
        info!(target: "exec","  load {}", sec_file.display());
        for (k, v) in dict.iter() {
            vars_dict.insert(
                UpperKey::from(format!("{}{}", SEC_PREFIX, k.as_str().to_uppercase())),
                SecValueType::sec_from(v.clone()),
            );
        }
    }
    Ok(vars_dict)
}

pub fn sec_value_galaxy_path() -> PathBuf {
    dot_path(GALAXY_DOT_DIR).join(SEC_VALUE_FILE_NAME)
}

pub fn dot_path(name: &str) -> PathBuf {
    match resolve_home_dir() {
        Some(home) => home.join(name),
        None => {
            warn!(target: "exec", "  HOME not set; defaulting to current directory for {}", GALAXY_DOT_DIR);
            PathBuf::from(DEFAULT_FALLBACK_DIR)
        }
    }
}

fn resolve_home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
        .or_else(|| {
            let drive = env::var_os("HOMEDRIVE")?;
            let path = env::var_os("HOMEPATH")?;
            let mut buf = PathBuf::from(drive);
            buf.push(path);
            Some(buf)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_load_secfile_by_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/path/to/file.yml");
        let result = load_secfile_by(path, SecFileFmt::Yaml);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_load_secfile_by_yaml() {
        let mut file = NamedTempFile::with_suffix(".yml").unwrap();
        writeln!(file, "username: admin").unwrap();
        writeln!(file, "password: secret123").unwrap();
        writeln!(file, "port: 8080").unwrap();

        let result = load_secfile_by(file.path().to_path_buf(), SecFileFmt::Yaml);
        assert!(result.is_ok());

        let obj = result.unwrap();
        assert_eq!(obj.len(), 3);
        assert!(obj.contains_key(&UpperKey::from("SEC_USERNAME".to_string())));
        assert!(obj.contains_key(&UpperKey::from("SEC_PASSWORD".to_string())));
        assert!(obj.contains_key(&UpperKey::from("SEC_PORT".to_string())));
    }

    #[test]
    fn test_load_secfile_by_toml() {
        let mut file = NamedTempFile::with_suffix(".toml").unwrap();
        writeln!(file, "api_key = \"abc123\"").unwrap();
        writeln!(file, "debug = true").unwrap();

        let result = load_secfile_by(file.path().to_path_buf(), SecFileFmt::Toml);
        assert!(result.is_ok());

        let obj = result.unwrap();
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key(&UpperKey::from("SEC_API_KEY".to_string())));
        assert!(obj.contains_key(&UpperKey::from("SEC_DEBUG".to_string())));
    }

    #[test]
    fn test_load_secfile_by_key_uppercase() {
        let mut file = NamedTempFile::with_suffix(".yml").unwrap();
        writeln!(file, "mixedCase: value1").unwrap();
        writeln!(file, "lower_case: value2").unwrap();

        let result = load_secfile_by(file.path().to_path_buf(), SecFileFmt::Yaml);
        assert!(result.is_ok());

        let obj = result.unwrap();
        assert!(obj.contains_key(&UpperKey::from("SEC_MIXEDCASE".to_string())));
        assert!(obj.contains_key(&UpperKey::from("SEC_LOWER_CASE".to_string())));
    }

    #[test]
    fn test_load_secfile_by_values_are_secret() {
        let mut file = NamedTempFile::with_suffix(".yml").unwrap();
        writeln!(file, "token: my_secret_token").unwrap();

        let result = load_secfile_by(file.path().to_path_buf(), SecFileFmt::Yaml);
        assert!(result.is_ok());

        let obj = result.unwrap();
        let value = obj.get(&UpperKey::from("SEC_TOKEN".to_string())).unwrap();
        assert!(matches!(value, SecValueType::String(s) if s.is_secret()));
    }

    #[test]
    fn test_load_secfile_by_empty_file() {
        let file = NamedTempFile::with_suffix(".yml").unwrap();

        let result = load_secfile_by(file.path().to_path_buf(), SecFileFmt::Yaml);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_load_sec_dict_by_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let old_home = env::var("HOME").ok();
        unsafe { env::set_var("HOME", temp_dir.path()) };

        let dot_dir = temp_dir.path().join(".myapp");
        fs::create_dir_all(&dot_dir).unwrap();

        let sec_file = dot_dir.join("secrets.yml");
        let mut file = fs::File::create(&sec_file).unwrap();
        writeln!(file, "db_user: root").unwrap();
        writeln!(file, "db_pass: password123").unwrap();

        let result = load_sec_dict_by(".myapp", "secrets.yml", SecFileFmt::Yaml);
        assert!(result.is_ok());

        let dict = result.unwrap();
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("SEC_DB_USER"));
        assert!(dict.contains_key("SEC_DB_PASS"));

        if let Some(home) = old_home {
            unsafe { env::set_var("HOME", home) };
        }
    }

    #[test]
    fn test_load_sec_dict_by_toml() {
        let temp_dir = TempDir::new().unwrap();
        let old_home = env::var("HOME").ok();
        unsafe { env::set_var("HOME", temp_dir.path()) };

        let dot_dir = temp_dir.path().join(".config");
        fs::create_dir_all(&dot_dir).unwrap();

        let sec_file = dot_dir.join("app.toml");
        let mut file = fs::File::create(&sec_file).unwrap();
        writeln!(file, "secret_key = \"abc123\"").unwrap();
        writeln!(file, "enabled = true").unwrap();

        let result = load_sec_dict_by(".config", "app.toml", SecFileFmt::Toml);
        assert!(result.is_ok());

        let dict = result.unwrap();
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("SEC_SECRET_KEY"));
        assert!(dict.contains_key("SEC_ENABLED"));

        if let Some(home) = old_home {
            unsafe { env::set_var("HOME", home) };
        }
    }

    #[test]
    fn test_load_sec_dict_by_nonexistent_dir() {
        let temp_dir = TempDir::new().unwrap();
        let old_home = env::var("HOME").ok();
        unsafe { env::set_var("HOME", temp_dir.path()) };

        let result = load_sec_dict_by(".nonexistent", "file.yml", SecFileFmt::Yaml);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());

        if let Some(home) = old_home {
            unsafe { env::set_var("HOME", home) };
        }
    }

    #[test]
    fn test_load_sec_dict_by_values_not_secret() {
        let temp_dir = TempDir::new().unwrap();
        let old_home = env::var("HOME").ok();
        unsafe { env::set_var("HOME", temp_dir.path()) };

        let dot_dir = temp_dir.path().join(".test");
        fs::create_dir_all(&dot_dir).unwrap();

        let sec_file = dot_dir.join("data.yml");
        let mut file = fs::File::create(&sec_file).unwrap();
        writeln!(file, "value: test_data").unwrap();

        let result = load_sec_dict_by(".test", "data.yml", SecFileFmt::Yaml);
        assert!(result.is_ok());

        let dict = result.unwrap();
        // EnvDict 中的值已经通过 no_sec() 转换，不再是 secret
        assert!(dict.contains_key("SEC_VALUE"));

        if let Some(home) = old_home {
            unsafe { env::set_var("HOME", home) };
        }
    }
}
