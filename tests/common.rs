// test

use kopeck::{Kopeck, OrderId, Payment};
use rust_decimal::Decimal;

#[test]
fn abc() {
    let amount = Kopeck::from_rub(Decimal::new(1, 1));
    // let builder = Payment::builder("a", amount, OrderId::I32(1)).build();
}
