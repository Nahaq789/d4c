use core::convert::TryFrom;
use std::str::Bytes;

#[derive(Debug, Clone)]
pub struct HeaderKey<'a> {
    key: Bytes<'a>
}

#[derive(Debug, Clone)]
pub struct HeaderValue<'a> {
    value: Bytes<'a>
}

impl TryFrom<&str> for HeaderKey {
    type Error = dyn std::error::Error;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}