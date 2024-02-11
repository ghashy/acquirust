// test

use mapi::domain::{Email, Kopeck};
use mapi::payment::{OrderId, Payment, TerminalType};
use mapi::payment_data::{OperationInitiatorType, PaymentData};
use mapi::receipt::item::{
    CashBoxType, Ffd105Data, Item, SupplierInfo, VatType,
};
use mapi::receipt::{FfdVersion, Receipt, Taxation};
use mapi::InitPaymentAction;
use rust_decimal::Decimal;

#[tokio::test]
async fn abc() {
    let amount = Kopeck::from_rub(Decimal::new(10, 0)).unwrap();
    let item = Item::builder(
        "abc",
        "12".parse().unwrap(),
        "12".parse().unwrap(),
        "10".parse().unwrap(),
        VatType::None,
        Some(CashBoxType::Atol),
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
    let receipt = Receipt::builder(Taxation::UsnIncomeOutcome)
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

    let client =
        mapi::AcquiClient::new("https://securepay.tinkoff.ru/v2").unwrap();
    let response = client.execute(InitPaymentAction, payment).await.unwrap();
    dbg!(response);
}

fn _init_tracing() {
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
