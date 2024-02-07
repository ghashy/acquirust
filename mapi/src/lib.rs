#![allow(dead_code)]

use phonenumber::PhoneNumber;
use rust_decimal::Decimal;
use serde::ser::SerializeSeq;
use serde::Deserialize;
use serde::Serializer;
use time::format_description::well_known::iso8601;
use time::format_description::well_known::iso8601::TimePrecision;
use time::format_description::well_known::Iso8601;
use url::Url;

pub use acquiconnect::AcquiClient;
use acquiconnect::ApiAction;

use self::payment::Payment;

pub mod domain;
pub mod notifications;
pub mod payment;
pub mod payment_data;
pub mod receipt;

const SIMPLE_ISO: Iso8601<6651332276402088934156738804825718784> = Iso8601::<
    {
        iso8601::Config::DEFAULT
            .set_year_is_six_digits(false)
            .set_time_precision(TimePrecision::Second {
                decimal_digits: None,
            })
            .encode()
    },
>;

time::serde::format_description!(iso_format, OffsetDateTime, SIMPLE_ISO);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InitPaymentResponse {
    success: bool,
    /// Код ошибки. «0» в случае успеха
    error_code: String,
    /// Ссылка на платежную форму (параметр возвращается только для Мерчантов без PCI DSS)
    payment_url: Option<Url>,
    /// Идентификатор терминала. Выдается Мерчанту Тинькофф Кассой при заведении терминала.
    terminal_key: String,
    /// Статус транзакции
    status: String,
    /// Идентификатор платежа в системе Тинькофф Кассы
    payment_id: u64,
    /// Идентификатор заказа в системе Мерчанта
    order_id: i32,
    /// Сумма в копейках
    amount: Decimal,
    /// Краткое описание ошибки
    message: Option<String>,
    /// Подробное описание ошибки
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
