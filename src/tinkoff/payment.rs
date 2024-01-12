use rust_decimal::Decimal;
use time::OffsetDateTime;

const AMOUNT_MAX_LEN: usize = 10;
const TERMINAL_KEY_MAX_LEN: usize = 20;

#[allow(non_camel_case_types, dead_code)]
pub struct PCI_DSS;

#[allow(non_camel_case_types, dead_code)]
pub struct NON_PCI_DSS;

pub struct TerminalKey(String);

impl TerminalKey {
    pub fn new(key: &str) -> Result<Self, ()> {
        if key.len() > TERMINAL_KEY_MAX_LEN {
            Err(())
        } else {
            Ok(TerminalKey(key.to_string()))
        }
    }
}
pub struct Amount(Decimal);

impl Amount {
    pub fn new(mut amount: Decimal) -> Result<Self, ()> {
        if amount.scale() != 2 {
            amount.rescale(2);
        }
        let decimal = amount.to_string();
        if decimal.len() > AMOUNT_MAX_LEN {
            Err(())
        } else {
            Ok(Amount(amount))
        }
    }
}

pub enum OrderId {
    I32(i32),
    UUID(uuid::Uuid),
}

// Если параметр передан - используется его значение.
// Если нет - значение в настройках терминала.
pub enum PayType {
    // Одностадийная оплата
    O,
    // Двухстадийная оплата
    T,
}

// Язык платежной формы.
pub enum Language {
    RU,
    EN,
}

pub struct Payment;

impl Payment {
    pub fn builder(
        terminal_key: &str,
        amount: Decimal,
        order_id: OrderId,
    ) -> Result<PaymentBuilder, ()> {
        Ok(PaymentBuilder {
            terminal_key: TerminalKey::new(terminal_key)?,
            amount: Amount::new(amount)?,
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
        })
    }
}

pub struct PaymentBuilder {
    terminal_key: TerminalKey,
    amount: Amount,
    order_id: OrderId,
    description: Option<String>, // <= 250 characters
    // Идентификатор клиента в системе Мерчанта.
    // Обязателен, если передан атрибут Recurrent.
    // Если был передан в запросе, в нотификации будет указан
    // CustomerKey и его CardId. См. метод GetCardList.
    // Необходим для сохранения карт на платежной форме (платежи в один клик).
    // Не является обязательным при реккурентных платежах через СБП.
    customer_key: Option<String>, // <= 36 characters
    // Для регистрации автоплатежа - обязателен.
    is_recurrent: bool,
    pay_type: Option<PayType>,
    language: Option<Language>,
    // URL на веб-сайте Мерчанта, куда будет отправлен POST запрос
    // о статусе выполнения вызываемых методов.
    // Если параметр передан – используется его значение.
    // Если нет – значение в настройках терминала.
    notification_url: Option<url::Url>,
    // URL на веб-сайте Мерчанта, куда будет переведен клиент
    // в случае успешной оплаты.
    // Если параметр передан – используется его значение.
    // Если нет – значение в настройках терминала.
    success_url: Option<url::Url>,
    // URL на веб-сайте Мерчанта, куда будет переведен клиент
    // в случае неуспешной оплаты.
    // Если параметр передан – используется его значение.
    // Если нет – значение в настройках терминала.
    fail_url: Option<url::Url>,
    // При выставлении счета через Личный кабинет:
    // В случае, если параметр RedirectDueDate не был передан,
    // проверяется настроечный параметр платежного терминала REDIRECT_TIMEOUT,
    // который может содержать значение срока жизни ссылки в часах.
    // Если его значение больше нуля, то оно будет установлено в качестве
    // срока жизни ссылки или динамического QR-кода.
    // Иначе, устанавливается значение «по умолчанию» - 1440 мин.(1 сутки)
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
        let max = i32::MAX;
        println!("{max},\nLength: {}", max.to_string().len());
        let a = "9018A108-B268-42D7-B230-B27FBF266091";
        println!("len2: {}", a.len());
    }
}
