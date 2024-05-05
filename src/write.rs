use std::env::set_var;
use std::error::Error;
use std::fmt::format;

pub struct Write;

impl Write {
    pub fn new(key: String, value: String) -> Result<(), String> {
        let env = set_var(key, value);

        match env {
            Some => {
                let result = format!("Error: env {} write", value);
                return Err(result);
            }
            None() => Ok(()),
        }
    }
}
