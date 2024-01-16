use garde::Validate;
use phonenumber::PhoneNumber;
use rust_decimal::Decimal;
use serde::{ser::Error, Serialize, Serializer};
use time::PrimitiveDateTime;

use crate::domain::CountryCode;
use crate::domain::Kopeck;
use crate::error_chain_fmt;

/// Данные агента, некоторые детали.
pub struct AgentDetails {
    /// Наименование операции.
    /// Максимальная длина: 64 символа.
    pub operation_name: String,
    /// Наименование оператора перевода.
    /// Максимальная длина: 64 символа.
    pub operator_name: String,
    /// Адрес оператора перевода.
    /// Максимальная длина: 243 символов.
    pub operator_address: String,
    /// ИНН оператора перевода.
    /// Максимальная длина: 12 символов.
    pub operator_inn: String,
    /// Телефоны платежного агента, в формате +{Ц}.
    /// Ограничения по длине: от 1 до 19 символов.
    pub phones: Vec<PhoneNumber>,
    /// Телефоны оператора перевода, в формате +{Ц}.
    /// Ограничения по длине: от 1 до 19 символов.
    pub transfer_phones: Vec<PhoneNumber>,
}

/// Параметры для инициализации AgentData.
pub enum AgentSignParams {
    BankPayingAgent(AgentDetails),
    BankPayingSubagent(AgentDetails),
    PayingAgent {
        /// Телефоны платежного агента, в формате +{Ц}.
        /// Ограничения по длине: от 1 до 19 символов.
        phones: Vec<PhoneNumber>,
        /// Телефоны оператора по приему платежей, в формате +{Ц}.
        /// Ограничения по длине: от 1 до 19 символов.
        receiver_phones: Vec<PhoneNumber>,
    },
    PayingSubagent {
        /// Телефоны платежного агента, в формате +{Ц}.
        /// Ограничения по длине: от 1 до 19 символов.
        phones: Vec<PhoneNumber>,
        /// Телефоны оператора по приему платежей, в формате +{Ц}.
        /// Ограничения по длине: от 1 до 19 символов.
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

/// Данные агента.
/// Для использования, если используется агентская схема.
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
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_phonenumber_vec"
    )]
    receiver_phones: Option<Vec<PhoneNumber>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_phonenumber_vec"
    )]
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
    pub(super) fn is_agent_sign_set(&self) -> bool {
        self.agent_sign.is_some()
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
    /// Задать наименование операции.
    /// Максимальная длина: 64 символа.
    pub fn with_operation_name(mut self, name: String) -> Self {
        self.operation_name = Some(name);
        self
    }
    /// Задать наименование оператора перевода.
    /// Максимальная длина: 64 символа.
    pub fn with_operator_name(mut self, name: String) -> Self {
        self.operator_name = Some(name);
        self
    }
    /// Задать адрес оператора перевода.
    /// Максимальная длина: 243 символов.
    pub fn with_operator_address(mut self, address: String) -> Self {
        self.operator_address = Some(address);
        self
    }
    /// Задать ИНН оператора перевода.
    /// Максимальная длина: 12 символов.
    pub fn with_operator_inn(mut self, inn: String) -> Self {
        self.operator_inn = Some(inn);
        self
    }
    /// Добавить список телефонов платежного агента, в формате +{Ц}.
    /// Ограничения по длине: от 1 до 19 символов.
    pub fn with_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.phones = Some(phones);
        self
    }
    /// Добавить список телефонов оператора по приему платежей, в формате +{Ц}.
    /// Ограничения по длине: от 1 до 19 символов.
    pub fn with_receiver_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.receiver_phones = Some(phones);
        self
    }
    /// Добавить список телефонов оператора перевода, в формате +{Ц}.
    /// Ограничения по длине: от 1 до 19 символов.
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

/// Данные поставщика платежного агента
#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct SupplierInfo {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "crate::serialize_phonenumber_vec"
    )]
    phones: Option<Vec<PhoneNumber>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 239))]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(min = 10, max = 12))]
    inn: Option<String>,
}

impl SupplierInfo {
    /// Создать новый `SupplierInfo` объект.
    ///
    /// Все параметры ОБЯЗАТЕЛЬНЫ, если передается значение AgentSign в объекте AgentData.
    ///
    /// - `phones`: Телефоны поставщика, в формате +{Ц}. Ограничения по длине: от 1 до 19 символов.
    ///
    /// - `name`: Наименование поставщика. Внимание: в данные 239 символов включаются
    /// телефоны поставщика: 4 символа на каждый телефон.
    /// Например, если передано два телефона поставщика длиной 12 и 14 символов,
    /// то максимальная длина наименования поставщика будет 239 – (12 + 4) – (14 + 4) = 205 символов
    ///
    /// - `inn`: ИНН поставщика, в формате ЦЦЦЦЦЦЦЦЦЦ. Атрибут обязателен, если передается
    /// значение AgentSign в объекте AgentData. Максимальная длина: 12 символов.
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
    #[error("Only one ffd item can be presented")]
    BothFfdVersionPresentedError,
    #[error("When MarkCode is set, quantity should be 1, but got {0}")]
    WrongQuantityValueError(Decimal),
    #[error("Bad quantity value: {0}")]
    BadQuantityValueError(String),
    #[error("No cashbox type set, and MarkCode is not set")]
    NoCashBoxSet,
}

impl std::fmt::Debug for ItemParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// Ставка НДС.
///
/// # Перечисление со значениями:
///
/// * none - без НДС;
/// * vat0 - НДС по ставке 0%
/// * vat10 - НДС по ставке 10%
/// * vat20 - НДС по ставке 20%
/// * vat110 - НДС чека по расчетной ставке 10/110
/// * vat120 - НДС чека по расчетной ставке 20/120
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VatType {
    None,
    Vat0,
    Vat10,
    Vat20,
    Vat110,
    Vat120,
}

/// Признак способа расчёта.
///
/// # Возможные значения:
///
/// * `full_prepayment` – предоплата 100%
/// * `prepayment` – предоплата
/// * `advance` – аванс
/// * `full_payment` – полный расчет
/// * `partial_payment` – частичный расчет и кредит
/// * `credit` – передача в кредит
/// * `credit_payment` – оплата кредита
/// Если значение не передано, по умолчанию в онлайн-кассу передается признак способа расчёта "full_payment".
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    FullPrepayment,
    Prepayment,
    Advance,
    FullPayment,
    PartialPayment,
    Credit,
    CreditPayment,
}

/// Значения реквизита "признак предмета расчета" (тег 1212) таблица 101
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentObjectFfd12 {
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

/// Признак предмета расчёта
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentObjectFfd105 {
    Commodity,
    Excise,
    Job,
    Service,
    GamblingBet,
    GamblingPrize,
    Lottery,
    LotteryPrize,
    IntellectualActivity,
    Payment,
    AgentCommission,
    Composite,
    Another,
}

/// Единицы измерения
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

/// Тип штрих кода.
///
/// # Возможные значения:
///
/// `Unknown` - код товара, формат которого не идентифицирован, как один из реквизитов
/// `Ean8` - код товара в формате EAN-8.
/// `Ean13` - код товара в формате EAN-13
/// `Itf14` - код товара в формате ITF-14
/// `Gs10` - код товара в формате GS1, нанесенный на товар, не подлежащий маркировке
/// `Gs1m` - код товара в формате GS1, нанесенный на товар, подлежащий маркировке
/// `Short` - код товара в формате короткого кода маркировки, нанесенный на товар,
/// `Fur` - контрольно-идентификационный знак мехового изделия.
/// `Egais20` - код товара в формате ЕГАИС-2.0.
/// `Egais30` - код товара в формате ЕГАИС-3.0.
/// `Rawcode` - Код маркировки, как он был прочитан сканером.
#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum MarkCodeType {
    Unknown,
    Ean8,
    Ean13,
    Itf14,
    Gs10,
    Gs1m,
    Short,
    Fur,
    Egais20,
    Egais30,
    Rawcode,
}

/// Код маркировки в машиночитаемой форме.
///
/// Представлен в виде одного из видов кодов,
/// формируемых в соответствии с требованиями, предусмотренными правилами,
/// для нанесения на потребительскую упаковку, или на товары, или на товарный ярлык.
///
/// Включается в чек в случае, если предметом расчета является товар,
/// подлежащий обязательной маркировке средством идентификации.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkCode {
    /// Тип штрих кода.
    pub mark_code_type: MarkCodeType,
    /// Код маркировки
    pub value: String,
}

/// Отраслевой реквизит предмета расчета.
///
/// Необходимо указывать только для товаров подлежащих обязательной маркировке
/// средством идентификации и включение данного реквизита предусмотрено НПА
/// отраслевого регулирования для соответствующей товарной группы.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectoralItemProps {
    /// Идентификатор ФОИВ (федеральный орган исполнительной власти).
    pub federal_id: String,
    /// Дата нормативного акта ФОИВ
    #[serde(serialize_with = "serialize_date_rfc3339")]
    pub date: PrimitiveDateTime,
    /// Номер нормативного акта ФОИВ
    pub number: String,
    /// Состав значений, определенных нормативным актом ФОИВ.
    pub value: String,
}

/// Фискальные данные транзакции согласно стандартам ФФД 1.2.
#[derive(Serialize, Validate)]
#[garde(allow_unvalidated)]
pub struct Ffd12Data {
    payment_object: PaymentObjectFfd12,
    payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(custom(check_excise))]
    excise: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_code: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 32))]
    declaration_number: Option<String>,
    measurement_unit: MeasurementUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_processing_mode: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_code: Option<MarkCode>,

    // TODO: not mandatory, implement later.
    #[serde(skip_serializing_if = "Option::is_none")]
    mark_quantity: Option<()>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sectoral_item_props: Option<SectoralItemProps>,
}

impl Ffd12Data {
    /// Создает билдер для конструирования экземпляра `Ffd12Data`.
    /// Эта функция инициализирует `Ffd12DataBuilder` обязательными полями, которые необходимы
    /// для любого фискального документа согласно стандартам ФФД 1.2.
    ///
    /// Аргументы:
    /// * `payment_object` - `PaymentObjectFFD_12`, который категоризирует тип транзакции.
    /// * `payment_method` - `PaymentMethod`, который указывает метод, с помощью которого
    /// происходит оплата.
    /// * `measurement_unit` - `MeasurementUnit`, который определяет единицу измерения товаров
    /// в транзакции.
    ///
    /// Возвращает:
    /// Экземпляр `Ffd12DataBuilder` с установленными обязательными полями и неустановленными необязательными полями,
    /// готовый к дальнейшему конструированию.
    pub fn builder(
        payment_object: PaymentObjectFfd12,
        payment_method: PaymentMethod,
        measurement_unit: MeasurementUnit,
    ) -> Ffd12DataBuilder {
        Ffd12DataBuilder {
            payment_object,
            payment_method,
            measurement_unit,
            user_data: None,
            excise: None,
            country_code: None,
            declaration_number: None,
            mark_processing_mode: None,
            mark_code: None,
            sectoral_item_props: None,
        }
    }
}

pub struct Ffd12DataBuilder {
    payment_object: PaymentObjectFfd12,
    payment_method: PaymentMethod,
    measurement_unit: MeasurementUnit,
    user_data: Option<String>,
    excise: Option<Decimal>,
    country_code: Option<CountryCode>,
    declaration_number: Option<String>,
    mark_processing_mode: Option<char>,
    mark_code: Option<MarkCode>,
    sectoral_item_props: Option<SectoralItemProps>,
}

impl Ffd12DataBuilder {
    /// Дополнительное поле пользовательских данных по платежному объекту.
    pub fn with_user_data(mut self, data: String) -> Self {
        self.user_data = Some(data);
        self
    }
    /// Сумма акциза в рублях, включая копейки, включенная в стоимость платежного объекта.
    /// Целая часть не более 8 цифр;
    /// Дробная часть не более 2 цифр;
    /// Значение не может быть отрицательным.
    pub fn with_excise(mut self, excise: Decimal) -> Self {
        self.excise = Some(excise);
        self
    }
    /// Цифровой код страны происхождения товара в соответствии
    /// с Всероссийским классификатором стран мира.
    pub fn with_country_code(mut self, code: CountryCode) -> Self {
        self.country_code = Some(code);
        self
    }
    /// Номер таможенной декларации
    /// Максимальная длина - 32 символа.
    pub fn with_declaration_number(mut self, code: String) -> Self {
        self.declaration_number = Some(code);
        self
    }
    /// Режим обработки кода маркировки.
    pub fn mark_processing_mode(mut self) -> Self {
        self.mark_processing_mode = Some('0');
        self
    }
    /// Включается в чек, если платежный объект является товаром, подлежащим обязательной маркировке средствами идентификации.
    pub fn with_mark_code(mut self, code: MarkCode) -> Self {
        self.mark_code = Some(code);
        self
    }
    /// Отраслевое требование к платежному объекту.
    pub fn with_sectoral_item_props(
        mut self,
        props: SectoralItemProps,
    ) -> Self {
        self.sectoral_item_props = Some(props);
        self
    }
    /// Строит объект Ffd12Data.
    /// Возвращает Ffd12Data или ошибку.
    pub fn build(self) -> Result<Ffd12Data, garde::Report> {
        let data = Ffd12Data {
            payment_object: self.payment_object,
            payment_method: self.payment_method,
            user_data: self.user_data,
            excise: self.excise,
            country_code: self.country_code,
            declaration_number: self.declaration_number,
            measurement_unit: self.measurement_unit,
            mark_processing_mode: self.mark_processing_mode,
            mark_code: self.mark_code,
            mark_quantity: None,
            sectoral_item_props: self.sectoral_item_props,
        };
        data.validate(&())?;
        Ok(data)
    }
}

/// Фискальные данные транзакции согласно стандартам ФФД 1.05.
#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct Ffd105Data {
    #[serde(skip_serializing_if = "Option::is_none", rename = "Ean13")]
    #[garde(length(max = 300))]
    ean_13: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shop_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_object: Option<PaymentObjectFfd105>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_method: Option<PaymentMethod>,
}

impl Ffd105Data {
    pub fn builder() -> Ffd105DataBuilder {
        Default::default()
    }
}

#[derive(Default)]
pub struct Ffd105DataBuilder {
    ean_13: Option<String>,
    shop_code: Option<String>,
    payment_object: Option<PaymentObjectFfd105>,
    payment_method: Option<PaymentMethod>,
}

impl Ffd105DataBuilder {
    /// Указание метода платежа.
    pub fn with_payment_method(mut self, method: PaymentMethod) -> Self {
        self.payment_method = Some(method);
        self
    }
    /// Штрих-код в требуемом формате. Требования могут варьироваться в зависимости от типа кассового аппарата:
    ///
    /// `ATOL Online` - шестнадцатеричное представление с пробелами.
    /// Максимальная длина – 32 байта.
    /// (^[a-fA-F0-9]{2}$)|(^([a-fA-F0-9]{2}\s){1,31}[a-fA-F0-9]{2}$)
    /// Пример:
    /// 00 00 00 01 00 21 FA 41 00 23 05 41 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 12 00 AB 00
    /// `CloudKassir` - длина строки: четная, от 8 до 150 байтов, то есть
    /// от 16 до 300 ASCII символов ['0' - '9', 'A' - 'F']
    /// шестнадцатеричное представление кода маркировки продукта.
    /// Пример: 303130323930303030630333435
    /// `OrangeData` - строка, содержащая массив в кодировке base64
    /// от 8 до 32 байтов. Пример: igQVAAADMTIzNDU2Nzg5MDEyMwAAAAAAAQ==
    /// В случае, если параметр Ean13, переданный в запросе, не прошел валидацию,
    /// возвращается неуспешный ответ с сообщением об ошибке в параметре
    /// message = "Invalid Ean13 parameter".
    pub fn with_ean_13(mut self, ean: &str) -> Self {
        self.ean_13 = Some(ean.to_string());
        self
    }
    /// Код магазина. Для параметра ShopСode следует использовать значение параметра
    /// `Submerchant_ID`, полученное в ответ на регистрацию магазинов через xml.
    /// Если xml не используется, поле передавать не нужно.
    pub fn with_shop_code(mut self, code: &str) -> Self {
        self.shop_code = Some(code.to_string());
        self
    }
    /// Указание платежного объекта.
    pub fn with_payment_object(mut self, obj: PaymentObjectFfd105) -> Self {
        self.payment_object = Some(obj);
        self
    }
    /// Строит объект Ffd105Data.
    pub fn build(self) -> Result<Ffd105Data, garde::Report> {
        let data = Ffd105Data {
            ean_13: self.ean_13,
            shop_code: self.shop_code,
            payment_object: self.payment_object,
            payment_method: self.payment_method,
        };
        data.validate(&())?;
        Ok(data)
    }
}

pub enum CashBoxType {
    Atol,
    CloudPayments,
}

/// Позиция в чеке с информацией о товаре
///
/// Атрибуты, указанные в протоколе отправки чеков
/// для маркированных товаров, не являются обязательными для товаров
/// без маркировки. Если используется ФФД 1.2, но продаваемый товар
/// не подлежит маркировке, то поля могут не отправляться или отправляться со значением null.
#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_data: Option<AgentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(dive)]
    supplier_info: Option<SupplierInfo>,
    #[garde(length(max = 128))]
    name: String,
    price: Kopeck,
    quantity: Decimal,
    amount: Kopeck,
    tax: VatType,

    #[serde(flatten)]
    pub(super) ffd_105_data: Option<Ffd105Data>,
    #[serde(flatten)]
    pub(super) ffd_12_data: Option<Ffd12Data>,
}

impl Item {
    /// Создает новый `ItemBuilder` с указанными свойствами.
    ///
    /// # Аргументы
    ///
    /// * `name` - Название товара.
    /// * `price` - Цена товара в копейках.
    /// * `quantity` - Количество или вес товара. Это значение
    ///   должно быть отформатировано с максимальным количеством 8 символов, включая до 5 цифр
    ///   для целой части и до 3 знаков после запятой для систем Atol или 2 для систем CloudPayments.
    ///   Установите значение '1', если передается объект MarkCode.
    /// * `amount` - Общая сумма товара в копейках, используя тип `Kopeck`.
    /// * `tax` - Тип НДС, который будет применен к товару, используя перечисление `VatType`.
    /// * `cashbox_type` - Тип кассы, которая будет использоваться, `Atol` или `CloudPayments`.
    ///
    /// # Примеры
    ///
    /// ```
    /// use rust_decimal::Decimal;
    /// use mapi::domain::Kopeck;
    /// use mapi::receipt::item::{VatType, Item, CashBoxType};
    ///
    /// let item_builder = Item::builder(
    ///     "Шоколадный батончик",
    ///     Kopeck::from_rub("50".parse().unwrap()).unwrap(),
    ///     Decimal::new(1, 0),
    ///     Kopeck::from_rub("50".parse().unwrap()).unwrap(),
    ///     VatType::Vat20,
    ///     Some(CashBoxType::Atol),
    /// );
    /// ```
    ///
    /// # Возвращаемое значение
    ///
    /// Возвращает экземпляр `ItemBuilder`, который может быть использован для создания товара с дополнительными
    /// необязательными свойствами.
    pub fn builder(
        name: &str,
        price: Kopeck,
        quantity: Decimal,
        amount: Kopeck,
        tax: VatType,
        cashbox_type: Option<CashBoxType>,
    ) -> ItemBuilder {
        ItemBuilder {
            agent_data: None,
            supplier_info: None,
            name: name.to_string(),
            price,
            quantity,
            amount,
            tax,
            ffd_105_data: None,
            ffd_12_data: None,
            cashbox_type,
        }
    }
}

pub struct ItemBuilder {
    cashbox_type: Option<CashBoxType>,
    agent_data: Option<AgentData>,
    supplier_info: Option<SupplierInfo>,
    name: String,
    price: Kopeck,
    quantity: Decimal,
    amount: Kopeck,
    tax: VatType,
    ffd_105_data: Option<Ffd105Data>,
    ffd_12_data: Option<Ffd12Data>,
}

impl ItemBuilder {
    /// Данные агента.
    ///
    /// Если в объекте AgentData передается значение AgentSign,
    /// SupplierInfo должен быть полностью инициализирован,
    /// в противном случае метод `build` вернет ошибку.
    pub fn with_agent_data(mut self, agent_data: AgentData) -> Self {
        self.agent_data = Some(agent_data);
        self
    }
    /// Данные поставщика платежного агента.
    /// Обязательны, если в объекте AgentData передается значение AgentSign.
    pub fn with_supplier_info(mut self, info: SupplierInfo) -> Self {
        self.supplier_info = Some(info);
        self
    }
    /// Фискальные данные транзакции согласно стандартам ФФД 1.05.
    pub fn with_ffd_105_data(mut self, data: Ffd105Data) -> Self {
        self.ffd_105_data = Some(data);
        self
    }
    /// Фискальные данные транзакции согласно стандартам ФФД 1.2.
    pub fn with_ffd_12_data(mut self, data: Ffd12Data) -> Self {
        self.ffd_12_data = Some(data);
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
            ffd_105_data: self.ffd_105_data,
            ffd_12_data: self.ffd_12_data,
        };
        item.validate(&())?;

        // Check that if mark_code set, quantity should be 1
        if let Some(ref data) = item.ffd_12_data {
            if data.mark_code.is_some()
                && !item.quantity.eq(&Decimal::new(1, 0))
            {
                return Err(ItemParseError::WrongQuantityValueError(
                    item.quantity,
                ));
            }
        } else {
            // Check general bounds for quantity
            if self.quantity.to_string().len() > 8
                || self.quantity.trunc().to_string().len() > 5
            {
                return Err(ItemParseError::BadQuantityValueError(
                    "Is out of range".to_string(),
                ));
            }
            // Check bounds for specific cashbox
            let (max_scale, cashbox_name) = match self.cashbox_type {
                Some(CashBoxType::Atol) => (3, "Atol"),
                Some(CashBoxType::CloudPayments) => (2, "CloudPayments"),
                None => return Err(ItemParseError::NoCashBoxSet),
            };
            if self.quantity.scale() > max_scale {
                return Err(ItemParseError::BadQuantityValueError(format!(
                    "Max scale is {} for {}",
                    max_scale, cashbox_name
                )));
            }
        }
        // Check if both ffd versions are set
        if item.ffd_105_data.is_some() && item.ffd_12_data.is_some() {
            return Err(ItemParseError::BothFfdVersionPresentedError);
        }
        // Check that supplier_info is fully initialized, if agent_sign is set
        if let Some(ref data) = item.agent_data {
            if data.is_agent_sign_set() {
                if let Some(ref s) = item.supplier_info {
                    if s.phones.is_none() || s.name.is_none() || s.inn.is_none()
                    {
                        return Err(ItemParseError::SupplierInfoError);
                    }
                } else {
                    return Err(ItemParseError::SupplierInfoError);
                }
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
