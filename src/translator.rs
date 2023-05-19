use crate::translator::api_responses::TranslationResponse;
use crate::translator::language::Language;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::sync::{Arc, Mutex};

mod api_responses;
mod language;
pub mod ui;

struct Translator {
    from: Language,
    to: Language,
    from_text: String,
    to_text: Arc<Mutex<String>>,
}

impl Default for Translator {
    fn default() -> Self {
        Self {
            from: Language::EN,
            to: Language::ES,
            from_text: "".to_string(),
            to_text: Arc::new(Mutex::new("".to_string())),
        }
    }
}

impl Translator {
    pub fn translate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let from = self.from.as_code();
        let to = self.to.as_code();

        let encode_set = NON_ALPHANUMERIC.to_owned();
        encode_set.add(b' ');
        encode_set.add(b'\n');

        let text = utf8_percent_encode(self.from_text.as_str(), encode_set).to_string();

        let url = format!("https://translate.google.com/translate_a/single?client=gtx&sl={from}&tl={to}&dt=t&q={text}");

        let to_text_clone = self.to_text.clone();

        tokio::spawn(async move {
            let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();
            let mut data = to_text_clone.lock().unwrap();
            *data = Self::process_response(resp);
        });

        Ok(())
    }

    fn process_response(response: String) -> String {
        let json: Result<TranslationResponse, _> = serde_json::from_str(&response);
        let json = match json {
            Ok(json) => json,
            Err(e) => {
                println!("Error: {}", e);
                return "".to_string();
            }
        };

        let mut complete_response = String::new();

        if let Some(inner_arr) = json.data.get(0) {
            for inner_inner_arr in inner_arr {
                if let Some(s) = inner_inner_arr.get(0) {
                    println!("s: {:?}", s);
                    complete_response.push_str(&s.to_string());
                }
            }
        };

        complete_response
    }
}
