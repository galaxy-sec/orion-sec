use std::{env, path::PathBuf};

use log::{info, warn};
use orion_conf::Yamlable;
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

pub fn load_secfile() -> SecResult<SecValueObj> {
    let default = sec_value_galaxy_path();
    load_secfile_by(default)
}

pub fn load_galaxy_secfile() -> SecResult<SecValueObj> {
    let default = sec_value_galaxy_path();
    load_secfile_by(default)
}

pub fn load_secfile_by(sec_file: PathBuf) -> SecResult<SecValueObj> {
    let mut vars_dict = SecValueObj::new();
    if sec_file.exists() {
        let dict = ValueDict::load_yaml(&sec_file)
            .owe_logic()
            .with(&sec_file)?;
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
    galaxy_dot_path().join(SEC_VALUE_FILE_NAME)
}

pub fn galaxy_dot_path() -> PathBuf {
    match resolve_home_dir() {
        Some(home) => home.join(GALAXY_DOT_DIR),
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
