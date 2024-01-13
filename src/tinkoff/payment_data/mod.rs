use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
pub enum Source {
    TinkoffPay,
    SBPQR,
    YandexPay,
}

#[allow(non_camel_case_types)]
#[derive(Serialize)]
pub enum OperationInitiatorType {
    // Сustomer Initiated Credential-Not-Captured
    // Стандартный платеж.
    //
    // Инициированная покупателем оплата без сохранения реквизитов карты
    // для последующего использования.
    #[serde(rename = "0")]
    CIT_CNC,
    // Сustomer Initiated Credential-Captured
    // Стандартный платеж с созданием родительского рекуррентного платежа.
    //
    // Инициированная покупателем оплата c сохранением реквизитов карты
    // для последующего использования.
    #[serde(rename = "1")]
    CIT_CC,
    // Сustomer Initiated Credential-on-File
    // Рекуррентный платеж, инициированный покупателем.
    //
    // Инициированная покупателем оплата по сохраненным реквизитам карты
    // (ранее была проведена операция с сохранением реквизитов CIT CC).
    #[serde(rename = "2")]
    CIT_COF,
    // Merchant Initiated Credential-on-File, Recurring
    // Рекуррентный платеж, инициированный торговым предприятием.
    //
    // Инициированные торговым предприятием повторяющиеся платежи без графика
    // (ранее была проведена операция с сохранением реквизитов CIT CC).
    // Применяются для оплаты коммунальных услуг, платежей за услуги связи,
    // кабельное/спутниковое телевидение и т.п.
    // Сумма может быть определена заранее или становится известна
    // непосредственно перед оплатой.
    #[serde(rename = "R")]
    CIT_COF_R,
    // Merchant Credential-on-File, Installment
    // Рекуррентный платеж, инициированный торговым предприятием.
    //
    // Инициированные торговым предприятием повторяющиеся платежи по графику
    // (ранее была проведена операция с сохранением реквизитов CIT CC).
    // Применяется для платежей в рассрочку по товарному кредиту,
    // для оплаты страховки в рассрочку, для погашения кредита в соответствии с
    // графиком платежей. График платежей может быть изменен по соглашению сторон,
    // т.е. суммы и даты платежей должны быть известны плательщику
    // (держателю карты) до момента проведения операции.
    #[serde(rename = "I")]
    CIT_COF_I,
}

impl OperationInitiatorType {
    fn allowed_with_recurrent_init(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => false,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => false,
            OperationInitiatorType::CIT_COF_R => false,
            OperationInitiatorType::CIT_COF_I => false,
        }
    }

    fn allowed_with_rebill_id_at_charge(&self) -> Option<()> {
        match self {
            OperationInitiatorType::CIT_CNC => None,
            OperationInitiatorType::CIT_CC => None,
            OperationInitiatorType::CIT_COF => Some(()),
            OperationInitiatorType::CIT_COF_R => Some(()),
            OperationInitiatorType::CIT_COF_I => Some(()),
        }
    }

    // AFT – это автоматизированные терминалы сбора платежей,
    // часто используемые в транспортной системе для оплаты проезда.
    // К примеру, это могут быть терминалы для продажи билетов или
    // пополнения транспортных карт в метро,
    // на автобусных станциях или на железнодорожных вокзалах.
    fn allowed_with_aft(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => true,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => false,
            OperationInitiatorType::CIT_COF_R => false,
            OperationInitiatorType::CIT_COF_I => true,
        }
    }

    // ECOM-терминалы – это терминалы, предназначенные для электронной коммерции.
    // Они могут использоваться в розничной торговле для обработки платежных карт,
    // мобильных платежей и других видов электронных платежей.
    // Такие терминалы обычно предоставляют возможность безналичной оплаты
    // за товары и услуги в интернет-магазинах или в торговых точках.
    fn allowed_with_ecom(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => true,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => true,
            OperationInitiatorType::CIT_COF_R => true,
            OperationInitiatorType::CIT_COF_I => false,
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
        operation_initiator_type: OperationInitiatorType,
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

/// Максимальная длина для каждого передаваемого параметра:
///
/// Ключ - 20 знаков
/// Значение - 100 знаков.
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentData {
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_enable_source: Option<Source>,
    // Для осуществления привязки и одновременной оплаты по CБП
    // необходимо передавать параметр "QR" = "true"
    #[serde(skip_serializing_if = "Option::is_none")]
    qr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation_initiator_type: Option<OperationInitiatorType>,
}
