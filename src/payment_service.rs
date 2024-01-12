pub trait PaymentService {
    async fn init_payment(&self);
}
