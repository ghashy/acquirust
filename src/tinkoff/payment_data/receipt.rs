use serde::{ser::SerializeStruct, Serialize, Serializer};
use time::PrimitiveDateTime;

use crate::domain::email::Email;

#[derive(Serialize)]
enum DocumentCode {
    PassportRussianCitizen,
    PassportRussianCitizenDiplomaticService,
    TemporaryIdentityCard,
    BirthCertificateRussianCitizen,
    OtherRussianCitizenIdentityDocument,
    ForeignCitizenPassport,
    OtherForeignCitizenIdentityDocument,
    ForeignDocumentRecognizedInternationalTreaty,
    ResidencePermit,
    TemporaryResidencePermit,
    RefugeeApplicationConsiderationCertificate,
    RefugeeCertificate,
    OtherStatelessPersonIdentityDocument,
    IdentityDocumentUnderConsideration,
}

impl DocumentCode {
    fn get_code(&self) -> u32 {
        match self {
            DocumentCode::PassportRussianCitizen => 21,
            DocumentCode::PassportRussianCitizenDiplomaticService => 22,
            DocumentCode::TemporaryIdentityCard => 26,
            DocumentCode::BirthCertificateRussianCitizen => 27,
            DocumentCode::OtherRussianCitizenIdentityDocument => 28,
            DocumentCode::ForeignCitizenPassport => 31,
            DocumentCode::OtherForeignCitizenIdentityDocument => 32,
            DocumentCode::ForeignDocumentRecognizedInternationalTreaty => 33,
            DocumentCode::ResidencePermit => 34,
            DocumentCode::TemporaryResidencePermit => 35,
            DocumentCode::RefugeeApplicationConsiderationCertificate => 36,
            DocumentCode::RefugeeCertificate => 37,
            DocumentCode::OtherStatelessPersonIdentityDocument => 38,
            DocumentCode::IdentityDocumentUnderConsideration => 40,
        }
    }

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
    Ver1_2,
    Ver1_05,
}

#[derive(Serialize)]
pub struct ClientInfo {
    birth_date: PrimitiveDateTime,
    // Числовой код страны, гражданином которой является клиент.
    // Код страны указывается в соответствии с
    // Общероссийским классификатором стран мира ОКСМ
    citizenship: u16,
    document_code: DocumentCode,
    // Реквизиты документа, удостоверяющего личность (например: серия и номер паспорта)
    document_data: String,
    // Адрес клиента, грузополучателя
    address: String, // should be <= 256 characters
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

/// <= 64 characters
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
                state.serialize_field("email", email.as_ref())?;
                state.end()
            }
            EmailOrPhone::Phone(phone) => {
                let mut state =
                    serializer.serialize_struct("EmailOrPhone", 1)?;
                // FIXME: test how phone.to_string looks. Is it correct?
                state.serialize_field("phone", &phone.to_string())?;
                state.end()
            }
        }
    }
}

#[derive(Serialize)]
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
}
