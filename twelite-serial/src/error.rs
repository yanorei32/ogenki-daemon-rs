use core::fmt;

/// A decode error structure.
#[derive(Debug, Eq, PartialEq)]
pub enum DecodeError {
    InvalidLength(usize),
    InvalidCharacter(u8),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(len) =>
                write!(f, "Unexpected length: {len}"),
            Self::InvalidCharacter(c) => 
                write!(f, "Hit to invalid character {c} when decode"),
        }
    }
}

/// A validate error structure.
/// This is only emmits when explicit calls validate functions.
#[derive(Debug, Eq, PartialEq)]
pub enum ValidateError {
    /// See [`crate::StatusNotify::validate_checksum`]
    InvalidChecksum(u8),

    /// See [`crate::StatusNotify::validate_command`]
    InvalidCommand(u8),

    /// See [`crate::StatusNotify::validate_protocol_version`]
    InvalidProtocolVersion(u8),

    /// See [`crate::StatusNotify::validate_relay_count`]
    InvalidRelayCount(u8),
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChecksum(checksum) =>
                write!(f, "Checksum is must be 0, actually {checksum}"),
            Self::InvalidCommand(c) => 
                write!(f, "Command is always 0x81, but actually {c}"),
            Self::InvalidProtocolVersion(c) => 
                write!(f, "Command is always 0x81, but actually {c}"),
            Self::InvalidRelayCount(count) => 
                write!(f, "Relay count is must be less or equal to 3, but actually {count}"),
        }
    }
}
