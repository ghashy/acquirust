use garde::Validate;
use phonenumber::PhoneNumber;
use rust_decimal::Decimal;
use serde::{ser::Error, Serialize, Serializer};
use time::PrimitiveDateTime;

use crate::{domain::country_code::CountryCode, error_chain_fmt, Kopeck};

// ───── AgentData ────────────────────────────────────────────────────────── //

pub struct AgentDetails {
    operation_name: String,
    operator_name: String,
    operator_address: String,
    operator_inn: String,
    phones: Vec<PhoneNumber>,
    transfer_phones: Vec<PhoneNumber>,
}

/// Признак агента
pub enum AgentSignParams {
    BankPayingAgent(AgentDetails),
    BankPayingSubagent(AgentDetails),
    PayingAgent {
        phones: Vec<PhoneNumber>,
        receiver_phones: Vec<PhoneNumber>,
    },
    PayingSubagent {
        phones: Vec<PhoneNumber>,
        receiver_phones: Vec<PhoneNumber>,
    },
    Attorney,
    CommissionAgent,
    Another,
}

impl std::fmt::Display for AgentSignParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AgentSignParams::BankPayingAgent(_) => "bank_paying_agent",
            AgentSignParams::BankPayingSubagent(_) => "bank_paying_subagent",
            AgentSignParams::PayingAgent { .. } => "paying_agent",
            AgentSignParams::PayingSubagent { .. } => "paying_subagent",
            AgentSignParams::Attorney => "attorney",
            AgentSignParams::CommissionAgent => "commission_agent",
            AgentSignParams::Another => "another",
        };
        f.write_str(s)
    }
}

// TODO: check that phonenumber has correct form here
// Телефоны в формате `+{Ц}`
/// Данные агента.
#[derive(Serialize, Validate, Default)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct AgentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_sign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 64))]
    operation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 64))]
    operator_name: Option<String>,
    #[garde(length(max = 243))]
    #[serde(skip_serializing_if = "Option::is_none")]
    operator_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 12))]
    operator_inn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_phones: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_phones: Option<Vec<PhoneNumber>>,
}

impl AgentData {
    pub fn builder(sign: AgentSignParams) -> AgentDataBuilder {
        let agent_sign = sign.to_string();
        match sign {
            AgentSignParams::BankPayingAgent(details)
            | AgentSignParams::BankPayingSubagent(details) => {
                AgentDataBuilder {
                    agent_sign: Some(agent_sign),
                    operation_name: Some(details.operation_name),
                    operator_name: Some(details.operator_name),
                    operator_address: Some(details.operator_address),
                    operator_inn: Some(details.operator_inn),
                    phones: Some(details.phones),
                    receiver_phones: None,
                    transfer_phones: Some(details.transfer_phones),
                }
            }
            AgentSignParams::PayingAgent {
                phones,
                receiver_phones,
            }
            | AgentSignParams::PayingSubagent {
                phones,
                receiver_phones,
            } => AgentDataBuilder {
                agent_sign: Some(agent_sign),
                phones: Some(phones),
                receiver_phones: Some(receiver_phones),
                ..Default::default()
            },
            AgentSignParams::Attorney
            | AgentSignParams::CommissionAgent
            | AgentSignParams::Another => Default::default(),
        }
    }
}

#[derive(Default)]
pub struct AgentDataBuilder {
    agent_sign: Option<String>,
    operation_name: Option<String>,
    operator_name: Option<String>,
    operator_address: Option<String>,
    operator_inn: Option<String>,
    phones: Option<Vec<PhoneNumber>>,
    receiver_phones: Option<Vec<PhoneNumber>>,
    transfer_phones: Option<Vec<PhoneNumber>>,
}

impl AgentDataBuilder {
    pub fn with_operation_name(mut self, name: String) -> Self {
        self.operation_name = Some(name);
        self
    }
    pub fn with_operator_name(mut self, name: String) -> Self {
        self.operator_name = Some(name);
        self
    }
    pub fn with_operator_address(mut self, address: String) -> Self {
        self.operator_address = Some(address);
        self
    }
    pub fn with_operator_inn(mut self, inn: String) -> Self {
        self.operator_inn = Some(inn);
        self
    }
    pub fn with_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.phones = Some(phones);
        self
    }
    pub fn with_receiver_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.receiver_phones = Some(phones);
        self
    }
    pub fn with_transfer_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.transfer_phones = Some(phones);
        self
    }
    pub fn build(self) -> Result<AgentData, garde::Report> {
        let data = AgentData {
            agent_sign: self.agent_sign,
            operation_name: self.operation_name,
            operator_name: self.operator_name,
            operator_address: self.operator_address,
            operator_inn: self.operator_inn,
            phones: self.phones,
            receiver_phones: self.receiver_phones,
            transfer_phones: self.transfer_phones,
        };
        data.validate(&())?;
        Ok(data)
    }
}

// ───── SupplierInfo ─────────────────────────────────────────────────────── //

#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct SupplierInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<Vec<PhoneNumber>>,
    /// Наименование поставщика.
    /// Внимание: в данные 239 символов включаются телефоны поставщика:
    /// 4 символа на каждый телефон.
    /// Например, если передано два телефона поставщика длиной 12 и 14 символов,
    /// то максимальная длина наименования поставщика будет
    /// 239 – (12 + 4) – (14 + 4) = 205 символов
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 239))]
    name: Option<String>,
    /// ИНН поставщика.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(min = 10, max = 12))]
    inn: Option<String>,
}

impl SupplierInfo {
    /// Все атрибуты ОБЯЗАТЕЛЕНЫ, если передается значение AgentSign в объекте AgentData
    pub fn new(
        phones: Option<Vec<PhoneNumber>>,
        name: Option<String>,
        inn: Option<String>,
    ) -> Result<Self, garde::Report> {
        let supplier_info = SupplierInfo { phones, name, inn };
        supplier_info.validate(&())?;
        Ok(supplier_info)
    }
}

// ───── Item ─────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum ItemParseError {
    #[error("SupplierInfo is not represented, but should")]
    SupplierInfoError,
    #[error("Validation error")]
    ValidationError(#[from] garde::Report),
}

impl std::fmt::Debug for ItemParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
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
pub enum MeasurementUnit {
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
    pub mark_code_type: MarkCodeType,
    /// Код маркировки
    pub value: String,
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
    /// Идентификатор ФОИВ (федеральный орган исполнительной власти)
    pub federal_id: String,
    /// Дата нормативного акта ФОИВ
    #[serde(serialize_with = "serialize_date_rfc3339")]
    pub date: PrimitiveDateTime,
    /// Номер нормативного акта ФОИВ
    pub number: String,
    /// Состав значений, определенных нормативным актом ФОИВ
    pub value: String,
}

// Атрибуты, предусмотренные в протоколе для отправки чеков
// по маркируемым товарам, не являются обязательными для товаров
// без маркировки. Если используется ФФД 1.2, но реализуемый товар -
// не подлежит маркировке, то поля можно не отправлять или отправить со значением null.
#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct Item {
    /// Данные агента. Обязателен, если используется агентская схема.
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_data: Option<AgentData>,
    /// Данные поставщика платежного агента.
    // Обязателен, если передается значение AgentSign в объекте AgentData.
    #[serde(skip_serializing_if = "Option::is_none")]
    supplier_info: Option<SupplierInfo>,
    #[garde(length(max = 128))]
    name: String,
    /// Цена в копейках
    price: Kopeck,
    /// Максимальное количество символов - 8, где целая часть не более 5 знаков,
    /// а дробная часть не более 3 знаков для `Атол`,
    /// не более 2 знаков для `CloudPayments`
    /// Значение «1», если передан объект `MarkCode`
    quantity: Decimal,
    /// Стоимость товара в копейках. Произведение Quantity и Price
    amount: Kopeck,
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
    #[garde(custom(check_excise))]
    excise: Option<Decimal>,
    /// Цифровой код страны происхождения товара в соответствии с
    /// Общероссийским классификатором стран мира (3 цифры)
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<CountryCode>,
    /// Номер таможенной декларации
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 32))]
    declaration_number: Option<String>,
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

impl Item {
    pub fn builder(
        name: String,
        price: Kopeck,
        quantity: Decimal,
        amount: Kopeck,
        tax: VatType,
        payment_object: PaymentObject,
        measurement_unit: MeasurementUnit,
    ) -> ItemBuilder {
        ItemBuilder {
            agent_data: None,
            supplier_info: None,
            name,
            price,
            quantity,
            amount,
            tax,
            payment_method: PaymentMethod::FullPrepayment,
            payment_object,
            user_data: None,
            excise: None,
            country_code: None,
            declaration_number: None,
            measurement_unit,
            mark_processing_mode: None,
            mark_code: None,
            mark_quantity: None,
            sectoral_item_props: None,
        }
    }
}

pub struct ItemBuilder {
    agent_data: Option<AgentData>,
    supplier_info: Option<SupplierInfo>,
    name: String,
    price: Kopeck,
    quantity: Decimal,
    amount: Kopeck,
    tax: VatType,
    payment_method: PaymentMethod,
    payment_object: PaymentObject,
    user_data: Option<String>,
    excise: Option<Decimal>,
    country_code: Option<CountryCode>,
    declaration_number: Option<String>,
    measurement_unit: MeasurementUnit,
    mark_processing_mode: Option<char>,
    mark_code: Option<MarkCode>,
    mark_quantity: Option<()>,
    sectoral_item_props: Option<SectoralItemProps>,
}

impl ItemBuilder {
    /// Данные агента.
    ///
    /// Если передается значение AgentSign в объекте AgentData,
    /// SupplierInfo должен быть полностью инициализирован,
    /// иначе `build` вернет ошибку.
    pub fn with_agent_data(mut self, agent_data: AgentData) -> Self {
        self.agent_data = Some(agent_data);
        self
    }
    /// Данные поставщика платежного агента.
    pub fn with_supplier_info(mut self, info: SupplierInfo) -> Self {
        self.supplier_info = Some(info);
        self
    }
    /// Признак способа расчёта.
    pub fn with_payment_method(mut self, method: PaymentMethod) -> Self {
        self.payment_method = method;
        self
    }
    /// Дополнительный реквизит предмета расчета.
    pub fn with_user_data(mut self, data: String) -> Self {
        self.user_data = Some(data);
        self
    }
    /// Сумма акциза в рублях с учетом копеек, включенная в стоимость предмета расчета.
    /// Целая часть не более 8 знаков;
    /// дробная часть не более 2 знаков;
    /// значение не может быть отрицательным.
    pub fn with_excise(mut self, excise: Decimal) -> Self {
        self.excise = Some(excise);
        self
    }
    /// Цифровой код страны происхождения товара в соответствии с
    /// Общероссийским классификатором стран мира.
    pub fn with_country_code(mut self, code: CountryCode) -> Self {
        self.country_code = Some(code);
        self
    }
    /// Номер таможенной декларации
    /// Max length is 32.
    pub fn with_declaration_number(mut self, code: String) -> Self {
        self.declaration_number = Some(code);
        self
    }
    /// Режим обработки кода маркировки.
    pub fn mark_processing_mode(mut self) -> Self {
        self.mark_processing_mode = Some('0');
        self
    }
    /// Включается в чек в случае, если предметом расчета является товар,
    /// подлежащий обязательной маркировке средством идентификации.
    pub fn with_mark_code(mut self, code: MarkCode) -> Self {
        self.mark_code = Some(code);
        self
    }
    /// Отраслевой реквизит предмета расчета
    pub fn with_sectoral_item_props(
        mut self,
        props: SectoralItemProps,
    ) -> Self {
        self.sectoral_item_props = Some(props);
        self
    }
    pub fn build(self) -> Result<Item, ItemParseError> {
        let item = Item {
            agent_data: self.agent_data,
            supplier_info: self.supplier_info,
            name: self.name,
            price: self.price,
            quantity: self.quantity,
            amount: self.amount,
            tax: self.tax,
            payment_method: self.payment_method,
            payment_object: self.payment_object,
            user_data: self.user_data,
            excise: self.excise,
            country_code: self.country_code,
            declaration_number: self.declaration_number,
            measurement_unit: self.measurement_unit,
            mark_processing_mode: self.mark_processing_mode,
            mark_code: self.mark_code,
            mark_quantity: self.mark_quantity,
            sectoral_item_props: self.sectoral_item_props,
        };
        item.validate(&())?;
        if item.agent_data.is_some() {
            // Check that supplier_info is fully initialized
            if let Some(ref s) = item.supplier_info {
                if s.phones.is_none() || s.name.is_none() || s.inn.is_none() {
                    return Err(ItemParseError::SupplierInfoError);
                }
            } else {
                return Err(ItemParseError::SupplierInfoError);
            }
        }
        Ok(item)
    }
}

// ───── Functions ────────────────────────────────────────────────────────── //

fn serialize_date_rfc3339<S>(
    date: &PrimitiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted_date = date
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(S::Error::custom)?;
    serializer.serialize_str(&formatted_date)
}

fn check_excise(excise: &Option<Decimal>, _: &()) -> Result<(), garde::Error> {
    if let Some(num) = excise {
        if num.is_sign_negative() {
            return Err(garde::Error::new("Number can't be negative"));
        }
        if num.trunc().to_string().len() > 8 {
            return Err(garde::Error::new("Number is too long"));
        }
        if num.fract().to_string().len() - 2 > 2 {
            return Err(garde::Error::new("Number fract part is too long"));
        }
    }
    Ok(())
}
