use crate::{
    action::types::trig::TrigActionType, error::RytmExternalError, util::get_bool_from_0_or_1,
};
use median::{atom::AtomValue, symbol::SymbolRef};
use rytm_rs::object::pattern::{track::trig::HoldsTrigFlags, Trig};
use std::convert::TryFrom;

pub struct TrigSetAction<'a> {
    pub trig: &'a mut Trig,
    pub action: SymbolRef,
    pub parameter: AtomValue,
}

pub fn handle_trig_set_action(action: TrigSetAction) -> Result<(), RytmExternalError> {
    let TrigSetAction {
        action,
        trig,
        parameter,
    } = action;

    match TrigActionType::try_from(action)? {
        TrigActionType::Enable => {
            trig.set_trig_enable(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Retrig => {
            trig.set_retrig(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Mute => {
            trig.set_mute(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Accent => {
            trig.set_accent(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Swing => {
            trig.set_swing(get_bool_from_0_or_1(parameter)?);
        }
        TrigActionType::Slide => {
            trig.set_slide(get_bool_from_0_or_1(parameter)?);
        }
    }

    Ok(())
}
