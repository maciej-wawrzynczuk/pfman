use std::collections::HashMap;

pub struct Portfolio {
    data: HashMap<String, i16>
}

impl Portfolio {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn add_transaction(&mut self, symbol: &str, amount: i16) {
        *self.data.entry(symbol.into())
            .or_insert(0) += amount;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single() {
        let mut sut = Portfolio::new();

        sut.add_transaction("FOO", 1);
        assert_eq!(*sut.data.get("FOO").unwrap(), 1);
    }
}