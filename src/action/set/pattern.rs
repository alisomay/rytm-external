use median::{atom::AtomValue, symbol::SymbolRef};
use rytm_rs::object::Pattern;

use crate::error::RytmExternalError;

pub struct PatternSetAction<'a> {
    pub pattern: &'a mut Pattern,
    pub action: SymbolRef,
    pub parameter: AtomValue,
}

pub fn handle_pattern_set_action(action: PatternSetAction) -> Result<(), RytmExternalError> {
    todo!()
}
