use std::num::ParseIntError;

use garde::Validate;
use phonenumber::PhoneNumber;
use rust_decimal::Decimal;
use serde::{ser::Error, ser::SerializeStruct, Serialize, Serializer};
use time::{macros::format_description, PrimitiveDateTime};

use crate::domain::{country_code::CountryCode, email::Email, kopeck::Kopeck};

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

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Receipt {
    ffd_version: FfdVersion,
    #[serde(skip_serializing_if = "Option::is_none")]
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

// ───── Item ─────────────────────────────────────────────────────────────── //

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentSign {
    BankPayingAgent,
    BankPayingSubagent,
    PayingAgent,
    PayingSubagent,
    Attorney,
    CommissionAgent,
    Another,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AgentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_sign: Option<AgentSign>,
    /// Наименование операции.
    /// Атрибут ОБЯЗАТЕЛЕН, если AgentSign передан в значениях
    /// * `bank_paying_agent`
    /// * `bank_paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_name: Option<String>, // <= 64 characters
    /// Атрибут ОБЯЗАТЕЛЕН, если в AgentSign передан в значениях:
    /// * `bank_paying_agent`
    /// * `bank_paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_name: Option<String>, // <= 64 characters
    /// Атрибут ОБЯЗАТЕЛЕН, если в AgentSign передан в значениях:
    /// * `bank_paying_agent`
    /// * `bank_paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_address: Option<String>, // <= 243 characters
    ///  Атрибут обязателен, если в AgentSign передан в значениях:
    /// * `bank_paying_agent`
    /// * `bank_paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_inn: Option<String>, // <= 12 characters

    // FIXME: check that phonenumber has correct form here:
    /// Телефоны в формате `+{Ц}`
    /// Атрибут ОБЯЗАТЕЛЕН, если в AgentSign передан в значениях:
    /// * `bank_paying_agent`
    /// * `bank_paying_subagent`
    /// * `paying_agent`
    /// * `paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<Vec<PhoneNumber>>,
    /// Телефоны в формате `+{Ц}`
    /// Атрибут ОБЯЗАТЕЛЕН, если в AgentSign передан в значениях:
    /// * `paying_agent`
    /// * `paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_phones: Option<Vec<PhoneNumber>>,
    /// Телефоны в формате `+{Ц}`
    /// Атрибут ОБЯЗАТЕЛЕН, если в AgentSign передан в значениях:
    /// * `bank_paying_agent`
    /// * `bank_paying_subagent`
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_phones: Option<Vec<PhoneNumber>>,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SupplierInfo {
    /// Телефон поставщика, в формате `+{Ц}`
    /// Атрибут ОБЯЗАТЕЛЕН, если передается значение AgentSign в объекте AgentData
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<Vec<PhoneNumber>>,
    /// Наименование поставщика. Атрибут обязателен, если передается
    /// значение AgentSign в объекте AgentData.
    /// Внимание: в данные 239 символов включаются телефоны поставщика:
    /// 4 символа на каждый телефон.
    /// Например, если передано два телефона поставщика длиной 12 и 14 символов,
    /// то максимальная длина наименования поставщика будет
    /// 239 – (12 + 4) – (14 + 4) = 205 символов
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>, // <= 239 characters
    /// ИНН поставщика, в формате ЦЦЦЦЦЦЦЦЦЦ.
    /// Атрибут обязателен, если передается значение AgentSign в объекте AgentData.
    #[serde(skip_serializing_if = "Option::is_none")]
    inn: Option<String>, // [ 10 .. 12 ] characters
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VatType {
    None,   // без НДС
    Vat0,   // НДС по ставке 0%
    Vat10,  // НДС по ставке 10%
    Vat20,  // НДС по ставке 20%
    Vat110, // НДС чека по расчетной ставке 10/110
    Vat120, // НДС чека по расчетной ставке 20/120
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    FullPrepayment, // предоплата 100%
    Prepayment,     // предоплата
    Advance,        // аванс
    FullPayment,    // полный расчет
    PartialPayment, // частичный расчет и кредит
    Credit,         // передача в кредит
    CreditPayment,  // оплата кредита
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentObject {
    Commodity,                         // товар
    Excise,                            // подакцизный товар
    Job,                               // работа
    Service,                           // услуга
    GamblingBet,                       // ставка азартной игры
    GamblingPrize,                     // выигрыш азартной игры
    Lottery,                           // лотерейный билет
    LotteryPrize,                      // выигрыш лотереи
    IntellectualActivity, // предоставление результатов интеллектуальной деятельности
    Payment,              // платеж
    AgentCommission,      // агентское вознаграждение
    Contribution,         // Выплата
    PropertyRights,       // Имущественное право
    Unrealization,        // Внереализационный доход
    TaxReduction,         // Иные платежи и взносы
    TradeFee,             // Торговый сбор
    ResortTax,            // Курортный сбор
    Pledge,               // Залог
    IncomeDecrease,       // Расход
    IePensionInsuranceWithoutPayments, // Взносы на ОПС ИП
    IePensionInsuranceWithPayments, // Взносы на ОПС
    IeMedicalInsuranceWithoutPayments, // Взносы на ОМС ИП
    IeMedicalInsuranceWithPayments, // Взносы на ОМС
    SocialInsurance,      // Взносы на ОСС
    CasinoChips,          // Платеж казино
    AgentPayment,         // Выдача ДС
    ExcisableGoodsWithoutMarkingCode, // АТНМ
    ExcisableGoodsWithMarkingCode, // АТМ
    GoodsWithoutMarkingCode, // ТНМ
    GoodsWithMarkingCode, // ТМ
    Another,              // иной предмет расчета
}

#[derive(Serialize)]
enum MeasurementUnit {
    #[serde(rename = "шт")]
    Piece,
    #[serde(rename = "г")]
    Gram,
    #[serde(rename = "кг")]
    Kilogram,
    #[serde(rename = "т")]
    Ton,
    #[serde(rename = "см")]
    Centimeter,
    #[serde(rename = "дм")]
    Decimeter,
    #[serde(rename = "м")]
    Meter,
    #[serde(rename = "кв. см")]
    SquareCentimeter,
    #[serde(rename = "кв. дм")]
    SquareDecimeter,
    #[serde(rename = "кв. м")]
    SquareMeter,
    #[serde(rename = "мл")]
    Milliliter,
    #[serde(rename = "л")]
    Liter,
    #[serde(rename = "куб. м")]
    CubicMeter,
    #[serde(rename = "кВт · ч")]
    KilowattHour,
    #[serde(rename = "Гкал")]
    Gigacalorie,
    #[serde(rename = "сутки")]
    Day,
    #[serde(rename = "час")]
    Hour,
    #[serde(rename = "мин")]
    Minute,
    #[serde(rename = "с")]
    Second,
    #[serde(rename = "Кбайт")]
    Kilobyte,
    #[serde(rename = "Мбайт")]
    Megabyte,
    #[serde(rename = "Гбайт")]
    Gigabyte,
    #[serde(rename = "Тбайт")]
    Terabyte,
    Other(u8),
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarkCodeType {
    /// Код товара, формат которого не идентифицирован,
    /// как один из реквизитов
    Unknown,
    /// Код товара в формате EAN-8
    Ean8,
    /// Код товара в формате EAN-13
    Ean13,
    /// Код товара в формате ITF-14
    Itf14,
    /// Код товара в формате GS1, нанесенный на товар,
    /// не подлежащий маркировке
    Gs10,
    /// Код товара в формате GS1, нанесенный на товар,
    /// подлежащий маркировке
    Gs1m,
    /// Код товара в формате короткого кода маркировки,
    /// нанесенный на товар
    Short,
    /// Контрольно-идентификационный знак мехового изделия
    Fur,
    /// Код товара в формате ЕГАИС-2.0
    Egais20,
    /// Код товара в формате ЕГАИС-3.0
    Egais30,
    /// Код маркировки, как он был прочитан сканером
    Rawcode,
}

/// Код маркировки в машиночитаемой форме, представленный в виде
/// одного из видов кодов, формируемых в соответствии с требованиями,
/// предусмотренными правилами, для нанесения на потребительскую упаковку,
/// или на товары, или на товарный ярлык
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkCode {
    /// Тип штрих кода
    mark_code_type: MarkCodeType,
    /// Код маркировки
    value: String,
}

/// Отраслевой реквизит предмета расчета.
///
/// Необходимо указывать только для товаров подлежащих обязательной
/// маркировке средством идентификации и включение данного реквизита
/// предусмотрено НПА отраслевого регулирования для соответствующей
/// товарной группы.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectoralItemProps {
    federal_id: String,
    date: PrimitiveDateTime,
    number: String,
    value: String,
}

// Атрибуты, предусмотренные в протоколе для отправки чеков
// по маркируемым товарам, не являются обязательными для товаров
// без маркировки. Если используется ФФД 1.2, но реализуемый товар -
// не подлежит маркировке, то поля можно не отправлять или отправить со значением null.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    /// Данные агента. Обязателен, если используется агентская схема.
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_data: Option<AgentData>,
    /// Данные поставщика платежного агента.
    // Обязателен, если передается значение AgentSign в объекте AgentData.
    #[serde(skip_serializing_if = "Option::is_none")]
    supplier_info: Option<SupplierInfo>,
    name: String, // <= 128 characters
    /// Цена в копейках
    price: Decimal,
    /// Максимальное количество символов - 8, где целая часть не более 5 знаков,
    /// а дробная часть не более 3 знаков для `Атол`,
    /// не более 2 знаков для `CloudPayments`
    /// Значение «1», если передан объект `MarkCode`
    quantity: Decimal, // <= 8 characters
    /// Стоимость товара в копейках. Произведение Quantity и Price
    amount: Decimal, // <= 10 characters
    tax: VatType,
    payment_method: PaymentMethod,
    payment_object: PaymentObject,
    /// Дополнительный реквизит предмета расчета.
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<String>,
    /// Сумма акциза в рублях с учетом копеек, включенная в стоимость предмета расчета.
    /// Целая часть не более 8 знаков;
    /// дробная часть не более 2 знаков;
    /// значение не может быть отрицательным.
    #[serde(skip_serializing_if = "Option::is_none")]
    excise: Option<Decimal>,
    /// Цифровой код страны происхождения товара в соответствии с
    /// Общероссийским классификатором стран мира (3 цифры)
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<CountryCode>,
    /// Номер таможенной декларации
    #[serde(skip_serializing_if = "Option::is_none")]
    declaration_number: Option<String>, // <= 32 characters
    measurement_unit: MeasurementUnit,
    /// Режим обработки кода маркировки. Должен принимать значение равное «0».
    /// Включается в чек в случае, если предметом расчета является товар,
    /// подлежащий обязательной маркировке средством идентификации
    /// (соответствующий код в поле paymentObject)
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_processing_mode: Option<char>,
    /// Включается в чек в случае, если предметом расчета является товар,
    /// подлежащий обязательной маркировке средством идентификации
    /// (соответствующий код в поле paymentObject)
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_code: Option<MarkCode>,
    // TODO: Не является обязательным объектом, implement later.
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_quantity: Option<()>,
    /// Отраслевой реквизит предмета расчета
    #[serde(skip_serializing_if = "Option::is_none")]
    sectoral_item_props: Option<SectoralItemProps>,
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
