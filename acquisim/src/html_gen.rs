use askama::Template;

#[derive(Template)]
// escape = "none": override the template's extension used
// for the purpose of determining the escaper for this template.
// {{ "Escape <>&"|e }} with escape will be this: Escape &lt;&gt;&amp;
// So we disable this
#[template(path = "index.html", escape = "none")]
pub struct SubmitPaymentPage {
    price: u64,
}

impl SubmitPaymentPage {
    pub fn new(price: u64) -> Self {
        SubmitPaymentPage { price }
    }
}

#[cfg(test)]
mod tests {
    use askama::Template;

    use super::SubmitPaymentPage;

    #[test]
    fn test_template_creation() {
        let page = SubmitPaymentPage::new(10);
        assert!(page.render().is_ok())
    }
}
