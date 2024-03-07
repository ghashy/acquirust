// benches/my_benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use tinkoff_mapi::domain::{Email, Kopeck};
use tinkoff_mapi::payment::{OrderId, Payment, TerminalType};
use tinkoff_mapi::payment_data::{OperationInitiatorType, PaymentData};
use tinkoff_mapi::receipt::item::{Ffd105Data, Item, SupplierInfo};
use tinkoff_mapi::receipt::{FfdVersion, Receipt};

fn benchmark_payment_json_creation(c: &mut Criterion) {
    c.bench_function("payment_json_creation", |b| {
        b.iter(|| {
            let amount = Kopeck::from_rub(Decimal::new(10, 0)).unwrap();
            let item = Item::builder(
                "abc",
                Kopeck::from_rub("12".parse().unwrap()).unwrap(),
                "12".parse().unwrap(),
                Kopeck::from_rub("10".parse().unwrap()).unwrap(),
                tinkoff_mapi::receipt::item::VatType::None,
                Some(tinkoff_mapi::receipt::item::CashBoxType::Atol),
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
                tinkoff_mapi::receipt::Taxation::UsnIncomeOutcome,
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
            let _payment = Payment::builder(
                "a",
                amount,
                OrderId::I32(1),
                TerminalType::ECOM,
            )
            .with_payment_data(payment_data)
            .with_receipt(receipt)
            .build()
            .unwrap();
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = benchmark_payment_json_creation
);
criterion_main!(benches);
