pub trait PaymentService {
    type PaymentData;
    type InitPaymentResponse;
    async fn init_payment(
        &self,
        data: Self::PaymentData,
    ) -> Self::InitPaymentResponse;
}
