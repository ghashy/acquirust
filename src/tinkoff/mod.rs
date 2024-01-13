// use crate::payment_service::PaymentService;

use crate::{Kopeck, OrderId, Payment};

pub(super) mod payment;
mod payment_data;

#[derive(Debug, Clone)]
pub struct MerchantWithPCIDSS;

impl MerchantWithPCIDSS {
    fn new() -> Self {
        MerchantWithPCIDSS
    }

    fn init_payment(terminal_key: String, amount: Kopeck, order_id: OrderId) {
        // let receipt = crate::Receipt::;
        let payment = Payment::builder(&terminal_key, amount, order_id)
            // .with_receipt()
            .build()
            .unwrap();
    }
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
