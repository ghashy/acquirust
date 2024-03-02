use serde::{Deserialize, Serialize};

use crate::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum EmailError {
    #[error("Not valid error")]
    NotValidEmail,
}

impl std::fmt::Debug for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// This type guarantees correctness of `subscriber's` email address.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, EmailError> {
        if garde::rules::email::parse_email(email).is_ok() {
            Ok(Self(email.to_string()))
        } else {
            Err(EmailError::NotValidEmail)
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let valid_email: String = SafeEmail().fake_with_rng(&mut rng);
            assert!(Email::parse(&valid_email).is_ok());
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(Email::parse(&email).is_err())
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomail.com".to_string();
        assert!(Email::parse(&email).is_err())
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domail.com".to_string();
        assert!(Email::parse(&email).is_err())
    }
}
