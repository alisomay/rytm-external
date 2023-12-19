use crate::error::RytmExternalError;

use self::{
    pattern::{handle_pattern_set_action, PatternSetAction},
    track::{handle_track_set_action, TrackSetAction},
    trig::{handle_trig_set_action, TrigSetAction},
};

pub mod kit;
pub mod pattern;
pub mod track;
pub mod trig;

pub enum SetAction<'a> {
    Pattern(PatternSetAction<'a>),
    Track(TrackSetAction<'a>),
    Trig(TrigSetAction<'a>),
}

pub fn handle_set_action(action: SetAction) -> Result<(), RytmExternalError> {
    match action {
        SetAction::Pattern(action) => handle_pattern_set_action(action),
        SetAction::Track(action) => handle_track_set_action(action),
        SetAction::Trig(action) => handle_trig_set_action(action),
    }
}
