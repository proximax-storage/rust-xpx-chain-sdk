use std::fmt;

use crate::models::message::MessageType;

pub trait Message: Sync + erased_serde::Serialize
    where
        Self: fmt::Debug,
{
    fn message_type(&self) -> &MessageType;

    fn payload_to_bytes(&self) -> &[u8];
}

serialize_trait_object!(Message);

impl fmt::Display for dyn Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Algo: {}", serde_json::to_string_pretty(&self).unwrap_or_default())
    }
}
