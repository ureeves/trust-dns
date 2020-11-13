
//! DID documents for decentralized identity. 

use crate::error::*;
use crate::rr::rdata::DIDDocument;

/// Parse the RData from a set of Tokens
pub fn parse<'i, I: Iterator<Item = &'i str>>(tokens: I) -> ParseResult<DIDDocument> {
    let mut record = Vec::new();
    
    for token in tokens {
        record.extend(token.as_bytes());
    }

    Ok(DIDDocument::new(record))
}
