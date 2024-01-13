use crate::payment_service::PaymentService;

mod payment;
mod payment_data;

#[derive(Debug, Clone)]
pub struct TinkoffPaymentService(());

impl TinkoffPaymentService {
    pub fn with_pci_dss() -> Self {
        TinkoffPaymentService(())
    }

    pub fn without_pci_dss() -> Self {
        TinkoffPaymentService(())
    }
}

pub struct TinkoffInitPaymentResponse;

impl PaymentService for TinkoffPaymentService {
    type InitPaymentResponse = TinkoffInitPaymentResponse;
    type PaymentData = payment::Payment;

    async fn init_payment(
        &self,
        data: payment::Payment,
    ) -> TinkoffInitPaymentResponse {
        TinkoffInitPaymentResponse
    }
}
