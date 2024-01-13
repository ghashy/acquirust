// test

use kopeck::{PaymentManager, TinkoffPaymentService};

#[test]
fn abc() {
    let tinkoff = TinkoffPaymentService::with_pci_dss();
    let manager = PaymentManager::new(tinkoff);
}
