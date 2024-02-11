use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CountryCode(String);

impl CountryCode {
    pub fn new(code: &str) -> Result<CountryCode, ()> {
        if code.len() != 3 {
            Err(())
        } else {
            let _ = code.parse::<u16>().map_err(|_| ())?;
            Ok(CountryCode(code.to_string()))
        }
    }
}
