use rust_decimal::Decimal;
use serde::Serialize;

use crate::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum KopeckParseError {
    #[error("Wrong scale")]
    WrongScale(#[from] rust_decimal::Error),
    #[error("Number can't be negative for Kopeck")]
    NumberIsNegativeError,
    #[error("Number is too big")]
    OverflowError,
}

impl std::fmt::Debug for KopeckParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Serialize)]
pub struct Kopeck(u32);

impl Kopeck {
    /// Scale should be equal 2, and mantissa length should be <= 10 symbols.
    pub fn from_rub(mut rub: Decimal) -> Result<Kopeck, KopeckParseError> {
        if rub.scale() != 2 {
            tracing::warn!(
                "Given rub decimal scale is not 2, but {}",
                rub.scale()
            );
            rub.set_scale(2)?;
        }
        if rub.is_sign_negative() {
            return Err(KopeckParseError::NumberIsNegativeError);
        }
        let mantissa = rub.mantissa();
        if mantissa > u32::MAX as i128 {
            return Err(KopeckParseError::OverflowError);
        }
        let kopeck = mantissa as u32;
        Ok(Kopeck(kopeck))
    }
}

impl std::fmt::Display for Kopeck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}
