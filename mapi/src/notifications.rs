use serde::{Deserialize, Serialize};

use crate::{domain::Kopeck, receipt::Receipt};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationData {
    /// Value: "TCB", способ платежа
    #[serde(skip_serializing_if = "Option::is_none")]
    route: Option<String>,
    /// Value: "Installment", источник платежа
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<String>,
    /// Сумма выданного кредита в копейках
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_amount: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationPayment {
    /// Идентификатор терминала. Выдается Мерчанту Тинькофф Кассой при заведении терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_key: Option<String>,
    /// Сумма в копейках
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Kopeck>,
    /// Идентификатор заказа в системе Мерчанта
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
    /// Выполнение платежа
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
    /// Статус платежа
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    /// Уникальный идентификатор транзакции в системе Тинькофф Кассы
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_id: Option<u64>,
    /// Код ошибки. «0» в случае успеха
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
    /// Краткое описание ошибки
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    /// Подробное описание ошибки
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    /// Идентификатор автоплатежа
    #[serde(skip_serializing_if = "Option::is_none")]
    rebill_id: Option<u64>,
    /// Идентификатор карты в системе Тинькофф Кассы
    #[serde(skip_serializing_if = "Option::is_none")]
    card_id: Option<i32>,
    /// Замаскированный номер карты/Замаскированный номер телефона
    #[serde(skip_serializing_if = "Option::is_none")]
    pan: Option<String>,
    /// Срок действия карты В формате MMYY, где YY — две последние цифры года
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_date: Option<String>,
    /// Подпись запроса. Формируется по такому же принципу, как и в случае запросов в Тинькофф Кассу
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    /// Дополнительные параметры платежа, переданные при создании заказа. Явяляются обязательными для платежей «в Рассрочку»
    #[serde(skip_serializing_if = "Option::is_none", rename = "DATA")]
    data: Option<NotificationData>,
}

/// Статус привязки карты. Получает в ответе 1 из 2 статусов привязки
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AddCardStatus {
    /// При одностадийной оплате
    Completed,
    /// При двухстадийной оплате
    Rejected,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationAddCard {
    /// Идентификатор терминала. Выдается Мерчанту Тинькофф Кассой при заведении терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_key: Option<String>,
    /// Идентификатор клиента в системе Мерчанта
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_key: Option<String>,
    /// Идентификатор запроса на привязку карты
    #[serde(skip_serializing_if = "Option::is_none")]
    request_key: Option<uuid::Uuid>,
    /// Выполнение платежа
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
    /// Статус привязки карты
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<AddCardStatus>,
    /// Уникальный идентификатор транзакции в системе Тинькофф Кассы
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_id: Option<u64>,
    /// Код ошибки. «0» в случае успеха
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
    /// Идентификатор автоплатежа
    #[serde(skip_serializing_if = "Option::is_none")]
    rebill_id: Option<u64>,
    /// Идентификатор карты в системе Тинькофф Кассы
    #[serde(skip_serializing_if = "Option::is_none")]
    card_id: Option<i32>,
    /// Замаскированный номер карты/Замаскированный номер телефона
    #[serde(skip_serializing_if = "Option::is_none")]
    pan: Option<String>,
    /// Срок действия карты В формате MMYY, где YY — две последние цифры года
    #[serde(skip_serializing_if = "Option::is_none")]
    exp_date: Option<String>,
    /// Подпись запроса. Формируется по такому же принципу, как и в случае запросов в Тинькофф Кассу
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationFiscalization {
    /// Идентификатор терминала. Выдается Мерчанту Тинькофф Кассой при заведении терминала.
    #[serde(skip_serializing_if = "Option::is_none")]
    terminal_key: Option<String>,
    /// Идентификатор заказа в системе Мерчанта
    #[serde(skip_serializing_if = "Option::is_none")]
    order_id: Option<String>,
    /// Выполнение платежа
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<bool>,
    /// Для нотификации о фискализации значение всегда RECEIPT
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    /// Уникальный идентификатор транзакции в системе Тинькофф Кассы
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_id: Option<u64>,
    /// Код ошибки. «0» в случае успеха
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
    /// Краткое описание ошибки
    #[serde(skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    /// Сумма в копейках
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<Kopeck>,
    /// Номер чека в смене
    #[serde(skip_serializing_if = "Option::is_none")]
    fiscal_number: Option<i32>,
    /// Номер смены
    #[serde(skip_serializing_if = "Option::is_none")]
    shift_number: Option<i32>,
    /// Дата и время документа из ФН
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt_date_time: Option<String>,
    /// Номер ФН
    #[serde(skip_serializing_if = "Option::is_none")]
    fn_number: Option<String>,
    /// Регистрационный номер ККТ
    #[serde(skip_serializing_if = "Option::is_none")]
    ecr_reg_number: Option<String>,
    /// Фискальный номер документа
    #[serde(skip_serializing_if = "Option::is_none")]
    fiscal_document_number: Option<i32>,
    /// Фискальный признак документа
    #[serde(skip_serializing_if = "Option::is_none")]
    fiscal_document_attribute: Option<i32>,
    /// Состав чека
    #[serde(skip_serializing_if = "Option::is_none")]
    receipt: Option<Receipt>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "Type")]
    transaction_type: Option<String>,
    /// Подпись запроса. Формируется по такому же принципу, как и в случае запросов в Тинькофф Кассу
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    /// Наименование оператора фискальных данных
    #[serde(skip_serializing_if = "Option::is_none")]
    ofd: Option<String>,
    /// URL адрес с копией чека
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    /// URL адрес с QR кодом для проверки чека в ФНС
    #[serde(skip_serializing_if = "Option::is_none")]
    qr_code_url: Option<String>,
    /// Место осуществления расчетов
    #[serde(skip_serializing_if = "Option::is_none")]
    calculation_place: Option<String>,
    /// Имя кассира
    #[serde(skip_serializing_if = "Option::is_none")]
    cashier_name: Option<String>,
    /// Место нахождения (установки) ККМ
    #[serde(skip_serializing_if = "Option::is_none")]
    selltle_place: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NotificationQr {
    /// Идентификатор терминала. Выдается Мерчанту Тинькофф Кассой при заведении терминала.
    terminal_key: String,
    /// Идентификатор запроса на привязку счета
    #[serde(skip_serializing_if = "Option::is_none")]
    request_key: Option<uuid::Uuid>,
    /// Идентификатор привязки счета, назначаемый банком-эмитентом
    #[serde(skip_serializing_if = "Option::is_none")]
    account_token: Option<String>,
    /// Идентификатор банка-эмитента клиента, который будет совершать оплату по привязаному счету - заполнен, если статус ACTIVE
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_member_id: Option<String>,
    /// Наименование банка-эмитента, заполнен если BankMemberId передан
    #[serde(skip_serializing_if = "Option::is_none")]
    bank_member_name: Option<String>,
    /// Тип нотификации, всегда константа «LINKACCOUNT»
    notification_type: String,
    /// Успешность операции
    success: bool,
    /// Код ошибки. «0» в случае успеха
    error_code: String,
    /// Краткое описание ошибки
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    /// Подпись запроса. Формируется по такому же принципу, как и в случае запросов в Тинькофф Кассу
    token: String,
    /// Cтатус привязки
    status: String,
}

/// На стороне Мерчанта для получения уведомлений об изменении статуса платежа
/// реализуется POST метод, принимающий тип `Notification` в виде JSON-body.
#[derive(Deserialize, Serialize)]
pub enum Notification {
    NotificationPayment(NotificationPayment),
    /// Нотификации о привязке (Для Мерчантов с PCI DSS)
    ///
    /// Уведомления магазину о статусе выполнения метода привязки карты AttachCard.
    /// После успешного выполнения метода AttachCard Тинькофф Касса отправляет
    /// POST-запрос с информацией о привязке карты. Нотификация отправляется на ресурс
    /// Мерчанта на адрес Notification URL синхронно и ожидает ответа в течение 10 секунд.
    /// После получения ответа или неполучения его за заданное время сервис переадресует
    /// клиента на Success AddCard URL или Fail AddCard URL в зависимости от результата
    /// привязки карты. В случае успешной обработки нотификации Мерчант должен вернуть
    /// ответ с телом сообщения: OK (без тегов и заглавными английскими буквами).
    /// Если тело сообщения отлично от OK, любая нотификация считается неуспешной,
    /// и сервис будет повторно отправлять нотификацию раз в час в течение 24 часов.
    /// Если нотификация за это время так и не доставлена, она складывается в дамп.
    NotificationAddCard(NotificationAddCard),
    /// Если используется подключенная онлайн касса, по результату фискализации будет отправлена нотификация с фискальными данными.
    NotificationFiscalization(NotificationFiscalization),
    /// После привязки счета по QR, магазину отправляется статус привязки и токен. Нотификация будет приходить по статусам ACTIVE и INACTIVE.
    NotificationQr(NotificationQr),
}
