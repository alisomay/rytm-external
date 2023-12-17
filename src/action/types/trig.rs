use crate::const_sym::trig_action_type::*;
use crate::error::RytmExternalError;
use median::symbol::SymbolRef;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub enum TrigActionType {
    Enable,
    Retrig,
    Mute,
    Accent,
    Swing,
    Slide,
}

impl TryFrom<SymbolRef> for TrigActionType {
    type Error = RytmExternalError;

    fn try_from(sym: SymbolRef) -> Result<Self, Self::Error> {
        if sym == *ENABLE {
            Ok(Self::Enable)
        } else if sym == *RETRIG {
            Ok(Self::Retrig)
        } else if sym == *MUTE {
            Ok(Self::Mute)
        } else if sym == *ACCENT {
            Ok(Self::Accent)
        } else if sym == *SWING {
            Ok(Self::Swing)
        } else if sym == *SLIDE {
            Ok(Self::Slide)
        } else {
            Err("Invalid trig action type".into())
        }
    }
}
