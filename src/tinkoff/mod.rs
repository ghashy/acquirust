mod payment;
mod payment_data;

#[derive(Debug, Clone)]
struct TinkoffPaymentService;

impl TinkoffPaymentService {
    fn new() -> Self {
        TinkoffPaymentService
    }
}
