use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// ───── Beneficiaries ────────────────────────────────────────────────────── //

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Beneficiar {
    card_token: String,
    part: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Beneficiaries {
    beneficiaries: Vec<Beneficiar>,
}

impl Beneficiaries {
    pub const NONE: Beneficiaries = Beneficiaries {
        beneficiaries: Vec::new(),
    };

    pub fn is_empty(&self) -> bool {
        self.beneficiaries.is_empty()
    }

    pub fn as_str(&self) -> String {
        self.beneficiaries
            .iter()
            .fold(String::new(), |mut acc, next| {
                acc.push_str(&next.card_token);
                acc
            })
    }

    pub fn iter_tokens(&self) -> BeneficiariesIterator<'_> {
        BeneficiariesIterator {
            internal: self.beneficiaries.iter(),
        }
    }

    pub fn validate(&self) -> Result<(), ()> {
        todo!()
    }
}

// ───── Beneficiaries Builder ────────────────────────────────────────────── //

pub struct BeneficiariesBuilder {
    beneficiaries: Vec<Beneficiar>,
}

impl BeneficiariesBuilder {
    pub fn new(card_token: String, part: Decimal) -> BeneficiariesBuilder {
        BeneficiariesBuilder {
            beneficiaries: vec![Beneficiar { card_token, part }],
        }
    }

    pub fn add(&mut self, card_token: String, part: Decimal) {
        self.beneficiaries.push(Beneficiar { card_token, part })
    }

    pub(crate) fn build(self) -> Result<Beneficiaries, ()> {
        let total = self
            .beneficiaries
            .iter()
            .fold(Decimal::ZERO, |acc, sum| acc + sum.part);
        if total != Decimal::ONE {
            Err(())
        } else {
            Ok(Beneficiaries {
                beneficiaries: self.beneficiaries,
            })
        }
    }
}

// ───── Beneficiaries Iterator ───────────────────────────────────────────── //

pub struct BeneficiariesIterator<'a> {
    internal: core::slice::Iter<'a, Beneficiar>,
}

impl<'a> Iterator for BeneficiariesIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next().map(|inner| inner.card_token.as_str())
    }
}
