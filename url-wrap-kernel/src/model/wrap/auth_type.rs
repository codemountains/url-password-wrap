use anyhow::anyhow;
use std::fmt;
use std::fmt::Formatter;

#[derive(Eq, PartialEq)]
pub enum WrapAuthType {
    Text,
    FourDigit,
}

impl WrapAuthType {
    pub fn id(&self) -> u32 {
        match self {
            WrapAuthType::Text => 1,
            WrapAuthType::FourDigit => 2,
        }
    }
}

impl From<String> for WrapAuthType {
    fn from(type_name: String) -> Self {
        match &*type_name {
            "Text" => WrapAuthType::Text,
            "FourDigit" => WrapAuthType::FourDigit,
            _ => WrapAuthType::Text,
        }
    }
}

impl TryFrom<u32> for WrapAuthType {
    type Error = anyhow::Error;

    fn try_from(type_id: u32) -> Result<Self, Self::Error> {
        match type_id {
            1 => Ok(WrapAuthType::Text),
            2 => Ok(WrapAuthType::FourDigit),
            _ => Err(anyhow!("WrapAuthTypeId is invalid value.")),
        }
    }
}

impl fmt::Display for WrapAuthType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            WrapAuthType::Text => write!(f, "Text"),
            WrapAuthType::FourDigit => write!(f, "FourDigit"),
        }
    }
}
