use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug)]
pub struct TranslationResponse {
    pub data: Vec<Vec<Vec<String>>>,
}

impl<'de> Deserialize<'de> for TranslationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data: Vec<Value> = Deserialize::deserialize(deserializer)?;

        let mut result = Vec::new();
        for value in data {
            if let Some(inner) = value.as_array() {
                let mut inner_result = Vec::new();
                for inner_value in inner {
                    if let Some(inner_inner) = inner_value.as_array() {
                        let mut inner_inner_result = Vec::new();
                        for inner_inner_value in inner_inner {
                            if let Some(string) = inner_inner_value.as_str() {
                                inner_inner_result.push(string.to_owned());
                            }
                        }
                        inner_result.push(inner_inner_result);
                    }
                }
                result.push(inner_result);
            }
        }

        Ok(TranslationResponse { data: result })
    }
}
