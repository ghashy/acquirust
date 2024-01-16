use std::collections::BTreeMap;

use garde::Validate;
use serde::{ser::Error, Serialize, Serializer};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use url::Url;

use super::payment_data::{OperationInitiatorType, PaymentData};
use crate::domain::Kopeck;
use crate::error_chain_fmt;
use crate::mapi::receipt::Receipt;

pub enum OrderId {
    I32(i32),
    UUID(uuid::Uuid),
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OrderId::I32(id) => id.to_string(),
            OrderId::UUID(id) => id.to_string(),
        };
        f.write_str(&s)
    }
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

#[derive(thiserror::Error)]
pub enum ShopParseError {
    #[error("Name is {0}, but max is 128")]
    NameTooLongError(usize),
}

impl std::fmt::Debug for ShopParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// Данные маркетплейса.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Shop {
    /// Код магазина
    shop_code: String,
    /// Cумма в копейках, которая относится к указанному ShopCode
    amount: Kopeck,
    /// Наименование товара
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>, // <= 128
    /// Сумма комиссии в копейках, удерживаемая из возмещения Партнера
    /// в пользу Маркетплейса. Если не передано, используется комиссия,
    /// указанная при регистрации.
    #[serde(skip_serializing_if = "Option::is_none")]
    fee: Option<Kopeck>,
}

impl Shop {
    pub fn new(
        shop_code: &str,
        amount: Kopeck,
        name: Option<String>,
        fee: Option<Kopeck>,
    ) -> Result<Shop, ShopParseError> {
        if let Some(ref name) = name {
            if name.len() > 128 {
                return Err(ShopParseError::NameTooLongError(name.len()));
            }
        }
        Ok(Shop {
            shop_code: shop_code.to_string(),
            amount,
            name,
            fee,
        })
    }
}

#[derive(Debug)]
pub enum TerminalType {
    /// ECOM – это терминалы, предназначенные для электронной коммерции.
    /// Они могут использоваться в розничной торговле для обработки платежных карт,
    /// мобильных платежей и других видов электронных платежей.
    /// Такие терминалы обычно предоставляют возможность безналичной оплаты
    /// за товары и услуги в интернет-магазинах или в торговых точках.
    ECOM,
    /// AFT – это автоматизированные терминалы сбора платежей,
    /// часто используемые в транспортной системе для оплаты проезда.
    /// К примеру, это могут быть терминалы для продажи билетов или
    /// пополнения транспортных карт в метро,
    /// на автобусных станциях или на железнодорожных вокзалах.
    AFT,
}

#[derive(thiserror::Error)]
pub enum PaymentParseError {
    #[error("Validation error")]
    ValidationError(#[from] garde::Report),
    #[error("Failed to parse date")]
    DateParseError(#[from] time::Error),
    #[error("Given OperationInitiatorType: {0:?} is not compatible with recurrent Init method")]
    NotAllowedWithInitError(OperationInitiatorType),
    #[error("Given OperationInitiatorType: {0:?} is not compatible with given terminal type: {1:?}")]
    NotCompatibleTerminalError(OperationInitiatorType, TerminalType),
}

impl std::fmt::Debug for PaymentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub struct Payment(PaymentBuilder);

impl Payment {
    pub fn builder(
        terminal_key: &str,
        amount: Kopeck,
        order_id: OrderId,
        terminal_type: TerminalType,
    ) -> PaymentBuilder {
        PaymentBuilder {
            terminal_key: terminal_key.to_string(),
            amount,
            order_id,
            description: None,
            customer_key: None,
            recurrent: "N".to_string(),
            pay_type: None,
            language: None,
            notification_url: None,
            success_url: None,
            fail_url: None,
            redirect_due_date: None,
            data: None,
            receipt: None,
            shops: None,
            descriptor: None,
            token: None,
            terminal_type,
        }
    }
    pub(super) fn inner(&self) -> &PaymentBuilder {
        &self.0
    }
    pub fn innertest(&self) -> &PaymentBuilder {
        &self.0
    }
}

#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct PaymentBuilder {
    #[garde(length(max = 20))]
    terminal_key: String,
    amount: Kopeck,
    order_id: OrderId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 250))]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 36))]
    customer_key: Option<String>,
    recurrent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay_type: Option<PayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "NotificationURL")]
    notification_url: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "SuccessURL")]
    success_url: Option<url::Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "FailURL")]
    fail_url: Option<url::Url>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_date_rfc3339"
    )]
    redirect_due_date: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PaymentData>,
    receipt: Option<Receipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shops: Option<Vec<Shop>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    descriptor: Option<String>,
    token: Option<String>,
    #[serde(skip)]
    terminal_type: TerminalType,
}

impl PaymentBuilder {
    /// Описание заказа.
    ///
    /// Поле необходимо обязательно заполнять для осуществления привязки
    /// и одновременной оплаты по CБП. При оплате через СБП данная информация
    /// будет отображена в приложении мобильного банка клиента.
    /// Максимально допустимое количество знаков для передачи назначения
    /// платежа в СБП - 140 символов.
    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
    /// Идентификатор клиента в системе Мерчанта.
    /// Обязателен, если Recurrent установлен в `true`,
    /// (`false` по умолчанию).
    /// Если был передан в запросе, в нотификации будет указан
    /// CustomerKey и его CardId. См. метод GetCardList.
    /// Необходим для сохранения карт на платежной форме (платежи в один клик).
    /// Не является обязательным при реккурентных платежах через СБП.
    pub fn with_customer_key(mut self, key: String) -> Self {
        self.customer_key = Some(key);
        self
    }
    /// Для регистрации автоплатежа - обязателен.
    pub fn with_recurrent(mut self, is: bool) -> Self {
        self.recurrent = if is { "Y".to_string() } else { "N".to_string() };
        self
    }
    /// Определяет тип проведения платежа – двухстадийная или одностадийная оплата.
    pub fn with_paytype(mut self, pay_type: PayType) -> Self {
        self.pay_type = Some(pay_type);
        self
    }
    /// Язык платежной формы.
    pub fn with_lang(mut self, lang: Language) -> Self {
        self.language = Some(lang);
        self
    }
    /// URL на веб-сайте Мерчанта, куда будет отправлен POST запрос
    /// о статусе выполнения вызываемых методов.
    /// Если параметр передан – используется его значение.
    /// Если нет – значение в настройках терминала.
    pub fn with_notification_url(mut self, url: Url) -> Self {
        self.notification_url = Some(url);
        self
    }
    /// URL на веб-сайте Мерчанта, куда будет переведен клиент
    /// в случае успешной оплаты.
    /// Если параметр передан – используется его значение.
    /// Если нет – значение в настройках терминала.
    pub fn with_success_url(mut self, url: Url) -> Self {
        self.success_url = Some(url);
        self
    }
    /// URL на веб-сайте Мерчанта, куда будет переведен клиент
    /// в случае неуспешной оплаты.
    /// Если параметр передан – используется его значение.
    /// Если нет – значение в настройках терминала.
    pub fn with_fail_url(mut self, url: Url) -> Self {
        self.fail_url = Some(url);
        self
    }
    /// При выставлении счета через Личный кабинет:
    /// В случае, если параметр RedirectDueDate не был передан,
    /// проверяется настроечный параметр платежного терминала REDIRECT_TIMEOUT,
    /// который может содержать значение срока жизни ссылки в часах.
    /// Если его значение больше нуля, то оно будет установлено в качестве
    /// срока жизни ссылки или динамического QR-кода.
    /// Иначе, устанавливается значение «по умолчанию» - 1440 мин.(1 сутки)
    pub fn with_redirect_due_date(mut self, date: OffsetDateTime) -> Self {
        self.redirect_due_date = Some(date);
        self
    }
    /// Тип, который позволяет передавать дополнительные параметры
    /// по операции и задавать определенные настройки в формате "ключ":"значение".
    pub fn with_payment_data(mut self, data: PaymentData) -> Self {
        self.data = Some(data);
        self
    }
    /// Тип с данными чека.
    /// Обязателен, если подключена онлайн-касса.
    pub fn with_receipt(mut self, receipt: Receipt) -> Self {
        self.receipt = Some(receipt);
        self
    }
    /// Объект с данными Маркетплейса. Обязательный для маркетплейсов
    pub fn with_shops(mut self, shops: Vec<Shop>) -> Self {
        self.shops = Some(shops);
        self
    }
    /// Динамический дескриптор точки
    pub fn with_descriptor(mut self, desc: String) -> Self {
        self.descriptor = Some(desc);
        self
    }
    pub fn build(mut self) -> Result<Payment, PaymentParseError> {
        self.validate(&())?;
        if let Some(ref pd) = self.data {
            if let Some(init_type) = pd.initiator_type() {
                if self.recurrent.eq("Y")
                    && !init_type.allowed_with_recurrent_init()
                {
                    return Err(PaymentParseError::NotAllowedWithInitError(
                        init_type.clone(),
                    ));
                }
                if init_type
                    .validate_terminal_type(&self.terminal_type)
                    .is_err()
                {
                    return Err(PaymentParseError::NotCompatibleTerminalError(
                        init_type.clone(),
                        self.terminal_type,
                    ));
                }
            }
        }
        let token = self.generate_token()?;
        self.token = Some(token);
        Ok(Payment(self))
    }

    fn generate_token(&self) -> Result<String, PaymentParseError> {
        let mut token_map = BTreeMap::new();
        token_map.insert("TerminalKey", self.terminal_key.clone());
        token_map.insert("Amount", self.amount.to_string());
        token_map.insert("OrderId", self.order_id.to_string());
        token_map.insert("Recurrent", self.recurrent.clone());
        if let Some(ref desc) = self.description {
            token_map.insert("Description", desc.clone());
        }
        if let Some(ref key) = self.customer_key {
            token_map.insert("CustomerKey", key.clone());
        }
        if let Some(ref url) = self.notification_url {
            token_map.insert("NotificationURL", url.clone().into());
        }
        if let Some(ref url) = self.success_url {
            token_map.insert("SuccessURL", url.clone().into());
        }
        if let Some(ref url) = self.fail_url {
            token_map.insert("FailURL", url.clone().into());
        }
        if let Some(ref date) = self.redirect_due_date {
            let date = format_date_rfc3339(date)?;
            token_map.insert("RedirectDueDate", date);
        }
        if let Some(ref desc) = self.descriptor {
            token_map.insert("Descriptor", desc.clone());
        }
        let concantenated = token_map.into_values().collect::<String>();

        // Hash the concatenated string with SHA-256
        let mut hasher: Sha256 = Digest::new();
        hasher.update(concantenated);
        let hash_result = hasher.finalize();

        // Convert hash result to a hex string
        let token = format!("{:x}", hash_result);
        Ok(token)
    }
}

// ───── Functions ────────────────────────────────────────────────────────── //

fn serialize_date_rfc3339<S>(
    date: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(date) => {
            let formatted_date =
                format_date_rfc3339(date).map_err(S::Error::custom)?;
            serializer.serialize_str(&formatted_date)
        }
        None => {
            // Serialize as a null value:
            serializer.serialize_none()
        }
    }
}

fn format_date_rfc3339(date: &OffsetDateTime) -> Result<String, time::Error> {
    let formatted_date =
        date.format(&time::format_description::well_known::Rfc3339)?;
    Ok(formatted_date)
}

// ───── Tests ────────────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test1() {
        let b = PaymentBuilder {
            terminal_key: "termkey".to_string(),
            amount: Kopeck::from_rub(Decimal::new(1000, 2)).unwrap(),
            order_id: OrderId::UUID(uuid::Uuid::new_v4()),
            description: None,
            customer_key: None,
            recurrent: String::from("N"),
            pay_type: None,
            language: None,
            notification_url: None,
            success_url: None,
            fail_url: None,
            redirect_due_date: Some(OffsetDateTime::now_utc()),
            data: None,
            receipt: None,
            shops: None,
            descriptor: None,
            token: None,
            terminal_type: TerminalType::ECOM,
        };
        let s = serde_json::to_string_pretty(&b).unwrap();
        println!("{s}");
    }

    #[test]
    fn test2() {
        use sha2::{Digest, Sha256};

        let mut hasher: Sha256 = Digest::new();
        hasher.update("19200Подарочная карта на 1000 рублей21090usaf8fw8fsw21gMerchantTerminalKey");
        // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
        // Note that calling `finalize()` consumes hasher
        let hash = hasher.finalize();
        let token = format!("{:x}", hash);
        println!("Gott hash: {}", token);
        println!("Should be: 0024a00af7c350a3a67ca168ce06502aa72772456662e38696d48b56ee9c97d9")
    }
}
