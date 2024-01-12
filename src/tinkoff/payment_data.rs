use std::char;

pub enum Source {
    TinkoffPay,
    SBPQR,
    YandexPay,
}

#[allow(non_camel_case_types)]
pub enum OperationInitiatorType {
    // Сustomer Initiated Credential-Not-Captured
    // Стандартный платеж.
    //
    // Инициированная покупателем оплата без сохранения реквизитов карты
    // для последующего использования.
    CIT_CNC,
    // Сustomer Initiated Credential-Captured
    // Стандартный платеж с созданием родительского рекуррентного платежа.
    //
    // Инициированная покупателем оплата c сохранением реквизитов карты
    // для последующего использования.
    CIT_CC,
    // Сustomer Initiated Credential-on-File
    // Рекуррентный платеж, инициированный покупателем.
    //
    // Инициированная покупателем оплата по сохраненным реквизитам карты
    // (ранее была проведена операция с сохранением реквизитов CIT CC).
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
    CIT_COF_I,
}

impl OperationInitiatorType {
    fn as_char(&self) -> char {
        match self {
            OperationInitiatorType::CIT_CNC => '0',
            OperationInitiatorType::CIT_CC => '1',
            OperationInitiatorType::CIT_COF => '2',
            OperationInitiatorType::CIT_COF_R => 'R',
            OperationInitiatorType::CIT_COF_I => 'I',
        }
    }

    fn allowed_with_recurrent_init(&self) -> bool {
        match self {
            OperationInitiatorType::CIT_CNC => false,
            OperationInitiatorType::CIT_CC => true,
            OperationInitiatorType::CIT_COF => false,
            OperationInitiatorType::CIT_COF_R => false,
            OperationInitiatorType::CIT_COF_I => false,
        }
    }

    fn allowed_with_RebillId 

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

/// Максимальная длина для каждого передаваемого параметра:
///
/// Ключ - 20 знаков
/// Значение - 100 знаков.
pub struct PaymentData {
    phone: Option<String>,
    email: Option<String>,
    account: Option<String>,
    default_card: Option<String>,
    tinkoff_pay_web: Option<bool>,
    yandex_pay_web: Option<bool>,
    device: Option<String>,
    device_os: Option<String>,
    device_web_view: Option<bool>,
    device_browser: Option<String>,
    notification_enable_source: Option<Source>,
    qr: Option<bool>,
    operation_initiator_type: Option<OperationInitiatorType>,
}
