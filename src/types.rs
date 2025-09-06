use indexmap::IndexMap;
use orion_variate::vars::UpperKey;

pub type UniString = UpperKey;
pub type UniCaseMap<T> = IndexMap<UpperKey, T>;
