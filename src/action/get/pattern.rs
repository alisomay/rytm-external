use crate::error::RytmExternalError;
use median::{outlet::OutAnything, symbol::SymbolRef};
use rytm_rs::object::Pattern;

pub struct PatternGetAction<'a> {
    pub pattern: &'a Pattern,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
}

pub fn handle_pattern_get_action(action: PatternGetAction) -> Result<(), RytmExternalError> {
    todo!()
}
