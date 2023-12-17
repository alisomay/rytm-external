use std::convert::TryFrom;

use median::{
    atom::{Atom, AtomValue},
    outlet::OutAnything,
    symbol::SymbolRef,
};
use rytm_rs::object::pattern::{track::trig::HoldsTrigFlags, Trig};

use crate::{action::types::trig::TrigActionType, error::RytmExternalError};

pub struct TrigGetAction<'a> {
    pub trig: &'a Trig,
    pub action: SymbolRef,
    pub out: &'a OutAnything,
}

pub fn handle_trig_get_action(action: TrigGetAction) -> Result<(), RytmExternalError> {
    let TrigGetAction { action, trig, out } = action;

    let value: isize = match TrigActionType::try_from(action.clone())? {
        TrigActionType::Enable => trig.enabled_trig().into(),
        TrigActionType::Retrig => trig.enabled_retrig().into(),
        TrigActionType::Mute => trig.enabled_mute().into(),
        TrigActionType::Accent => trig.enabled_accent().into(),
        TrigActionType::Swing => trig.enabled_swing().into(),
        TrigActionType::Slide => trig.enabled_slide().into(),
    };

    let action_atom = Atom::from(action);
    let value_atom = Atom::from(AtomValue::Int(value));

    if let Err(_stack_overflow_err) = out.send(&[action_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}
