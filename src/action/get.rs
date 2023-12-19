use crate::error::RytmExternalError;

use self::{
    pattern::{handle_pattern_get_action, PatternGetAction},
    track::{handle_track_get_action, TrackGetAction},
    trig::{handle_trig_get_action, TrigGetAction},
};

pub mod kit;
pub mod pattern;
pub mod track;
pub mod trig;

pub enum GetAction<'a> {
    Pattern(PatternGetAction<'a>),
    Track(TrackGetAction<'a>),
    Trig(TrigGetAction<'a>),
}

pub fn handle_get_action(action: GetAction) -> Result<(), RytmExternalError> {
    match action {
        GetAction::Pattern(action) => handle_pattern_get_action(action),
        GetAction::Track(action) => handle_track_get_action(action),
        GetAction::Trig(action) => handle_trig_get_action(action),
    }
}
