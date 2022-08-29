#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Currency {
    decimal_precision: u32,
    iso_code: &'static str,
    symbol: &'static str,
}

impl Currency {
    pub fn decimal_precision(&self) -> u32 {
        self.decimal_precision
    }

    pub fn iso_code(&self) -> &'static str {
        self.iso_code
    }
    pub fn symbol(&self) -> &'static str {
        self.symbol
    }
}

pub const USD: Currency = Currency {
    decimal_precision: 2,
    iso_code: "USD",
    symbol: "$",
};

// impl Currency for USD {
//     fn iso_code() -> &'static str {
//        "USD"
//    }

//    fn symbol() -> &'static str {
//        "$"
//    }
// }
