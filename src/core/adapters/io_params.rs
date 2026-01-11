use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ParamKey {
    ApiKey,
    Encoding,
}

impl ParamKey {
    pub fn as_str(&self) -> &str {
        match self {
            ParamKey::ApiKey => "X-API-Key",
            ParamKey::Encoding => "encoding",
        }
    }
}

pub enum ParamValue {
    Str(String),
}

impl ParamValue {
    pub fn as_str(&self) -> &str {
        match self {
            ParamValue::Str(s) => s.as_str(),
        }
    }
}
pub type Extras = HashMap<ParamKey, ParamValue>;
