// use crate::payment_service::PaymentService;

pub mod payment;
pub mod payment_data;
pub mod receipt;

#[derive(Debug, Clone)]
pub struct MerchantWithPCIDSS;

impl MerchantWithPCIDSS {
    fn new() -> Self {
        MerchantWithPCIDSS
    }

    // fn init_payment(payment: Payment) {
    //     //
    // }
}

#[derive(Debug, Clone)]
pub struct MerchantWithoutPCIDSS;

impl MerchantWithoutPCIDSS {}

// impl PaymentService for MerchantWithoutPciDss {
//     type InitPaymentResponse = TinkoffInitPaymentResponse;
//     type PaymentData = payment::Payment;

//     async fn init_payment(
//         &self,
//         data: payment::Payment,
//     ) -> TinkoffInitPaymentResponse {
//         TinkoffInitPaymentResponse
//     }
// }
