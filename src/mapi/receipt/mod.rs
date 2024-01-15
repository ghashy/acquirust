use garde::Validate;
use phonenumber::PhoneNumber;
use rust_decimal::Decimal;
use serde::{ser::Error, ser::SerializeStruct, Serialize, Serializer};
use time::{macros::format_description, PrimitiveDateTime};

use crate::domain::CountryCode;
use crate::domain::Email;
use crate::domain::Kopeck;
use crate::error_chain_fmt;

use self::item::Item;
use self::item::PaymentMethod;

pub mod item;

pub static SIMPLE_DATE_FORMAT: &[time::format_description::FormatItem] =
    format_description!("[day].[month].[year]");

#[derive(Serialize)]
pub enum DocumentCode {
    #[serde(rename = "21")]
    PassportRussianCitizen,
    #[serde(rename = "22")]
    PassportRussianCitizenDiplomaticService,
    #[serde(rename = "26")]
    TemporaryIdentityCard,
    #[serde(rename = "27")]
    BirthCertificateRussianCitizen,
    #[serde(rename = "28")]
    OtherRussianCitizenIdentityDocument,
    #[serde(rename = "31")]
    ForeignCitizenPassport,
    #[serde(rename = "32")]
    OtherForeignCitizenIdentityDocument,
    #[serde(rename = "33")]
    ForeignDocumentRecognizedInternationalTreaty,
    #[serde(rename = "34")]
    ResidencePermit,
    #[serde(rename = "35")]
    TemporaryResidencePermit,
    #[serde(rename = "36")]
    RefugeeApplicationConsiderationCertificate,
    #[serde(rename = "37")]
    RefugeeCertificate,
    #[serde(rename = "38")]
    OtherStatelessPersonIdentityDocument,
    #[serde(rename = "40")]
    IdentityDocumentUnderConsideration,
}

impl DocumentCode {
    pub fn get_description(&self) -> &str {
        match self {
            DocumentCode::PassportRussianCitizen => "Паспорт гражданина Российской Федерации",
            DocumentCode::PassportRussianCitizenDiplomaticService => "Паспорт гражданина Российской Федерации, дипломатический паспорт, служебный паспорт, удостоверяющие личность гражданина Российской Федерации за пределами Российской Федерации",
            DocumentCode::TemporaryIdentityCard => "Временное удостоверение личности гражданина Российской Федерации, выдаваемое на период оформления паспорта гражданина Российской Федерации",
            DocumentCode::BirthCertificateRussianCitizen => "Свидетельство о рождении гражданина Российской Федерации (для граждан Российской Федерации в возрасте до 14 лет)",
            DocumentCode::OtherRussianCitizenIdentityDocument => "Иные документы, признаваемые документами, удостоверяющими личность гражданина Российской Федерации в соответствии с законодательством Российской Федерации",
            DocumentCode::ForeignCitizenPassport => "Паспорт иностранного гражданина",
            DocumentCode::OtherForeignCitizenIdentityDocument => "Иные документы, признаваемые документами, удостоверяющими личность иностранного гражданина в соответствии с законодательством Российской Федерации и международным договором Российской Федерации",
            DocumentCode::ForeignDocumentRecognizedInternationalTreaty => "Документ, выданный иностранным государством и признаваемый в соответствии с международным договором Российской Федерации в качестве документа, удостоверяющего личность лица безгражданства.",
            DocumentCode::ResidencePermit => "Вид на жительство (для лиц без гражданства)",
            DocumentCode::TemporaryResidencePermit => "Разрешение на временное проживание (для лиц без гражданства)",
            DocumentCode::RefugeeApplicationConsiderationCertificate => "Свидетельство о рассмотрении ходатайства о признании лица без гражданства беженцем на территории Российской Федерации по существу",
            DocumentCode::RefugeeCertificate => "Удостоверение беженца",
            DocumentCode::OtherStatelessPersonIdentityDocument => "Иные документы, признаваемые документами, удостоверяющими личность лиц без гражданства в соответствии с законодательством Российской Федерации и международным договором Российской Федерации",
            DocumentCode::IdentityDocumentUnderConsideration => "Документ, удостоверяющий личность лица, не имеющего действительного документа, удостоверяющего личность, на период рассмотрения заявления о признании гражданином Российской Федерации или о приеме в гражданство Российской Федерации",
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum FfdVersion {
    #[serde(rename = "1.2")]
    Ver1_2,
    #[serde(rename = "1.05")]
    Ver1_05,
}

/// Информация о клиенте. Обязательна для маркированных товаров.
#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct ClientInfo {
    /// Дата рождения клиента
    #[serde(serialize_with = "serialize_date_simple")]
    pub birth_date: PrimitiveDateTime,
    /// Цифровой код страны, гражданином которой является клиент.
    /// Код страны указывается в соответствии с Общероссийским
    /// классификатором стран мира (ОКСМ).
    pub citizenship: CountryCode,
    /// Цифровой код типа документа, удостоверяющего личность.
    pub document_code: DocumentCode,
    /// Детали документа, удостоверяющего личность
    /// (например, серия и номер паспорта).
    pub document_data: String,
    /// Адрес клиента или получателя.
    #[garde(length(max = 256))]
    pub address: String,
}

/// Система налогообложения
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Taxation {
    Osn,
    UsnIncome,
    UsnIncomeOutcome,
    Envd,
    Esn,
    Patent,
}

pub enum EmailOrPhone {
    Email(Email),
    Phone(phonenumber::PhoneNumber),
}

impl Serialize for EmailOrPhone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EmailOrPhone::Email(email) => {
                let mut state =
                    serializer.serialize_struct("EmailOrPhone", 1)?;
                let email = email.as_ref();
                if email.len() > 64 {
                    return Err(S::Error::custom(
                        "Email length exceeds 64 characters",
                    ));
                }
                state.serialize_field("Email", email)?;
                state.end()
            }
            EmailOrPhone::Phone(phone) => {
                let mut state =
                    serializer.serialize_struct("EmailOrPhone", 1)?;
                let phone_number =
                    phone.format().mode(phonenumber::Mode::E164).to_string();
                if phone_number.len() > 64 {
                    return Err(S::Error::custom(
                        "Phone number length exceeds 64 characters",
                    ));
                }
                state.serialize_field("Phone", &phone_number)?;
                state.end()
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payments {
    #[serde(skip_serializing_if = "Option::is_none")]
    cash: Option<Kopeck>,
    electronic: Kopeck,
    #[serde(skip_serializing_if = "Option::is_none")]
    advance_payment: Option<Kopeck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credit: Option<Kopeck>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provision: Option<Kopeck>,
}

impl Payments {
    ///
    /// # Аргументы
    ///
    /// * `electronic`: вид оплаты "Безналичный", (обязательный)
    pub fn builder(electronic: Kopeck) -> PaymentsBuilder {
        PaymentsBuilder {
            electronic,
            cash: None,
            advance_payment: None,
            credit: None,
            provision: None,
        }
    }
}

pub struct PaymentsBuilder {
    electronic: Kopeck,
    cash: Option<Kopeck>,
    advance_payment: Option<Kopeck>,
    credit: Option<Kopeck>,
    provision: Option<Kopeck>,
}

impl PaymentsBuilder {
    /// Вид оплаты "Наличные". Сумма к оплате в копейках
    pub fn with_cash(mut self, amount: Kopeck) -> Self {
        self.cash = Some(amount);
        self
    }
    /// Вид оплаты "Предварительная оплата (Аванс)"
    pub fn with_advance_payment(mut self, amount: Kopeck) -> Self {
        self.cash = Some(amount);
        self
    }
    /// Вид оплаты "Постоплата (Кредит)"
    pub fn with_credit(mut self, amount: Kopeck) -> Self {
        self.cash = Some(amount);
        self
    }
    /// Вид оплаты "Иная форма оплаты"
    pub fn with_provision(mut self, amount: Kopeck) -> Self {
        self.cash = Some(amount);
        self
    }
    pub fn build(self) -> Payments {
        Payments {
            cash: self.cash,
            electronic: self.electronic,
            advance_payment: self.advance_payment,
            credit: self.credit,
            provision: self.provision,
        }
    }
}

#[derive(thiserror::Error)]
pub enum ReceiptParseError {
    #[error("Wrong ffd is set")]
    FfdNotCompatibleError,
    #[error("Ffd is set, but not found in items")]
    FfdIsNotRepresentedInItems,
    #[error("Validation error")]
    ValidationError(#[from] garde::Report),
    #[error("For this ffd version: {0:?}, given values are not available")]
    WrongValuesForFfdVersion(FfdVersion),
}

impl std::fmt::Debug for ReceiptParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct Receipt {
    ffd_version: Option<FfdVersion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(dive)]
    client_info: Option<ClientInfo>,
    taxation: Taxation,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    email_or_phone: Option<EmailOrPhone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_inn: Option<String>,
    #[garde(dive)]
    items: Vec<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payments: Option<Payments>,
}

impl Receipt {
    pub fn builder(taxation: Taxation) -> ReceiptBuilder {
        ReceiptBuilder {
            ffd_version: None,
            client_info: None,
            taxation,
            email_or_phone: None,
            customer: None,
            customer_inn: None,
            items: Vec::new(),
            payments: None,
        }
    }
}

pub struct ReceiptBuilder {
    ffd_version: Option<FfdVersion>,
    client_info: Option<ClientInfo>,
    taxation: Taxation,
    email_or_phone: Option<EmailOrPhone>,
    customer: Option<String>,
    customer_inn: Option<String>,
    items: Vec<Item>,
    payments: Option<Payments>,
}

impl ReceiptBuilder {
    /// Задать версию ФФД.
    pub fn with_ffd_version(mut self, ver: FfdVersion) -> Self {
        self.ffd_version = Some(ver);
        self
    }
    /// Информация о клиенте. Обязательна для маркированных товаров.
    /// Только для ФФД 1.2.
    pub fn with_client_info(mut self, info: ClientInfo) -> Self {
        self.client_info = Some(info);
        self
    }
    /// Электронная почта клиента.
    /// Атрибут должен быть заполнен, если не передано значение в атрибуте “Phone”
    pub fn with_email(mut self, email: Email) -> Self {
        self.email_or_phone = Some(EmailOrPhone::Email(email));
        self
    }
    /// Телефон клиента в формате +{Ц}
    /// Атрибут должен быть заполнен, если не передано значение в атрибуте “Email”.
    pub fn with_phone(mut self, phone: PhoneNumber) -> Self {
        self.email_or_phone = Some(EmailOrPhone::Phone(phone));
        self
    }
    /// Идентификатор/Имя клиента. Только для ФФД 1.2.
    pub fn with_customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }
    /// Только для ФФД 1.2.
    pub fn with_customer_inn(mut self, inn: String) -> Self {
        self.customer_inn = Some(inn);
        self
    }
    /// Детали платежа.
    ///
    /// Если объект не передан, будет автоматически
    /// указана итоговая сумма чека с видом оплаты "Безналичный".
    /// Если передан, то значение в `Electronic` должно быть равно итоговому значению
    /// Amount в методе `Init`. При этом сумма введенных значений по всем видам оплат,
    /// включая `Electronic`, должна быть равна сумме (Amount) всех товаров,
    /// переданных в объекте `receipt.Items`.
    pub fn with_payments(mut self, payments: Payments) -> Self {
        self.payments = Some(payments);
        self
    }
    pub fn add_item(mut self, item: Item) -> Self {
        self.items.push(item);
        self
    }
    pub fn add_items(mut self, items: Vec<Item>) -> Self {
        self.items.extend(items);
        self
    }
    pub fn build(self) -> Result<Receipt, ReceiptParseError> {
        let receipt = Receipt {
            ffd_version: self.ffd_version,
            client_info: self.client_info,
            taxation: self.taxation,
            email_or_phone: self.email_or_phone,
            customer: self.customer,
            customer_inn: self.customer_inn,
            items: self.items,
            payments: self.payments,
        };
        receipt.validate(&())?;

        if let Some(ref ffd) = receipt.ffd_version {
            match ffd {
                FfdVersion::Ver1_2 => {
                    for item in receipt.items.iter() {
                        if item.ffd_105_data.is_some() {
                            return Err(
                                ReceiptParseError::FfdNotCompatibleError,
                            );
                        } else if item.ffd_12_data.is_none() {
                            return Err(
                                ReceiptParseError::FfdIsNotRepresentedInItems,
                            );
                        }
                    }
                }
                FfdVersion::Ver1_05 => {
                    for item in receipt.items.iter() {
                        if item.ffd_12_data.is_some() {
                            return Err(
                                ReceiptParseError::FfdNotCompatibleError,
                            );
                        } else if item.ffd_105_data.is_none() {
                            return Err(
                                ReceiptParseError::FfdIsNotRepresentedInItems,
                            );
                        }
                    }
                    if receipt.client_info.is_some()
                        || receipt.customer.is_some()
                        || receipt.customer_inn.is_some()
                    {
                        return Err(
                            ReceiptParseError::WrongValuesForFfdVersion(
                                ffd.clone(),
                            ),
                        );
                    }
                }
            }
        }
        Ok(receipt)
    }
}

// ───── Functions ────────────────────────────────────────────────────────── //

fn serialize_date_simple<S>(
    date: &PrimitiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(SIMPLE_DATE_FORMAT).map_err(S::Error::custom)?;
    serializer.serialize_str(&s)
}

fn is_valid_formatted_decimal_length(
    cash: Option<Decimal>,
    max_length: usize,
    scale: u32,
) -> bool {
    match cash {
        Some(value) => {
            let value_str = value.round_dp(scale).to_string(); // Rounds the Decimal to 'scale' decimal places before converting to string
            value_str.len() <= max_length
        }
        None => true, // Assuming a None value is also valid
    }
}
