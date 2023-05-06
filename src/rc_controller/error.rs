use std::{error::Error, fmt::Display};
#[derive(Debug)]
#[allow(unused)]
pub enum ControllerError {
    NoSuchChannel(usize),
    NotInitiallized,
    LeverNotAsigned(String),
    #[cfg(feature = "hidapi")]
    HidError(hidapi::HidError),
}

impl Error for ControllerError {}

impl Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchChannel(id) => write!(f, "Channel {} does not exist.", id),
            Self::NotInitiallized => write!(f, "Not initiallized."),
            Self::LeverNotAsigned(s) => write!(f, "Lever {} not assigned", s),
            #[cfg(feature = "hidapi")]
            Self::HidError(hiderror) => write!(f, "hid error:{}", hiderror),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_no_such_channel() {
        let no_such_channel = ControllerError::NoSuchChannel(42);
        let no_such_channel_display = format!("{}", no_such_channel);
        let no_such_channel_debug = format!("{:?}", no_such_channel);
        assert_eq!("Channel 42 does not exist.", no_such_channel_display);
        assert_eq!("NoSuchChannel(42)", no_such_channel_debug)
    }
}

#[cfg(feature = "hidapi")]
impl From<hidapi::HidError> for ControllerError {
    fn from(value: hidapi::HidError) -> Self {
        Self::HidError(value)
    }
}
