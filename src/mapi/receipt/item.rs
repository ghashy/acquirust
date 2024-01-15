use garde::Validate;
use phonenumber::PhoneNumber;
use rust_decimal::Decimal;
use serde::ser::SerializeSeq;
use serde::{ser::Error, Serialize, Serializer};
use time::PrimitiveDateTime;

use crate::domain::CountryCode;
use crate::domain::Kopeck;
use crate::error_chain_fmt;

// ───── AgentData ────────────────────────────────────────────────────────── //

/// Represents the details of an agent in the form of various strings and lists of phone numbers.
pub struct AgentDetails {
    /// The operation name
    /// Maximum length: 64 characters.
    pub operation_name: String,
    /// The name of the operator for fund transfers
    /// Maximum length: 64 characters.
    pub operator_name: String,
    /// The address of the operator for fund transfers
    /// Maximum length: 243 characters.
    pub operator_address: String,
    /// The tax identification number (INN) of the operator;
    /// Maximum length: 12 characters.
    pub operator_inn: String,
    /// The phone numbers of the payment agent; in `+{digit}` format.
    /// Each item must be 1 to 19 characters long.
    pub phones: Vec<PhoneNumber>,
    /// The phone numbers for transfers; in `+{digit}` format.
    /// Each item must be 1 to 19 characters long.
    pub transfer_phones: Vec<PhoneNumber>,
}

/// Agent sign params for initializing AgentData type.
pub enum AgentSignParams {
    BankPayingAgent(AgentDetails),
    BankPayingSubagent(AgentDetails),
    PayingAgent {
        /// The phone numbers of the payment agent; in `+{digit}` format.
        /// Each item must be 1 to 19 characters long.
        phones: Vec<PhoneNumber>,
        /// The phone numbers of the payment agent; in `+{digit}` format.
        /// Each item must be 1 to 19 characters long.
        receiver_phones: Vec<PhoneNumber>,
    },
    PayingSubagent {
        /// The phone numbers of the payment agent; in `+{digit}` format.
        /// Each item must be 1 to 19 characters long.
        phones: Vec<PhoneNumber>,
        /// The phone numbers of the payment agent; in `+{digit}` format.
        /// Each item must be 1 to 19 characters long.
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

/// Contains optional parameters for an agent's data.
/// To be used when an agent scheme is applied.
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
    /// Sets the operation name for the `AgentData`.
    ///
    /// This attribute is mandatory if `AgentSign` is one of the following:
    /// - bank_paying_agent
    /// - bank_paying_subagent
    ///
    /// The operation name should contain a maximum of 64 characters.
    pub fn with_operation_name(mut self, name: String) -> Self {
        self.operation_name = Some(name);
        self
    }
    /// Sets the operator name for the `AgentData`.
    ///
    /// This attribute is mandatory if `AgentSign` is one of the following:
    /// - bank_paying_agent
    /// - bank_paying_subagent
    ///
    /// The operator name should contain a maximum of 64 characters.
    pub fn with_operator_name(mut self, name: String) -> Self {
        self.operator_name = Some(name);
        self
    }
    /// Sets the operator address for the `AgentData`.
    ///
    /// This attribute is mandatory if `AgentSign` is one of the following:
    /// - bank_paying_agent
    /// - bank_paying_subagent
    ///
    /// The operator address should contain a maximum of 243 characters.
    pub fn with_operator_address(mut self, address: String) -> Self {
        self.operator_address = Some(address);
        self
    }
    /// Sets the tax identification number (INN) of the operator for the `AgentData`.
    ///
    /// This attribute is mandatory if `AgentSign` is one of the following:
    /// - bank_paying_agent
    /// - bank_paying_subagent
    ///
    /// The operator INN should contain a maximum of 12 characters.
    pub fn with_operator_inn(mut self, inn: String) -> Self {
        self.operator_inn = Some(inn);
        self
    }
    /// Adds a list of phone numbers associated with the agent.
    ///
    /// This function sets the phone numbers for the `AgentData` being built.
    /// The phone numbers must be in the format +{N}.
    pub fn with_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.phones = Some(phones);
        self
    }
    /// Assigns the phone numbers of the receiver operator.
    ///
    /// Use this method to set the receiver phones for the `AgentData`.
    /// The format expected is the international format beginning with a plus sign: +{N}.
    pub fn with_receiver_phones(mut self, phones: Vec<PhoneNumber>) -> Self {
        self.receiver_phones = Some(phones);
        self
    }
    /// Defines the transfer phones of the transfer operator.
    ///
    /// This method is utilized to specify the transfer phone numbers within the `AgentData`.
    /// Such numbers are to be formatted with a leading plus symbol followed by digits: +{N}.
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

/// The `SupplierInfo` type stores detailed information about a supplier.
///
/// It includes the supplier's name, identifier, contact details, and status.
/// This type is typically used in supply chain management systems to
/// access supplier-related data operations.
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
    /// Creates a new `SupplierInfo` object.
    ///
    /// All attributes are MANDATORY if the `AgentSign` value is passed in the `AgentData` object.
    ///
    /// - `phones`: An array of supplier phone numbers in the format +{CountryCode}{Number}.
    ///   Each phone number must be between 1 to 19 characters in length.
    ///   This attribute is required if the `AgentSign` value is passed in the `AgentData` object.
    ///
    /// - `name`: The name of the supplier.
    ///   This attribute is required if the `AgentSign` value is passed in the `AgentData` object.
    ///   The maximum length of the supplier name is 239 characters, which includes 4 characters for each phone number.
    ///   For example, if two supplier phone numbers are provided with lengths of 12 and 14 characters,
    ///   then the maximum length of the supplier name will be 239 - (12 + 4) - (14 + 4) = 205 characters.
    ///
    /// - `inn`: The Taxpayer Identification Number (INN) of the supplier.
    ///   This must be in the format of ten to twelve digits.
    ///   This attribute is required if the `AgentSign` value is passed in the `AgentData` object.
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

/// Represents the type of VAT (Value Added Tax) applicable.
///
/// # Variants
///
/// * `None` - No VAT.
/// * `Vat0` - VAT at the rate of 0%.
/// * `Vat10` - VAT at the rate of 10%.
/// * `Vat20` - VAT at the rate of 20%.
/// * `Vat110` - VAT at the fraction rate of 10/110, usually used for calculating VAT for a receipt.
/// * `Vat120` - VAT at the fraction rate of 20/120, usually used for calculating VAT for a receipt.
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

/// Describes the method of payment used in a transaction.
///
/// # Variants
///
/// * `FullPrepayment` - Full prepayment, 100% paid in advance.
/// * `Prepayment` - Partial prepayment paid in advance.
/// * `Advance` - Advance payment, typically paid before receiving goods or services.
/// * `FullPayment` - Complete payment made at the time of purchase.
/// * `PartialPayment` - Partial payment made with the understanding that the
/// remaining amount will be paid later, possibly financed through credit.
/// * `Credit` - Transfer of goods or services where payment is due at a later time.
/// * `CreditPayment` - A payment that is made towards clearing owed credit.
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

/// Represents the types of payment objects defined by the
/// Fiscal Feature Descriptor (FFD) version 1.2.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentObjectFFD_12 {
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

/// Represents the types of payment objects as per
/// Fiscal Feature Descriptor (FFD) version 1.05.
/// Similar to FFD 1.2 but tailored for a specific set of fiscal operations,
/// it covers a subset of transaction types necessary for
/// simplified fiscal reporting.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentObjectFFD_105 {
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

/// Describes various measurement units that can be used for item quantities in financial documents.
/// Each unit variant includes a serialization rename attribute for compatibility with external
/// systems that require specific formats, such as fiscal data operators.
///
/// This enumeration covers a comprehensive range of commonly used measurement units in commerce,
/// from simple countable units to volumetric and electronic data measurements, providing flexibility
/// in specifying the nature of the items being transacted.
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
    /// Product code with an unidentified format among the requisites.
    Unknown,
    /// Product code in EAN-8 format.
    Ean8,
    /// Product code in EAN-13 format.
    Ean13,
    /// Product code in ITF-14 format.
    Itf14,
    /// GS1 product code applied on an unmarked product.
    Gs10,
    /// GS1 product code applied on a marked product.
    Gs1m,
    /// Short marking code applied on a product.
    Short,
    /// Control identification mark for fur products.
    Fur,
    /// Product code in EGAISt-2.0 format.
    Egais20,
    /// Product code in EGAISt-3.0 format.
    Egais30,
    /// Marking code as read by a scanner.
    Rawcode,
}

/// Machine-readable mark code in the form of
/// one of the types of codes generated according to the requirements
/// provided by the rules for printing on consumer packaging,
/// or on goods, or on a product label.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkCode {
    /// Barcode type
    pub mark_code_type: MarkCodeType,
    /// Mark code
    pub value: String,
}

/// Sectoral attribute of the calculation subject.
///
/// It is necessary to specify only for goods subject to mandatory
/// marking with an identification tool, and the inclusion of this attribute
/// is provided for by industry-specific regulatory acts for the corresponding
/// product group.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SectoralItemProps {
    /// Identifier of the Federal Executive Authority
    pub federal_id: String,
    /// Date of the normative act of the Federal Executive Authority
    #[serde(serialize_with = "serialize_date_rfc3339")]
    pub date: PrimitiveDateTime,
    /// Number of the normative act of the Federal Executive Authority
    pub number: String,
    /// Composition of values​defined by the normative act of the
    /// Federal Executive Authority
    pub value: String,
}

/// Represents the detailed fiscal data for a transaction according to FFD 1.2 standards.
/// This structure is designed to integrate with fiscal data operators and is primarily
/// used for serialized data exchange between software systems and fiscal data recorders.
///
/// The `Ffd12Data` includes key fiscal attributes such as payment objects and methods,
/// important for correct fiscal reporting. Additionally, it holds optional fields
/// capturing detailed information like user data, excise amounts, country codes of origin for goods,
/// customs declaration numbers, and specific marking codes for tracked goods.
///
/// The structure enforces strict data formats and validation through its serialization
/// representation, ensuring compliance with fiscal regulations. It aligns with the required
/// formats for excise amounts, country codes, and custom declaration numbers, also handling
/// the inclusion of marked goods in the fiscal documents.
///
/// This is employed during the creation of fiscal documents where mandatory and regulated
/// data is necessary for proper transaction processing and reporting to fiscal authorities.
#[derive(Serialize, Validate)]
#[garde(allow_unvalidated)]
pub struct Ffd12Data {
    payment_object: PaymentObjectFFD_12,
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
    /// Creates a builder for constructing an instance of `Ffd12Data`.
    /// This function initializes the `Ffd12DataBuilder` with the mandatory fields which are necessary
    /// for any fiscal document according to FFD 1.2 standards.
    ///
    /// It requires the essential components of a fiscal operation which include the type of payment object,
    /// the method of payment, and the unit of measurement for the items involved. These core attributes
    /// define the basic structure of a fiscal transaction and are thus necessary parameters to initiate
    /// the builder pattern.
    ///
    /// The builder returned will have all optional fields uninitialized (set to `None`), allowing for
    /// optional attributes to be chained and set through the builder's setter methods, enabling
    /// a flexible and controlled construction of the `Ffd12Data` struct.
    ///
    /// Arguments:
    /// * `payment_object` - A `PaymentObjectFFD_12` which categorizes the transaction type.
    /// * `payment_method` - A `PaymentMethod` which indicates the method through which the
    /// payment is made.
    /// * `measurement_unit` - A `MeasurementUnit` which specifies the unit of measure of the items
    /// in the transaction.
    ///
    /// Returns:
    /// A `Ffd12DataBuilder` instance with the provided mandatory fields set and the optional fields
    /// unset, ready for further construction.
    pub fn builder(
        payment_object: PaymentObjectFFD_12,
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
    payment_object: PaymentObjectFFD_12,
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
    /// Additional user data field for the payment item.
    pub fn with_user_data(mut self, data: String) -> Self {
        self.user_data = Some(data);
        self
    }
    /// Excise amount in rubles, including cents, included in the cost of the payment item.
    /// Integer part not exceeding 8 digits;
    /// Fractional part not exceeding 2 digits;
    /// Value cannot be negative.
    pub fn with_excise(mut self, excise: Decimal) -> Self {
        self.excise = Some(excise);
        self
    }
    /// The digital country code of origin of goods in accordance with
    /// the All-Russian Classifier of World Countries.
    pub fn with_country_code(mut self, code: CountryCode) -> Self {
        self.country_code = Some(code);
        self
    }
    /// Customs declaration number
    /// Max length is 32.
    pub fn with_declaration_number(mut self, code: String) -> Self {
        self.declaration_number = Some(code);
        self
    }
    /// Marking code processing mode.
    pub fn mark_processing_mode(mut self) -> Self {
        self.mark_processing_mode = Some('0');
        self
    }
    /// Included in the receipt if the payment item is a product subject to mandatory marking with identification means.
    pub fn with_mark_code(mut self, code: MarkCode) -> Self {
        self.mark_code = Some(code);
        self
    }
    /// Industry-specific requirement of the payment item
    pub fn with_sectoral_item_props(
        mut self,
        props: SectoralItemProps,
    ) -> Self {
        self.sectoral_item_props = Some(props);
        self
    }
    /// Builds the Ffd12Data object.
    /// Returns a result of Ffd12Data or an error from the garde::Report.
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

/// `Ffd105Data` is a data structure that represents the fiscal information required
/// for reporting payment transactions to fiscal data operators like online cash registers.
///
/// Fields within this structure correspond to fiscal data standards, and they are
/// all optional to provide flexibility for data inclusion.
///
/// Fiscal data operators may include ATOL Online, CloudKassir, OrangeData, etc.,
/// and this structure ensures that the data conforms to the varied formats and
/// specifications required by these systems.
///
/// The structure uses Rust's serialization framework to enforce specific data
/// constraints, such as the maximum length of strings for EAN-13 barcodes, as well as
/// adapting field naming patterns to fit the desired serialization style (i.e., PascalCase).
///
/// For example:
///
/// - EAN-13 barcode validation accommodates different formatting requirements, like
///   hexadecimal representation with spaces for ATOL Online, even-length strings for
///   CloudKassir, and base64 encoded strings for OrangeData.
/// - Payment methods and objects are represented as enums with specific allowed values,
///   defaulting to "full_payment" and "commodity" respectively if not provided.
/// - Shop codes are used in scenarios that utilize submerchants registered through XML.
///
/// The structure is primarily used for serialization and validation when interfacing
/// with fiscal data systems and ensures compliance with fiscal regulations.
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
    payment_object: Option<PaymentObjectFFD_105>,
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
    payment_object: Option<PaymentObjectFFD_105>,
    payment_method: Option<PaymentMethod>,
}

impl Ffd105DataBuilder {
    /// Indication of payment method.
    pub fn with_payment_method(mut self, method: PaymentMethod) -> Self {
        self.payment_method = Some(method);
        self
    }
    /// Barcode in the required format. Requirements may vary depending on the type of cash register:
    ///
    /// ATOL Online - hexadecimal representation with spaces.
    /// Maximum length – 32 bytes
    /// (^[a-fA-F0-9]{2}$)|(^([a-fA-F0-9]{2}\s){1,31}[a-fA-F0-9]{2}$)
    /// Example:
    /// 00 00 00 01 00 21 FA 41 00 23 05 41 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 12 00 AB 00
    /// CloudKassir - string length: even, from 8 to 150 bytes, i.e.
    /// from 16 to 300 ASCII characters ['0' - '9', 'A' - 'F']
    /// hexadecimal representation of the product marking code.
    /// Example: 303130323930303030630333435
    /// OrangeData - a string containing a base64 encoded array
    /// from 8 to 32 bytes Example: igQVAAADMTIzNDU2Nzg5MDEyMwAAAAAAAQ==
    /// In case the Ean13 parameter transmitted in the request failed validation,
    /// an unsuccessful response is returned with the error message in the parameter
    /// message = "Invalid Ean13 parameter".
    pub fn with_ean_13(mut self, ean: &str) -> Self {
        self.ean_13 = Some(ean.to_string());
        self
    }
    /// Shop code. For the ShopСode parameter, you need to use
    /// the `Submerchant_ID` parameter value, received in response to
    /// registering shops through xml.
    /// If xml is not used, the field does not need to be transmitted.
    pub fn with_shop_code(mut self, code: &str) -> Self {
        self.shop_code = Some(code.to_string());
        self
    }
    /// Indication of a payment object.
    pub fn with_payment_object(mut self, obj: PaymentObjectFFD_105) -> Self {
        self.payment_object = Some(obj);
        self
    }
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

/// Receipt position with information about goods
///
/// Attributes specified in the protocol for sending receipts
/// for labeled goods are not mandatory for goods
/// without labeling. If FFD 1.2 is used, but the item being sold
/// is not subject to marking, then the fields can either not be sent or sent with a null value.
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
    /// Constructs a new `ItemBuilder` with the specified properties.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the item.
    /// * `price` - The price of the item in Kopecks, using the `Kopeck` type.
    /// * `quantity` - The quantity or weight of the item, represented as a `Decimal`. This value
    ///   must be formatted with a maximum of 8 characters, comprising up to 5 digits for the
    ///   integer part and up to 3 decimal places for Atol systems or 2 for CloudPayments systems.
    ///   Set the value to '1' if a MarkCode object is being passed.
    /// * `amount` - The total amount of the item in Kopecks, using the `Kopeck` type.
    /// * `tax` - The type of VAT to be applied to the item, using the `VatType` enum.
    /// * `cashbox_type` - The type of cashbox that will be used, `Atol` or `CloudPayments`.
    ///
    /// # Examples
    ///
    /// ```
    /// let item_builder = ItemBuilder::builder(
    ///     "Chocolate Bar",
    ///     Kopeck::new(4999),
    ///     Decimal::new(1, 0),
    ///     Kopeck::new(4999),
    ///     VatType::Vat20,
    ///     CashBoxType::Automated,
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// Returns an `ItemBuilder` instance, which can be used to build an item with additional
    /// optional properties.
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
    /// Agent data.
    ///
    /// If the AgentSign value is passed in the AgentData object,
    /// SupplierInfo must be fully initialized,
    /// otherwise `build` will return an error.
    pub fn with_agent_data(mut self, agent_data: AgentData) -> Self {
        self.agent_data = Some(agent_data);
        self
    }
    /// Payment agent supplier data.
    /// Mandatory if an AgentSign value is passed in the AgentData object.
    pub fn with_supplier_info(mut self, info: SupplierInfo) -> Self {
        self.supplier_info = Some(info);
        self
    }
    pub fn with_ffd_105_data(mut self, data: Ffd105Data) -> Self {
        self.ffd_105_data = Some(data);
        self
    }
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
