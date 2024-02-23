use acquiconnect::{ApiAction, Client, ClientError, Url};

struct InitPayment;

struct Request;
struct Response;

impl ApiAction for InitPayment {
    type Request = Request;
    type Response = Response;

    fn url_path(&self) -> &'static str {
        "Init"
    }

    async fn perform_action(
        req: Self::Request,
        addr: Url,
        client: &Client,
    ) -> Result<Self::Response, ClientError> {
        todo!()
    }
}
