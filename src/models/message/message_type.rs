use std::fmt;

/// Plain text or unencrypted message.
pub(crate) const PLAIN_MESSAGE: MessageType = MessageType::PlainMessageType; // 0

/// Secured text or encrypted message.
static ENCRYPTED_MESSAGE: MessageType = MessageType::SecureMessageType; // 1

/// MessageType:
/// The type of the message:
/// * 0 - Plain text or unencrypted message.
/// * 1 - Secured text or encrypted message.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageType {
    #[serde(rename = "0")]
    PlainMessageType = 0x00,
    #[serde(rename = "1")]
    SecureMessageType = 0x01,
}

impl MessageType {
    /// Returns a 'MessageType' for the given int value.
    ///
    /// Throws an error when the type is unknown.
    pub fn get_type(value: u8) -> crate::Result<Self> {
        match value {
            0x00 => Ok(MessageType::PlainMessageType),
            0x01 => Ok(MessageType::SecureMessageType),
            _ => bail!("unknown message type")
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::PlainMessageType => write!(f, "PlainMessageType"),
            MessageType::SecureMessageType => write!(f, "SecureMessageType"),
        }
    }
}
