pub trait Currency: Copy {
    fn decimal_precision(&self) -> u32 {
        2
    }

    fn iso_code(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
}

#[derive(Copy, Clone, Debug)]
pub struct USD {}

impl Currency for USD {
    fn iso_code(&self) -> &'static str {
        "USD"
    }

    fn symbol(&self) -> &'static str {
        "$"
    }
}
