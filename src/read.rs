use std::env;

pub struct Read;

impl Read {
    pub fn env(key: String) -> Result <String, String> {
        match env::var(key) {
            Ok(var) => {
                Ok(var)
            },
            Err(e) => {
                let error = format!("Error: read env {} return a error => {}", key, e);
                Err(error)
            }
        }
    }
}