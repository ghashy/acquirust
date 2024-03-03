// Restafaire
use std::future::Future;

use reqwest::IntoUrl;
use url::Url;

pub use reqwest::Client;
pub use reqwest::StatusCode;

#[derive(thiserror::Error)]
pub enum ClientError {
    #[error("Request error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to parse url")]
    UrlError(#[from] url::ParseError),
}

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

/// This trait allows to generalize api actions behaviour, it assumes that
/// every action has:
/// `Request` type - some data as input
/// `Response` type - some data as output
/// And, using that model, we can define any entire backend in similar way:
/// ```rust
/// use serde::Deserialize;
/// use url::Url;
/// use acquiconnect::AcquiClient;
/// use acquiconnect::ClientError;
/// use acquiconnect::ApiAction;
///
/// // Define action struct
/// pub struct SayHello;
/// // Define request and response types
/// pub struct SimpleRequest(pub String);
/// #[derive(Deserialize)]
/// pub struct SimpleResponse(pub String);
///
/// // Implement `ApiAction` for action struct
/// impl ApiAction for SayHello {
///     type Request = SimpleRequest;
///     type Response = SimpleResponse;
///     fn url_path(&self) -> &'static str {
///         "SayHello"
///     }
///     async fn perform_action(
///         req: Self::Request,
///         _addr: Url,
///         _client: &reqwest::Client,
///     ) -> Result<Self::Response, ClientError> {
///         let name = req.0;
///         Ok(SimpleResponse(format!("Hello, {name}!")))
///     }
/// }
///
/// // Now we can use that action:
/// async fn run() {
/// let client = AcquiClient::new("https://happydog.org").unwrap();
/// let response = client
///     .execute(SayHello, SimpleRequest("Dog".to_string()))
///     .await
///     .unwrap();
/// }
/// ```
pub trait ApiAction {
    type Request;
    type Response;
    fn url_path(&self) -> &'static str;
    fn perform_action(
        req: Self::Request,
        addr: Url,
        client: &Client,
    ) -> impl Future<Output = Result<Self::Response, ClientError>> + Send;
}

impl std::fmt::Debug for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Clone, Debug)]
pub struct AcquiClient {
    client: Client,
    address: Url,
}

impl AcquiClient {
    pub fn new(url: impl IntoUrl) -> Result<Self, ClientError> {
        Ok(AcquiClient {
            client: Client::new(),
            address: url.into_url()?,
        })
    }
    pub async fn execute<T: ApiAction>(
        &self,
        action: T,
        data: T::Request,
    ) -> Result<T::Response, ClientError> {
        T::perform_action(
            data,
            self.address.join(action.url_path())?,
            &self.client,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use url::Url;

    use super::{AcquiClient, ApiAction, ClientError};

    pub struct SayHello;
    pub struct SimpleRequest(pub String);
    #[derive(Deserialize)]
    pub struct SimpleResponse(pub String);

    impl ApiAction for SayHello {
        type Request = SimpleRequest;
        type Response = SimpleResponse;
        fn url_path(&self) -> &'static str {
            "SayHello"
        }
        async fn perform_action(
            req: Self::Request,
            _addr: Url,
            _client: &reqwest::Client,
        ) -> Result<Self::Response, ClientError> {
            let name = req.0;
            Ok(SimpleResponse(format!("Hello, {name}!")))
        }
    }

    #[tokio::test]
    async fn it_works() {
        let client = AcquiClient::new("https://happydog.org").unwrap();
        let response = client
            .execute(SayHello, SimpleRequest("Dog".to_string()))
            .await
            .unwrap();
        assert_eq!(response.0, "Hello, Dog!".to_string())
    }
}
