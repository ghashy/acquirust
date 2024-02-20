use askama::Template;
use url::Url;
use uuid::Uuid;

#[derive(Template)]
#[allow(dead_code)]
// escape = "none": override the template's extension used
// for the purpose of determining the escaper for this template.
// {{ "Escape <>&"|e }} with escape will be this: Escape &lt;&gt;&amp;
// So we disable this
#[template(path = "index.html", escape = "none")]
pub struct SubmitPaymentPage {
    price: i64,
    payment_id: Uuid,
    post_payment_url: Url,
}

impl SubmitPaymentPage {
    pub fn new(price: i64, payment_id: Uuid, post_payment_url: Url) -> Self {
        SubmitPaymentPage {
            price,
            payment_id,
            post_payment_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use askama::Template;
    use uuid::Uuid;

    use super::SubmitPaymentPage;

    #[test]
    fn test_template_creation() {
        let page = SubmitPaymentPage::new(
            10,
            Uuid::new_v4(),
            "http://mydomain/path".parse().unwrap(),
        );
        assert!(page.render().is_ok())
    }
}
