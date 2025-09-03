use std::{env::home_dir, path::PathBuf};

use log::info;
use orion_conf::Yamlable;
use orion_error::{ErrorOwe, ErrorWith};
use orion_variate::vars::{EnvDict, ValueDict};
use unicase::UniCase;

use crate::{
    error::SecResult,
    sec::{NoSecConv, SecFrom, SecValueObj, SecValueType},
};

pub fn load_sec_dict() -> SecResult<EnvDict> {
    let space = load_secfile()?;
    let mut dict = EnvDict::new();
    for (k, v) in space.no_sec() {
        dict.insert(k, v);
    }
    //let dict = EnvDict::from(space.no_sec());
    Ok(dict)
}

pub fn load_secfile() -> SecResult<SecValueObj> {
    let env_path = std::env::var("GAL_SEC_FILE_PATH").map(PathBuf::from);
    let default = sec_value_default_path();
    let path = env_path.unwrap_or(default);
    let mut vars_dict = SecValueObj::new();
    if path.exists() {
        let dict = ValueDict::from_yml(&path).owe_logic().with(&path)?;
        info!(target: "exec","  load {}", path.display());
        for (k, v) in dict.iter() {
            vars_dict.insert(
                UniCase::from(format!("SEC_{}", k.as_str().to_uppercase())),
                SecValueType::sec_from(v.clone()),
            );
        }
    }
    Ok(vars_dict)
}

pub fn sec_value_default_path() -> PathBuf {
    galaxy_dot_path().join("sec_value.yml")
}

pub fn galaxy_dot_path() -> PathBuf {
    home_dir()
        .map(|x| x.join(".galaxy"))
        .unwrap_or(PathBuf::from("./"))
}
