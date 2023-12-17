use crate::error::RytmExternalError;
use median::{outlet::OutAnything, symbol::SymbolRef};
use rytm_rs::object::pattern::track::Track;

pub struct TrackGetAction<'a> {
    pub track: &'a Track,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
}
pub fn handle_track_get_action(action: TrackGetAction) -> Result<(), RytmExternalError> {
    todo!()
}
