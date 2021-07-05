/**
A lossless representation of currency.

This type encodes the currency using its smallest possible unit. This is a better approach
than floating point numbers where imprecision can change the results of calculations.
*/
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    USD(USD),
}

impl Currency {
    pub fn usd(cents: u64) -> Self {
        Currency::USD(USD::new(cents))
    }
}

/**
A currency value in USD.

The value is encoded as whole cents.
*/
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct USD {
    cents: u64,
}

impl USD {
    pub fn new(cents: u64) -> Self {
        USD { cents }
    }
}
