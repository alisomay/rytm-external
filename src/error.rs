use crate::traits::*;
use median::{max_sys, object::MaxObj};
use rytm_rs::error::RytmError;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum QueryError {
    #[error("Invalid query selector: Query selector must be one of pattern, pattern_wb, kit, kit_wb, global, global_wb, sound, sound_wb or settings.")]
    InvalidSelector,
    #[error("Invalid query format: Query format is <selector> <index> for pattern, kit, global, sound, sound_wb and <selector> for pattern_wb, kit_wb, global_wb, settings types.")]
    InvalidFormat,
    #[error("Invalid index type: Index must be an integer.")]
    InvalidIndexType,
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
    Enum(#[from] EnumError),
    #[error("{0}")]
    Action(#[from] ActionError),
    #[error("{0}")]
    RytmSdk(#[from] RytmError),
    #[error("{0}")]
    StringConversionError(#[from] std::str::Utf8Error),
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
            Self::Enum(err) => median::object::error(obj, err.to_string()),
            Self::Action(err) => median::object::error(obj, err.to_string()),
            Self::RytmSdk(err) => median::object::error(obj, err.to_string()),
            Self::StringConversionError(err) => median::object::error(obj, err.to_string()),
        }
    }

    pub fn post(&self) {
        match self {
            Self::Custom(err) => median::error(err.to_string()),
            Self::Query(err) => median::error(err.to_string()),
            Self::Enum(err) => median::error(err.to_string()),
            Self::Action(err) => median::error(err.to_string()),
            Self::RytmSdk(err) => median::error(err.to_string()),
            Self::StringConversionError(err) => median::error(err.to_string()),
        }
    }
}
