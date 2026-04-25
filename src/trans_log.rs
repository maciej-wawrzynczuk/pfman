//! Example CSV
//! ```csv
//! date;symbol;number;price;commission;currency
//! 2000-01-01;FOO;10;123.134;1.00;USD
//! ```

use chrono::NaiveDate;
use csv::ReaderBuilder;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use std::io::Read;

#[derive(Serialize, PartialEq, Debug)]
pub struct TransLog {
    data: Vec<TransEntry>
}

impl TransLog {
    pub fn from_reader<R: Read>(rdr: R) -> Result<Self, csv::Error> {
        let data = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(rdr)
            .deserialize()
            .collect::<Result<Vec<TransEntry>,_>>()?;

        Ok(Self { data })
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct TransEntry {
    date: NaiveDate,
    symbol: String,
    number: i16,
    price: Decimal,
    commission: Decimal,
    currency: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use rust_decimal::dec;
    #[test]
    fn json_roundtrip() {
        let csv1 = indoc! {"\
            date;symbol;number;price;commission;currency
            2000-01-02;FOO;10;123.134;1.00;USD
        "};

        let sut = TransLog::from_reader(csv1.as_bytes()).unwrap();
        let expected: TransLog = TransLog { 
            data: vec! [
                TransEntry {
                    date: chrono::NaiveDate::from_ymd_opt(2000, 1, 2).unwrap(),
                    symbol: "FOO".into(),
                    number: 10,
                    price: dec!(123.134),
                    commission: dec!(1),
                    currency: "USD".into()
                }
            ]
         };
         assert_eq!(sut, expected);
    }
}
