use super::transaction::TransactionEntry;

pub struct TransactionLog {
    entries: Vec<TransactionEntry>,
}

impl TransactionLog {
    pub fn new(entries: Vec<TransactionEntry>) -> Self {
        Self { entries }
    }

    pub fn iter(&self) -> impl Iterator<Item = &TransactionEntry> {
        self.entries.iter()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    fn entry(symbol: &str) -> TransactionEntry {
        TransactionEntry {
            date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            symbol: symbol.to_owned(),
            number: 1,
            price: dec!(1.00),
            commission: dec!(0.10),
            currency: "USD".to_owned(),
        }
    }

    #[test]
    fn iter_yields_all_entries_in_order() {
        let log = TransactionLog::new(vec![entry("AAPL"), entry("MSFT"), entry("GOOG")]);
        let symbols: Vec<&str> = log.iter().map(|e| e.symbol.as_str()).collect();
        assert_eq!(symbols, ["AAPL", "MSFT", "GOOG"]);
    }

    #[test]
    fn is_empty_on_empty_log() {
        let log = TransactionLog::new(vec![]);
        assert!(log.is_empty());
    }

    #[test]
    fn is_empty_false_on_non_empty_log() {
        let log = TransactionLog::new(vec![entry("AAPL")]);
        assert!(!log.is_empty());
    }

    #[test]
    fn len_returns_correct_count() {
        let log = TransactionLog::new(vec![entry("AAPL"), entry("MSFT")]);
        assert_eq!(log.len(), 2);
    }
}
