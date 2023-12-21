use median::max_sys;
use rytm_rs::error::RytmError;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum QueryError {
    #[error("Invalid query selector: Query selector must be one of pattern, pattern_wb, kit, kit_wb, global, global_wb, sound, sound_wb or settings.")]
    InvalidSelector,
    #[error("Invalid query format: Query format is <selector> [<index>]. Example: query pattern_wb or query pattern 0")]
    InvalidFormat,
    #[error("Invalid index type: Index must be an integer.")]
    InvalidIndexType,
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum SendError {
    #[error("Invalid send format: Send format is send <selector> [<index>]. Example: send pattern_wb or send pattern 0.")]
    InvalidFormat,
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum GetError {
    #[error("Invalid getter format: {0}")]
    InvalidFormat(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum SetError {
    #[error("Invalid setter format: {0}")]
    InvalidFormat(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum EnumError {
    #[error("Invalid enum type: {0}")]
    InvalidEnumType(String),
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ActionError {
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),
    #[error("Invalid action parameter: {0}")]
    InvalidActionParameter(String),
}

/// Wrapper error type for all rytm errors.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RytmExternalError {
    #[error("{0}")]
    Custom(String),
    #[error("{0}")]
    Query(#[from] QueryError),
    #[error("{0}")]
    Send(#[from] SendError),
    #[error("{0}")]
    Get(#[from] GetError),
    #[error("{0}")]
    Set(#[from] SetError),
    #[error("{0}")]
    Enum(#[from] EnumError),
    #[error("{0}")]
    Action(#[from] ActionError),
    #[error("{0}")]
    RytmSdk(#[from] RytmError),
    #[error("{0}")]
    StringConversionError(#[from] std::str::Utf8Error),

    #[error("Not implemented, if you need it open an issue in https://github.com/alisomay/rytm-external.")]
    NotYetImplemented,
}

impl From<rytm_rs::error::ConversionError> for RytmExternalError {
    fn from(err: rytm_rs::error::ConversionError) -> Self {
        Self::RytmSdk(err.into())
    }
}

impl From<&str> for RytmExternalError {
    fn from(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

impl From<String> for RytmExternalError {
    fn from(s: String) -> Self {
        Self::Custom(s)
    }
}

impl RytmExternalError {
    pub fn obj_post(&self, obj: *mut max_sys::t_object) {
        match self {
            Self::Custom(err) => median::object::error(obj, err.to_string()),
            Self::Query(err) => median::object::error(obj, err.to_string()),
            Self::Send(err) => median::object::error(obj, err.to_string()),
            Self::Get(err) => median::object::error(obj, err.to_string()),
            Self::Set(err) => median::object::error(obj, err.to_string()),
            Self::Enum(err) => median::object::error(obj, err.to_string()),
            Self::Action(err) => median::object::error(obj, err.to_string()),
            Self::RytmSdk(err) => median::object::error(obj, err.to_string()),
            Self::StringConversionError(err) => median::object::error(obj, err.to_string()),
            Self::NotYetImplemented => {
                median::object::error(obj, "Not yet implemented.".to_string());
            }
        }
    }

    pub fn post(&self) {
        match self {
            Self::Custom(err) => median::error(err.to_string()),
            Self::Query(err) => median::error(err.to_string()),
            Self::Send(err) => median::error(err.to_string()),
            Self::Get(err) => median::error(err.to_string()),
            Self::Set(err) => median::error(err.to_string()),
            Self::Enum(err) => median::error(err.to_string()),
            Self::Action(err) => median::error(err.to_string()),
            Self::RytmSdk(err) => median::error(err.to_string()),
            Self::StringConversionError(err) => median::error(err.to_string()),
            Self::NotYetImplemented => median::error("Not yet implemented.".to_string()),
        }
    }
}
