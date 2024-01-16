use acquiconnect::ApiAction;
use rust_decimal::Decimal;
use serde::Deserialize;
use url::Url;

use self::payment::Payment;

pub mod payment;
pub mod payment_data;
pub mod receipt;

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
