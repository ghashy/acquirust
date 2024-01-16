#![allow(dead_code)]

use phonenumber::PhoneNumber;
use serde::ser::SerializeSeq;
use serde::Serializer;

pub mod domain;
pub mod payment;
pub mod payment_data;
pub mod receipt;

pub use acquiconnect::AcquiClient;

use acquiconnect::ApiAction;
use rust_decimal::Decimal;
use serde::Deserialize;
use url::Url;

use self::payment::Payment;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentResponse {
    success: bool,
    error_code: String,
    payment_url: Option<Url>,
    terminal_key: Option<String>,
    status: Option<String>,
    payment_id: Option<u64>,
    order_id: Option<i32>,
    amount: Option<Decimal>,
    message: Option<String>,
    details: Option<String>,
}

pub struct InitPaymentAction;

impl ApiAction for InitPaymentAction {
    type Request = Payment;
    type Response = InitPaymentResponse;
    fn url_path(&self) -> &'static str {
        "Init"
    }
    async fn perform_action(
        req: Self::Request,
        addr: Url,
        client: &reqwest::Client,
    ) -> Result<Self::Response, acquiconnect::ClientError> {
        let response =
            client.post(addr).json(&req.inner()).send().await.unwrap();
        Ok(response.json().await?)
    }
}

// ───── Functions ────────────────────────────────────────────────────────── //

pub(crate) fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub(crate) fn serialize_phonenumber<S>(
    number: &Option<PhoneNumber>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match number {
        Some(number) => serializer.serialize_str(
            &number.format().mode(phonenumber::Mode::E164).to_string(),
        ),
        None => serializer.serialize_none(),
    }
}

pub(crate) fn serialize_phonenumber_vec<S>(
    numbers: &Option<Vec<PhoneNumber>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match numbers {
        Some(numbers) => {
            let vec: Vec<_> = numbers
                .iter()
                .map(|number| {
                    number.format().mode(phonenumber::Mode::E164).to_string()
                })
                .collect();
            // Now we serialize the collected vector of formatted phone numbers.
            let mut seq = serializer.serialize_seq(Some(vec.len()))?;
            for element in vec {
                seq.serialize_element(&element)?;
            }
            seq.end()
        }
        None => serializer.serialize_none(),
    }
}
