use garde::Validate;
use rust_decimal::Decimal;
use serde::{Serialize, Serializer};
use time::OffsetDateTime;

use crate::domain::kopeck::Kopeck;

pub enum OrderId {
    I32(i32),
    UUID(uuid::Uuid),
}

impl Serialize for OrderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            OrderId::I32(ref i) => serializer.serialize_i32(*i),
            OrderId::UUID(ref u) => {
                serializer.serialize_str(u.to_string().as_str())
            }
        }
    }
}

// Если параметр передан - используется его значение.
// Если нет - значение в настройках терминала.
#[derive(Serialize)]
pub enum PayType {
    // Одностадийная оплата
    O,
    // Двухстадийная оплата
    T,
}

// Язык платежной формы.
#[derive(Serialize)]
pub enum Language {
    RU,
    EN,
}

pub struct Payment;

impl Payment {
    pub fn builder(
        terminal_key: &str,
        amount_rub: Decimal,
        order_id: OrderId,
    ) -> Result<PaymentBuilder, ()> {
        let payment_builder = PaymentBuilder {
            terminal_key: terminal_key.to_string(),
            amount: Kopeck::from_rub(amount_rub).map_err(|e| ())?,
            order_id,
            description: None,
            customer_key: None,
            is_recurrent: false,
            pay_type: None,
            language: None,
            notification_url: None,
            success_url: None,
            fail_url: None,
            redirect_due_date: None,
        };
        Ok(payment_builder)
    }
}

#[derive(Serialize, Validate)]
#[garde(allow_unvalidated)]
pub struct PaymentBuilder {
    #[garde(length(max = 20))]
    terminal_key: String,
    amount: Kopeck,
    order_id: OrderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>, // <= 250 characters
    /// Идентификатор клиента в системе Мерчанта.
    /// Обязателен, если передан атрибут Recurrent.
    /// Если был передан в запросе, в нотификации будет указан
    /// CustomerKey и его CardId. См. метод GetCardList.
    /// Необходим для сохранения карт на платежной форме (платежи в один клик).
    /// Не является обязательным при реккурентных платежах через СБП.
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_key: Option<String>, // <= 36 characters
    /// Для регистрации автоплатежа - обязателен.
    is_recurrent: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_type: Option<PayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    /// URL на веб-сайте Мерчанта, куда будет отправлен POST запрос
    /// о статусе выполнения вызываемых методов.
    /// Если параметр передан – используется его значение.
    /// Если нет – значение в настройках терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_url: Option<url::Url>,
    /// URL на веб-сайте Мерчанта, куда будет переведен клиент
    /// в случае успешной оплаты.
    /// Если параметр передан – используется его значение.
    /// Если нет – значение в настройках терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    success_url: Option<url::Url>,
    /// URL на веб-сайте Мерчанта, куда будет переведен клиент
    /// в случае неуспешной оплаты.
    /// Если параметр передан – используется его значение.
    /// Если нет – значение в настройках терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_url: Option<url::Url>,
    /// При выставлении счета через Личный кабинет:
    /// В случае, если параметр RedirectDueDate не был передан,
    /// проверяется настроечный параметр платежного терминала REDIRECT_TIMEOUT,
    /// который может содержать значение срока жизни ссылки в часах.
    /// Если его значение больше нуля, то оно будет установлено в качестве
    /// срока жизни ссылки или динамического QR-кода.
    /// Иначе, устанавливается значение «по умолчанию» - 1440 мин.(1 сутки)
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_due_date: Option<OffsetDateTime>,
}

impl PaymentBuilder {
    pub fn build() -> Payment {
        Payment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let b = PaymentBuilder {
            terminal_key: "termkey".to_string(),
            amount: Kopeck::from_rub(Decimal::new(1000, 2)).unwrap(),
            order_id: OrderId::UUID(uuid::Uuid::new_v4()),
            description: None,
            customer_key: None,
            is_recurrent: false,
            pay_type: None,
            language: None,
            notification_url: None,
            success_url: None,
            fail_url: None,
            redirect_due_date: None,
        };
        let s = serde_json::to_string_pretty(&b).unwrap();
        println!("{s}");
    }
}
