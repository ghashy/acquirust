use garde::Validate;
use rust_decimal::Decimal;
use serde::{ser::Error, ser::SerializeStruct, Serialize, Serializer};
use time::{macros::format_description, PrimitiveDateTime};

use crate::domain::{country_code::CountryCode, email::Email, kopeck::Kopeck};

use self::item::Item;

mod item;

pub static SIMPLE_DATE_FORMAT: &[time::format_description::FormatItem] =
    format_description!("[day].[month].[year]");

#[derive(Serialize)]
enum DocumentCode {
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
    fn get_description(&self) -> &str {
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

#[derive(Serialize)]
pub enum FfdVersion {
    #[serde(rename = "1.2")]
    Ver1_2,
    #[serde(rename = "1.05")]
    Ver1_05,
}

#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct ClientInfo {
    #[serde(serialize_with = "serialize_date_simple")]
    birth_date: PrimitiveDateTime,
    // Числовой код страны, гражданином которой является клиент.
    // Код страны указывается в соответствии с
    // Общероссийским классификатором стран мира ОКСМ
    citizenship: CountryCode,
    document_code: DocumentCode,
    // Реквизиты документа, удостоверяющего личность (например: серия и номер паспорта)
    document_data: String,
    // Адрес клиента, грузополучателя
    #[garde(length(max = 256))]
    address: String,
}

// Система налогообложения
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Taxation {
    // общая СН
    Osn,
    // упрощенная СН (доходы)
    UsnIncome,
    // упрощенная СН (доходы минус расходы)
    UsnIncomeOutcome,
    // Единый налог на вмененный доход
    Envd,
    // Единый сельскохозяйственный налог
    Esn,
    // Патентная СН
    Patent,
}

pub enum EmailOrPhone {
    Email(Email),
    Phone(phonenumber::PhoneNumber),
}

// FIXME: check if that works correctly
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
                state.serialize_field("email", email)?;
                state.end()
            }
            EmailOrPhone::Phone(phone) => {
                let mut state =
                    serializer.serialize_struct("EmailOrPhone", 1)?;
                // FIXME: test how phone.to_string looks. Is it correct?
                let phone_number =
                    phone.format().mode(phonenumber::Mode::E164).to_string();
                if phone_number.len() > 64 {
                    return Err(S::Error::custom(
                        "Phone number length exceeds 64 characters",
                    ));
                }
                state.serialize_field("phone", &phone_number)?;
                state.end()
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payments {
    /// Вид оплаты "Наличные". Сумма к оплате в копейках
    #[serde(skip_serializing_if = "Option::is_none")]
    cash: Option<Kopeck>,
    /// Вид оплаты "Безналичный"
    electronic: Kopeck,
    /// Вид оплаты "Предварительная оплата (Аванс)"
    #[serde(skip_serializing_if = "Option::is_none")]
    advance_payment: Option<Kopeck>,
    /// Вид оплаты "Постоплата (Кредит)"
    #[serde(skip_serializing_if = "Option::is_none")]
    credit: Option<Kopeck>,
    /// Вид оплаты "Иная форма оплаты"
    #[serde(skip_serializing_if = "Option::is_none")]
    provision: Option<Kopeck>,
}

#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct Receipt {
    ffd_version: FfdVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(dive)]
    client_info: Option<ClientInfo>,
    taxation: Taxation,
    #[serde(flatten)]
    email_or_phone: EmailOrPhone,
    /// Идентификатор/Имя клиента
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_inn: Option<String>,
    // Массив, содержащий в себе информацию о товарах.
    items: Vec<Item>,
    /// Детали платежа. Если объект не передан, будет автоматически
    /// указана итоговая сумма чека с видом оплаты "Безналичный".
    /// Если передан объект receipt.Payments, то значение в
    /// Electronic должно быть равно итоговому значению Amount в методе Init.
    /// При этом сумма введенных значений по всем видам оплат,
    /// включая Electronic, должна быть равна сумме (Amount)
    /// всех товаров, переданных в объекте receipt.Items.
    #[serde(skip_serializing_if = "Option::is_none")]
    payments: Option<Payments>,
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
