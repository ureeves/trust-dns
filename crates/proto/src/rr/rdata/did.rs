
//! DID Records for decentralized identities.

use crate::error::*;
use crate::serialize::binary::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DIDDocument {
    record: Vec<u8>,
}

impl DIDDocument {
    /// Creates a new DIDDocument. The record will NOT be validated.
    pub fn new(record: Vec<u8>) -> DIDDocument {
        DIDDocument { record }
    }

    /// Byte content of the record. This should be a valid DID document but it is not guaranteed. 
    pub fn record(&self) -> &[u8] {
        &self.record
    }
}

/// Read the DIDDocument from the given Decoder
pub fn read(decoder: &mut BinDecoder, rdata_length: Restrict<u16>) -> ProtoResult<DIDDocument> {
    let rdata_length = rdata_length.map(usize::from).unverified();
    let record =
        decoder.read_vec(rdata_length)?.unverified(/*we do not enforce a specific format*/);
    Ok(DIDDocument::new(record))
}

/// Write the RData using the given encoder
pub fn emit(encoder: &mut BinEncoder, did_document: &DIDDocument) -> ProtoResult<()> {
    encoder.emit_vec(did_document.record())
}
