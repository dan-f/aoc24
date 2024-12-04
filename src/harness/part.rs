use thiserror::Error;

/// Daily exercise component
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Part {
    One,
    Two,
}

impl Part {
    pub fn num(&self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
        }
    }
}

impl TryFrom<u8> for Part {
    type Error = PartParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(PartParseError(value)),
        }
    }
}

#[derive(Debug, Error)]
#[error("expected 1 or 2, but got: {0}")]
pub struct PartParseError(u8);
