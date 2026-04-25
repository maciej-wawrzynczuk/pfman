//! Example CSV
//! ```csv
//! date;symbol;number;price;commission;currency
//! 2000-01-01;FOO;10;123.134;1.00;USD
//! ```

use std::io::Read;

use chrono::NaiveDate;
use csv::ReaderBuilder;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TransLogEDO {
    date: NaiveDate,
    symbol: String,
    number: i16,
    price: Decimal,
    commission: Decimal,
    currency: String
}

fn parse_csv<R: Read>(rdr: R) -> Result<Vec<TransLogEDO>, csv::Error> {
    ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(rdr)
        .deserialize()
        .collect()
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;
    #[test]
    fn deserialize1() {
        let csv1 = indoc! {"\
            date;symbol;number;price;commission;currency
            2000-01-01;FOO;10;123.134;1.00;USD
        "};
        let _result = parse_csv(csv1.as_bytes()).unwrap();
    }
}
