use crate::error::RytmExternalError;
use median::{atom::AtomValue, symbol::SymbolRef};
use rytm_rs::object::pattern::track::Track;

pub struct TrackSetAction<'a> {
    pub track: &'a mut Track,
    pub action: SymbolRef,
    pub parameter: AtomValue,
}

pub fn handle_track_set_action(action: TrackSetAction) -> Result<(), RytmExternalError> {
    todo!()
}
