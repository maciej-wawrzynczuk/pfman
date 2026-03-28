use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionEntry {
    #[serde(rename = "date")]
    pub date: NaiveDate,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "number")]
    pub number: i64,
    #[serde(rename = "price", with = "rust_decimal::serde::str")]
    pub price: Decimal,
    #[serde(rename = "commission", with = "rust_decimal::serde::str")]
    pub commission: Decimal,
    #[serde(rename = "currency")]
    pub currency: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    fn deserialize(row: &str) -> Result<TransactionEntry, csv::Error> {
        let header = "date;symbol;number;price;commission;currency\n";
        let input = format!("{header}{row}");
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(input.as_bytes());
        rdr.deserialize::<TransactionEntry>().next().unwrap()
    }

    #[test]
    fn valid_row_deserializes_correctly() {
        let entry = deserialize("2024-01-15;AAPL;10;182.50;1.00;USD").unwrap();
        assert_eq!(entry.date, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
        assert_eq!(entry.symbol, "AAPL");
        assert_eq!(entry.number, 10);
        assert_eq!(entry.price, dec!(182.50));
        assert_eq!(entry.commission, dec!(1.00));
        assert_eq!(entry.currency, "USD");
    }

    #[test]
    fn invalid_date_returns_error() {
        assert!(deserialize("not-a-date;AAPL;10;182.50;1.00;USD").is_err());
    }

    #[test]
    fn invalid_decimal_in_price_returns_error() {
        assert!(deserialize("2024-01-15;AAPL;10;not-a-number;1.00;USD").is_err());
    }
}
