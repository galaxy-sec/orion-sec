use std::{env, path::PathBuf};

use log::{info, warn};
use orion_conf::{TomlIO, YamlIO};
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
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from(DEFAULT_FALLBACK_DIR));
    let local_candidate = current_dir.join(name);

    if local_candidate.exists() {
        return local_candidate;
    }

    match resolve_home_dir() {
        Some(home) => home.join(name),
        None => {
            warn!(target: "exec", "  HOME not set; defaulting to current directory for {}", name);
            local_candidate
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
    use std::ffi::OsString;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, MutexGuard, OnceLock};
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
        with_temp_home(|home_path| {
            let dot_dir = home_path.join(".myapp");
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
        });
    }

    #[test]
    fn test_load_sec_dict_by_toml() {
        with_temp_home(|home_path| {
            let dot_dir = home_path.join(".config");
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
        });
    }

    #[test]
    fn test_load_sec_dict_by_nonexistent_dir() {
        with_temp_home(|_| {
            let result = load_sec_dict_by(".nonexistent", "file.yml", SecFileFmt::Yaml);
            assert!(result.is_ok());
            assert!(result.unwrap().is_empty());
        });
    }

    #[test]
    fn test_load_sec_dict_by_values_not_secret() {
        with_temp_home(|home_path| {
            let dot_dir = home_path.join(".test");
            fs::create_dir_all(&dot_dir).unwrap();

            let sec_file = dot_dir.join("data.yml");
            let mut file = fs::File::create(&sec_file).unwrap();
            writeln!(file, "value: test_data").unwrap();

            let result = load_sec_dict_by(".test", "data.yml", SecFileFmt::Yaml);
            assert!(result.is_ok());

            let dict = result.unwrap();
            // EnvDict 中的值已经通过 no_sec() 转换，不再是 secret
            assert!(dict.contains_key("SEC_VALUE"));
        });
    }

    #[test]
    fn test_dot_path_prefers_current_dir_before_home() {
        let workspace = TempDir::new().unwrap();
        let workspace_path = fs::canonicalize(workspace.path()).unwrap();
        let local_dot_dir = workspace_path.join(GALAXY_DOT_DIR);
        fs::create_dir_all(&local_dot_dir).unwrap();

        with_temp_home(|home_path| {
            let home_dot_dir = home_path.join(GALAXY_DOT_DIR);
            fs::create_dir_all(&home_dot_dir).unwrap();

            let _cwd_guard = CurrentDirGuard::set(&workspace_path);
            let resolved = dot_path(GALAXY_DOT_DIR);
            assert_eq!(resolved, local_dot_dir);
        });
    }

    #[test]
    fn test_dot_path_falls_back_to_home_when_local_missing() {
        let workspace = TempDir::new().unwrap();
        let workspace_path = fs::canonicalize(workspace.path()).unwrap();

        with_temp_home(|home_path| {
            let home_dot_dir = home_path.join(GALAXY_DOT_DIR);
            fs::create_dir_all(&home_dot_dir).unwrap();

            let _cwd_guard = CurrentDirGuard::set(&workspace_path);
            let resolved = dot_path(GALAXY_DOT_DIR);
            assert_eq!(resolved, home_dot_dir);
        });
    }

    #[test]
    fn test_load_sec_dict_by_prefers_current_dir() {
        let workspace = TempDir::new().unwrap();
        let workspace_path = fs::canonicalize(workspace.path()).unwrap();
        let dot_name = ".pref";
        let local_dot_dir = workspace_path.join(dot_name);
        fs::create_dir_all(&local_dot_dir).unwrap();
        let local_file = local_dot_dir.join("data.yml");
        let mut local_writer = fs::File::create(&local_file).unwrap();
        writeln!(local_writer, "local_only: true").unwrap();

        with_temp_home(|home_path| {
            let home_dot_dir = home_path.join(dot_name);
            fs::create_dir_all(&home_dot_dir).unwrap();
            let home_file = home_dot_dir.join("data.yml");
            let mut home_writer = fs::File::create(&home_file).unwrap();
            writeln!(home_writer, "home_only: true").unwrap();

            let _cwd_guard = CurrentDirGuard::set(&workspace_path);
            let dict = load_sec_dict_by(dot_name, "data.yml", SecFileFmt::Yaml).unwrap();
            assert!(dict.contains_key("SEC_LOCAL_ONLY"));
            assert!(!dict.contains_key("SEC_HOME_ONLY"));
        });
    }

    fn with_temp_home<F>(test: F)
    where
        F: FnOnce(&Path),
    {
        let temp_dir = TempDir::new().unwrap();
        let _guard = HomeGuard::set(temp_dir.path());
        test(temp_dir.path());
    }

    struct HomeGuard {
        old_home: Option<OsString>,
        _lock: MutexGuard<'static, ()>,
    }

    impl HomeGuard {
        fn set(path: &Path) -> Self {
            let lock = home_lock().lock().unwrap_or_else(|err| err.into_inner());
            let old_home = env::var_os("HOME");
            unsafe {
                env::set_var("HOME", path);
            }

            Self {
                old_home,
                _lock: lock,
            }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            if let Some(ref home) = self.old_home {
                unsafe {
                    env::set_var("HOME", home);
                }
            } else {
                unsafe {
                    env::remove_var("HOME");
                }
            }
        }
    }

    fn home_lock() -> &'static Mutex<()> {
        static HOME_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        HOME_MUTEX.get_or_init(|| Mutex::new(()))
    }

    struct CurrentDirGuard {
        old_dir: PathBuf,
        _lock: MutexGuard<'static, ()>,
    }

    impl CurrentDirGuard {
        fn set(path: &Path) -> Self {
            let lock = current_dir_lock()
                .lock()
                .unwrap_or_else(|err| err.into_inner());
            let old_dir = env::current_dir().unwrap();
            env::set_current_dir(path).unwrap();

            Self {
                old_dir,
                _lock: lock,
            }
        }
    }

    impl Drop for CurrentDirGuard {
        fn drop(&mut self) {
            env::set_current_dir(&self.old_dir).unwrap();
        }
    }

    fn current_dir_lock() -> &'static Mutex<()> {
        static CURRENT_DIR_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        CURRENT_DIR_MUTEX.get_or_init(|| Mutex::new(()))
    }
}
