use crate::domain::transaction::TransactionEntry;

/// Parse a semicolon-delimited CSV byte slice into a list of transaction entries.
///
/// # Errors
///
/// Returns a [`csv::Error`] if the input is malformed, has missing columns,
/// or contains values that cannot be deserialized into [`TransactionEntry`].
pub fn parse(input: &[u8]) -> Result<Vec<TransactionEntry>, csv::Error> {
    if input.is_empty() {
        return Err(csv::Error::from(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "empty input",
        )));
    }
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(input);
    rdr.deserialize::<TransactionEntry>().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "date;symbol;number;price;commission;currency\n";

    fn csv(rows: &str) -> Vec<u8> {
        format!("{HEADER}{rows}").into_bytes()
    }

    #[test]
    fn tc1_valid_multi_row_csv() {
        let input = csv("2024-01-15;AAPL;10;182.50;1.00;USD\n2024-01-16;MSFT;5;370.00;0.50;USD\n");
        let entries = parse(&input).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn tc2_valid_single_row_csv() {
        let input = csv("2024-01-15;AAPL;10;182.50;1.00;USD\n");
        let entries = parse(&input).unwrap();
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn tc3_header_only_returns_empty() {
        let input = HEADER.as_bytes().to_vec();
        let entries = parse(&input).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn tc4_comma_delimiter_returns_error() {
        let input = b"date,symbol,number,price,commission,currency\n2024-01-15,AAPL,10,182.50,1.00,USD\n".to_vec();
        assert!(parse(&input).is_err());
    }

    #[test]
    fn tc5_missing_column_returns_error() {
        let input = csv("2024-01-15;AAPL;10;182.50;1.00\n");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn tc6_invalid_decimal_in_price_returns_error() {
        let input = csv("2024-01-15;AAPL;10;not-a-number;1.00;USD\n");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn tc7_invalid_date_format_returns_error() {
        let input = csv("15-01-2024;AAPL;10;182.50;1.00;USD\n");
        assert!(parse(&input).is_err());
    }

    #[test]
    fn tc8_empty_input_returns_error() {
        assert!(parse(b"").is_err());
    }
}
