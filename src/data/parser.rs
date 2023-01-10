use super::serializers::*;
use serde_json::{from_str, Value, Map};
use std::include_str;


type S = &'static str;
type Payload = (S, S, S, S, S, S, S, Option<S>);

/// Contains compile-time parsed json of specified locale
pub struct ParsedData {
    pub lang_code: String,
    pub address: Address,
    pub builtin: Map<String, Value>,
    pub datetime: Datetime,
    pub finance: Finance,
    pub food: Food,
    pub person: Person,
    pub text: Text,
}

impl From<Payload> for ParsedData {
    fn from(payload: Payload) -> Self {
        let lang_code = payload.0;

        let builtin = match payload.7 {
            Some(b) => match from_str(b).expect(&format!("Can't load builtin from {lang_code}")) {
                Value::Object(builtin) => builtin,
                _ => panic!("Builtin from {lang_code} misformed!")
            },
            None => Map::default(),
        };

        Self {
            lang_code: lang_code.to_string(),
            address:   from_str(payload.1).expect(&format!("Can't load address from {lang_code}")),
            builtin:   builtin,
            datetime:  from_str(payload.2).expect(&format!("Can't load datetime from {lang_code}")),
            finance:   from_str(payload.3).expect(&format!("Can't load finance from {lang_code}")),
            food:      from_str(payload.4).expect(&format!("Can't load food from {lang_code}")),
            person:    from_str(payload.5).expect(&format!("Can't load person from {lang_code}")),
            text:      from_str(payload.6).expect(&format!("Can't load text from {lang_code}")),
        }
    }
}
