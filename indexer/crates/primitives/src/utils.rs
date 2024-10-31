use alloy::{
    dyn_abi::{DecodedEvent, DynSolEvent, DynSolType},
    primitives::{Bytes, LogData, B256},
};

/// This function is used to decode an event
/// params:
/// topics: Vec<B256> - The topics of the event
/// data: Bytes - The data of the event
/// decoder_format: DynSolType - The format of the event; example -> DynSolType::Tuple(
///     vec![
///         DynSolType::Uint(256)
///     ]
///  ),
/// indexed: Vec<DynSolType> - The indexed values of the event; example -> vec![DynSolType::Address]
pub fn decode_event(
    topics: Vec<B256>,
    data: Bytes,
    decoder_format: DynSolType,
    indexed: Vec<DynSolType>,
) -> Result<DecodedEvent, anyhow::Error> {
    let event: DynSolEvent = DynSolEvent::new_unchecked(Some(topics[0]), indexed, decoder_format);
    let log_data = LogData::new_unchecked(topics, data);
    let decoded_event = event.decode_log_data(&log_data, true)?;

    Ok(decoded_event)
}
