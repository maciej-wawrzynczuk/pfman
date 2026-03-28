use crate::domain::transaction::TransactionEntry;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TransactionEntryDto {
    pub date: String,
    pub symbol: String,
    pub number: i64,
    #[serde(with = "rust_decimal::serde::str")]
    pub price: rust_decimal::Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub commission: rust_decimal::Decimal,
    pub currency: String,
}

impl From<&TransactionEntry> for TransactionEntryDto {
    fn from(e: &TransactionEntry) -> Self {
        Self {
            date: e.date.format("%Y-%m-%d").to_string(),
            symbol: e.symbol.clone(),
            number: e.number,
            price: e.price,
            commission: e.commission,
            currency: e.currency.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    #[test]
    fn decimal_and_date_serialize_as_exact_strings() {
        let entry = TransactionEntry {
            date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            symbol: "AAPL".to_owned(),
            number: 10,
            price: dec!(182.50),
            commission: dec!(1.00),
            currency: "USD".to_owned(),
        };
        let dto = TransactionEntryDto::from(&entry);
        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains(r#""date":"2024-01-15""#));
        assert!(json.contains(r#""price":"182.50""#));
        assert!(json.contains(r#""commission":"1.00""#));
    }
}
