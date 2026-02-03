use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ParamKey {
    ApiKey,
    Encoding,
}

impl ParamKey {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::ApiKey => "X-API-Key",
            Self::Encoding => "encoding",
        }
    }
}

pub enum ParamValue {
    Str(String),
}

impl ParamValue {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Str(s) => s.as_str(),
        }
    }
}
pub type Extras = HashMap<ParamKey, ParamValue>;
