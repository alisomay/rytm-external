use crate::traits::*;
use median::{max_sys, object::MaxObj};
use rytm_rs::error::RytmError;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum QueryError {
    #[error("Invalid query selector: Query selector must be one of pattern, pattern_wb, kit, kit_wb, global, global_wb, sound, sound_wb or settings.")]
    InvalidSelector,
    #[error("Invalid query format: Query format is <selector> <index> for pattern, kit, global, sound and <selector> for pattern_wb, kit_wb, global_wb, sound_wb, settings types.")]
    InvalidFormat,
    #[error("Invalid index type: Index must be an integer.")]
    InvalidIndexType,
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
    RytmSdk(#[from] RytmError),
}

impl From<&str> for RytmExternalError {
    fn from(s: &str) -> Self {
        Self::Custom(s.to_string())
    }
}

impl RytmExternalError {
    pub fn obj_post(&self, obj: *mut max_sys::t_object) {
        match self {
            Self::Custom(err) => median::object::error(obj, err.to_string()),
            Self::Query(err) => median::object::error(obj, err.to_string()),
            Self::RytmSdk(err) => median::object::error(obj, err.to_string()),
        }
    }

    pub fn post(&self) {
        match self {
            Self::Custom(err) => median::error(err.to_string()),
            Self::Query(err) => median::error(err.to_string()),
            Self::RytmSdk(err) => median::error(err.to_string()),
        }
    }
}
