// test

use kopeck::domain::{Email, Kopeck};
use kopeck::mapi::payment::receipt::item::{Ffd105Data, Item, SupplierInfo};
use kopeck::mapi::payment::receipt::{FfdVersion, Receipt};
use kopeck::mapi::payment::{OrderId, Payment, TerminalType};
use kopeck::mapi::payment_data::{OperationInitiatorType, PaymentData};
use rust_decimal::Decimal;

#[test]
fn abc() {
    let amount = Kopeck::from_rub(Decimal::new(10, 0)).unwrap();
    let item = Item::builder(
        "abc",
        Kopeck::from_rub("12".parse().unwrap()).unwrap(),
        "12".parse().unwrap(),
        Kopeck::from_rub("10".parse().unwrap()).unwrap(),
        kopeck::mapi::payment::receipt::item::VatType::None,
        kopeck::mapi::payment::receipt::item::CashBoxType::Atol,
    )
    .with_ffd_105_data(Ffd105Data::builder().build().unwrap())
    .with_supplier_info(
        SupplierInfo::new(
            Some(vec!["+79112211999".parse().unwrap()]),
            None,
            None,
        )
        .unwrap(),
    )
    .build()
    .unwrap();
    let receipt = Receipt::builder(
        kopeck::mapi::payment::receipt::Taxation::UsnIncomeOutcome,
    )
    .with_ffd_version(FfdVersion::Ver1_05)
    .with_phone("+79210127878".parse().unwrap())
    .add_item(item)
    .build()
    .unwrap();
    let payment_data = PaymentData::builder()
        .with_operation_initiator_type(OperationInitiatorType::CIT_CNC)
        .with_phone("+79312211603".parse().unwrap())
        .with_email(Email::parse("ghashy@gmail.com").unwrap())
        .build()
        .unwrap();
    let payment =
        Payment::builder("a", amount, OrderId::I32(1), TerminalType::ECOM)
            .with_payment_data(payment_data)
            .with_receipt(receipt)
            .build()
            .unwrap();

    let json = serde_json::to_string_pretty(&payment.innertest()).unwrap();
    println!("{json}");
}

fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    let subscriber = tracing_subscriber::fmt()
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::TRACE.into()),
        )
        .compact()
        .with_level(true)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}
