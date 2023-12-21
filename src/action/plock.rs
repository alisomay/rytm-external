use crate::{api::plock_type::*, error::RytmExternalError};
use median::atom::Atom;
use median::outlet::OutAnything;
use rytm_rs::object::pattern::Trig;

use self::clear::{handle_trig_plock_clear_action, handle_trig_plock_clear_enum_value};
use self::get::{handle_trig_plock_get_action, handle_trig_plock_get_enum_value};
use self::set::{handle_trig_plock_set_action, handle_trig_plock_set_enum_value};

pub mod clear;
pub mod get;
pub mod set;

pub fn handle_trig_plock_setter_action(
    trig: &Trig,
    selector: &str,
    atoms: &[Atom],
    slice_index: usize,
) -> Result<(), RytmExternalError> {
    // If so, the next atom must either be an action or an enum value.
    if let Some(atom) = atoms.get(slice_index) {
        let action_or_enum_value = atom.get_symbol();
        let action_or_enum_value_str = action_or_enum_value.to_string()?;

        let enum_pair = action_or_enum_value_str.split_once(':');

        return match selector {
            PLOCK_SET => {
                if let Some((enum_type, enum_value)) = enum_pair {
                    handle_trig_plock_set_enum_value(trig, enum_type, enum_value)
                } else {
                    handle_trig_plock_set_action(trig, &action_or_enum_value, atoms, slice_index + 1)
                }
            }
            PLOCK_GET => {
                Err(format!("Invalid plock action: ({action_or_enum_value_str}): You may use {PLOCK_SET} and {PLOCK_CLEAR} with setters and {PLOCK_GET} with getters.").into())
            }
            PLOCK_CLEAR => {
                if let Some((enum_type, _)) = enum_pair {
                    handle_trig_plock_clear_enum_value(trig, enum_type)
                } else {
                    handle_trig_plock_clear_action(trig, &action_or_enum_value)
                }
            }
            _ => unreachable!("Use this function only with plock selectors."),
        };
    }

    Err("Invalid plock setter format: The list must be followed by either an identifier or enum value.".into())
}

pub fn handle_trig_plock_getter_action(
    trig: &Trig,
    selector: &str,
    atoms: &[Atom],
    slice_index: usize,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    // If so, the next atom must either be an action or an enum value.
    if let Some(atom) = atoms.get(slice_index) {
        let action_or_enum_value = atom.get_symbol();
        let action_or_enum_value_str = action_or_enum_value.to_string()?;

        let enum_pair = action_or_enum_value_str.split_once(':');

        return match selector {
            PLOCK_SET | PLOCK_CLEAR => {
                Err(format!("Invalid plock action ({action_or_enum_value_str}): You may use {PLOCK_SET} and {PLOCK_CLEAR} with setters and {PLOCK_GET} with getters.").into())
            }
            PLOCK_GET => {
                if let Some((enum_type, _)) = enum_pair {
                    handle_trig_plock_get_enum_value(trig, enum_type, out)
                } else {
                    handle_trig_plock_get_action(trig, action_or_enum_value, out)
                }
            }

            _ => unreachable!("Use this function only with plock selectors."),
        };
    }

    Err("Invalid plock getter format: The list must be followed by either an identifier or enum value.".into())
}
