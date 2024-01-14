use std::collections::HashMap;

use garde::Validate;
use serde::Serialize;

use crate::domain::Email;
use crate::{error_chain_fmt, serialize_phonenumber};

use super::payment::TerminalType;

#[derive(Serialize)]
pub enum Source {
    TinkoffPay,
    SBPQR,
    YandexPay,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug, Clone)]
pub enum OperationInitiatorType {
    /// Сustomer Initiated Credential-Not-Captured
    /// Стандартный платеж.
    ///
    /// Инициированная покупателем оплата без сохранения реквизитов карты
    /// для последующего использования.
    #[serde(rename = "0")]
    CIT_CNC,
    /// Сustomer Initiated Credential-Captured
    /// Стандартный платеж с созданием родительского рекуррентного платежа.
    ///
    /// Инициированная покупателем оплата c сохранением реквизитов карты
    /// для последующего использования.
    #[serde(rename = "1")]
    CIT_CC,
    /// Сustomer Initiated Credential-on-File
    /// Рекуррентный платеж, инициированный покупателем.
    ///
    /// Инициированная покупателем оплата по сохраненным реквизитам карты
    /// (ранее была проведена операция с сохранением реквизитов CIT CC).
    #[serde(rename = "2")]
    CIT_COF,
    /// Merchant Initiated Credential-on-File, Recurring
    /// Рекуррентный платеж, инициированный торговым предприятием.
    ///
    /// Инициированные торговым предприятием повторяющиеся платежи без графика
    /// (ранее была проведена операция с сохранением реквизитов CIT CC).
    /// Применяются для оплаты коммунальных услуг, платежей за услуги связи,
    /// кабельное/спутниковое телевидение и т.п.
    /// Сумма может быть определена заранее или становится известна
    /// непосредственно перед оплатой.
    #[serde(rename = "R")]
    CIT_COF_R,
    /// Merchant Credential-on-File, Installment
    /// Рекуррентный платеж, инициированный торговым предприятием.
    ///
    /// Инициированные торговым предприятием повторяющиеся платежи по графику
    /// (ранее была проведена операция с сохранением реквизитов CIT CC).
    /// Применяется для платежей в рассрочку по товарному кредиту,
    /// для оплаты страховки в рассрочку, для погашения кредита в соответствии с
    /// графиком платежей. График платежей может быть изменен по соглашению сторон,
    /// т.е. суммы и даты платежей должны быть известны плательщику
    /// (держателю карты) до момента проведения операции.
    #[serde(rename = "I")]
    CIT_COF_I,
}

impl OperationInitiatorType {
    pub(super) fn allowed_with_recurrent_init(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => false,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => false,
            OperationInitiatorType::CIT_COF_R => false,
            OperationInitiatorType::CIT_COF_I => false,
        }
    }

    pub(super) fn allowed_with_rebill_id_at_charge(&self) -> Option<()> {
        match self {
            OperationInitiatorType::CIT_CNC => None,
            OperationInitiatorType::CIT_CC => None,
            OperationInitiatorType::CIT_COF => Some(()),
            OperationInitiatorType::CIT_COF_R => Some(()),
            OperationInitiatorType::CIT_COF_I => Some(()),
        }
    }

    fn allowed_with_aft(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => true,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => false,
            OperationInitiatorType::CIT_COF_R => false,
            OperationInitiatorType::CIT_COF_I => true,
        }
    }

    fn allowed_with_ecom(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => true,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => true,
            OperationInitiatorType::CIT_COF_R => true,
            OperationInitiatorType::CIT_COF_I => false,
        }
    }

    pub(super) fn validate_terminal_type(
        &self,
        terminal_type: &TerminalType,
    ) -> Result<(), ()> {
        match terminal_type {
            TerminalType::ECOM => {
                if self.allowed_with_ecom() {
                    Ok(())
                } else {
                    Err(())
                }
            }
            TerminalType::AFT => {
                if self.allowed_with_aft() {
                    Ok(())
                } else {
                    Err(())
                }
            }
        }
    }
}

#[derive(Serialize)]
pub enum DeviceType {
    SDK,
    Desktop,
    MobileWeb,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum PayMethod {
    Common {
        additional_properties: String,
    },
    TinkoffPay {
        device: DeviceType,
        device_os: String,
        // Признак открытия в WebView
        device_web_view: bool,
        device_browser: String,
        // Признак проведения операции через Tinkoff Pay по API
        tinkoff_pay_web: bool,
    },
    YandexPay {
        // Признак проведения операции через Yandex Pay
        yandex_pay_web: bool,
    },
    // TODO: Implement LongPlay
    LongPlay,
}

#[derive(thiserror::Error)]
pub enum PaymentDataParseError {
    #[error("Too many fields: {0}, but max is 20")]
    TooManyFields(u32),
}

impl std::fmt::Debug for PaymentDataParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Serialize, Validate)]
#[serde(rename_all = "PascalCase")]
#[garde(allow_unvalidated)]
pub struct PaymentData {
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_phonenumber"
    )]
    phone: Option<phonenumber::PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[garde(length(max = 30))]
    account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_card: Option<String>,
    /// Параметр позволяет отправлять нотификации только если Source
    /// (также присутствует в параметрах сессии) платежа входит в
    /// перечень указанных в параметре.
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_enable_source: Option<Source>,
    // Для осуществления привязки и одновременной оплаты по CБП
    // необходимо передавать параметр "QR" = "true"
    #[serde(skip_serializing_if = "Option::is_none", rename = "QR")]
    qr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_initiator_type: Option<OperationInitiatorType>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pay_method: Option<PayMethod>,
    /// Additional fields
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    other: Option<HashMap<String, String>>,
}

impl PaymentData {
    /// Максимальная длина для каждого передаваемого параметра:
    ///
    /// Ключ - 20 знаков
    /// Значение - 100 знаков.
    /// Максимальное количество пар "ключ":"значение" - 20.
    pub fn builder() -> PaymentDataBuilder {
        PaymentDataBuilder::default()
    }
    pub(super) fn initiator_type(&self) -> Option<&OperationInitiatorType> {
        self.operation_initiator_type.as_ref()
    }
}

#[derive(Default)]
pub struct PaymentDataBuilder {
    phone: Option<phonenumber::PhoneNumber>,
    email: Option<Email>,
    account: Option<String>,
    default_card: Option<String>,
    notification_enable_source: Option<Source>,
    qr: Option<bool>,
    operation_initiator_type: Option<OperationInitiatorType>,
    pay_method: Option<PayMethod>,
    other: Option<HashMap<String, String>>,
    count: u32,
}

impl PaymentDataBuilder {
    /// Для МСС 4814 обязательно передать значение в параметре Phone.
    pub fn with_phone(mut self, phone: phonenumber::PhoneNumber) -> Self {
        self.phone = Some(phone);
        self.count += 1;
        self
    }
    pub fn with_email(mut self, email: Email) -> Self {
        self.email = Some(email);
        self.count += 1;
        self
    }
    /// Для МСС 6051 и 6050 обязательно передать параметр account
    /// (номер электронного кошелька, не должен превышать 30 символов
    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self.count += 1;
        self
    }
    /// Если используется функционал сохранения карт на платежной форме,
    /// то при помощи опционального параметра DefaultCard можно задать
    /// какая карта будет выбираться по умолчанию.
    ///
    /// Чтобы оставить платежную форму пустой, передайте `none`.
    pub fn with_default_card(mut self, card: String) -> Self {
        self.default_card = Some(card);
        self.count += 1;
        self
    }
    /// Позволяет отправлять нотификации только если Source
    /// платежа входит в перечень указанных в параметре.
    pub fn with_notification_source(mut self, source: Source) -> Self {
        self.notification_enable_source = Some(source);
        self.count += 1;
        self
    }
    /// Для осуществления привязки и одновременной оплаты по CБП
    /// необходимо передавать параметр "QR" = "true".
    pub fn with_qr(mut self) -> Self {
        self.qr = Some(true);
        self.count += 1;
        self
    }
    /// Передача признака инициатора операции.
    pub fn with_operation_initiator_type(
        mut self,
        initiator: OperationInitiatorType,
    ) -> Self {
        self.operation_initiator_type = Some(initiator);
        self.count += 1;
        self
    }
    pub fn with_pay_method(mut self, method: PayMethod) -> Self {
        match method {
            PayMethod::Common { .. } => self.count += 1,
            PayMethod::TinkoffPay { .. } => self.count += 5,
            PayMethod::YandexPay { .. } => self.count += 1,
            PayMethod::LongPlay => unimplemented!(),
        }
        self.pay_method = Some(method);
        self
    }
    pub fn with_other(mut self, params: HashMap<String, String>) -> Self {
        self.count += params.len() as u32;
        self.other = Some(params);
        self
    }
    pub fn build(self) -> Result<PaymentData, PaymentDataParseError> {
        if self.count > 20 {
            return Err(PaymentDataParseError::TooManyFields(self.count));
        }
        Ok(PaymentData {
            phone: self.phone,
            email: self.email,
            account: self.account,
            default_card: self.default_card,
            notification_enable_source: self.notification_enable_source,
            qr: self.qr,
            operation_initiator_type: self.operation_initiator_type,
            pay_method: self.pay_method,
            other: self.other,
        })
    }
}
