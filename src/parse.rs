use std::str::FromStr;

use crate::harness::InputError;

pub fn parse_u8(s: &str) -> Result<u8, InputError> {
    u8::from_str(s).map_err(|error| InputError::InvalidInput {
        msg: format!("Failed to parse {}", s),
        source: Some(Box::new(error)),
    })
}

pub fn parse_u32(s: &str) -> Result<u32, InputError> {
    u32::from_str(s).map_err(|error| InputError::InvalidInput {
        msg: format!("Failed to parse {}", s),
        source: Some(Box::new(error)),
    })
}

pub fn parse_u64(s: &str) -> Result<u64, InputError> {
    u64::from_str(s).map_err(|error| InputError::InvalidInput {
        msg: format!("Failed to parse {}", s),
        source: Some(Box::new(error)),
    })
}
