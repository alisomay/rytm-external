use crate::api::settings_action_type::*;
use crate::api::settings_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use median::atom::Atom;
use median::outlet::OutAnything;
use median::symbol::SymbolRef;
use rytm_rs::object::Settings;
use std::convert::TryFrom;
use std::ffi::CString;

pub fn handle_settings_get_enum_value(
    settings: &Settings,
    enum_type: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        PARAMETER_MENU_ITEM => settings.selected_parameter_menu_item().into(),
        FX_PARAMETER_MENU_ITEM => settings.selected_fx_menu_item().into(),
        SEQUENCER_MODE => settings.selected_mode().into(),
        PATTERN_MODE => settings.selected_pattern_mode().into(),
        SAMPLE_RECORDER_SOURCE => settings.sample_recorder_source().into(),
        SAMPLE_RECORDER_RECORDING_LENGTH => settings.sample_recorder_recording_length().into(),
        other => return Err(InvalidEnumType(other.to_string()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) =
        out.send(&[enum_type_atom, Atom::from(0isize), enum_value_atom][..])
    {
        // Stack overflow ignore
    }

    Ok(())
}

// pub fn handle_settings_set_action(
//     settings: &mut Settings,
//     action_or_enum_value_str: &str,
//     parameter_atom: &Atom,
// ) -> Result<(), RytmExternalError> {
//     let action_or_enum_value = SymbolRef::try_from(action_or_enum_value_str)?;
//     let action_or_enum_value_str = action_or_enum_value.to_string()?;

//     only_allow_numbers_as_action_parameter(parameter_atom)?;

//     match action_or_enum_value_str.as_str() {
//         BPM_PROJECT => {
//             settings.set_bpm(parameter_atom.get_float() as f32)?;
//             Ok(())
//         }
//         SELECTED_TRACK => {
//             settings.set_selected_track(parameter_atom.get_int() as usize)?;
//             Ok(())
//         }
//         SELECTED_PAGE => {
//             settings.set_selected_page(parameter_atom.get_int() as usize)?;
//             Ok(())
//         }
//         MUTE => {
//             settings.mute_sound(parameter_atom.get_int() as usize)?;
//             Ok(())
//         }
//         FIXED_VELOCITY_ENABLE => {
//             settings.set_fixed_velocity_enable(get_bool_from_0_or_1(parameter_atom)?);
//             Ok(())
//         }
//         FIXED_VELOCITY_AMOUNT => {
//             settings.set_fixed_velocity_amount(parameter_atom.get_int() as usize)?;
//             Ok(())
//         }
//         SAMPLE_RECORDER_THR => {
//             settings.set_sample_recorder_threshold(parameter_atom.get_int() as usize)?;
//             Ok(())
//         }
//         SAMPLE_RECORDER_MONITOR_ENABLE => {
//             settings.set_sample_recorder_monitor_enable(get_bool_from_0_or_1(parameter_atom)?);
//             Ok(())
//         }

//         other => Err(InvalidActionType(other.to_string()).into()),
//     }
// }

pub fn handle_settings_get_action(
    settings: &Settings,
    action: &str,
    maybe_next_atom: Option<&Atom>,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value_atom: Atom = match action {
        BPM_PROJECT => Atom::from(settings.bpm() as f64),
        SELECTED_TRACK => Atom::from(settings.selected_track() as isize),
        SELECTED_PAGE => Atom::from(settings.selected_page() as isize),
        MUTE => {
            let index = maybe_next_atom
                .ok_or("Invalid format: mute should be followed by an integer sound index.")?
                .get_int();
            Atom::from(settings.is_sound_muted(index as usize)? as isize)
        }
        FIXED_VELOCITY_ENABLE => Atom::from(settings.fixed_velocity_enabled() as isize),
        FIXED_VELOCITY_AMOUNT => Atom::from(settings.fixed_velocity_amount() as isize),
        SAMPLE_RECORDER_THR => Atom::from(settings.sample_recorder_threshold() as isize),
        SAMPLE_RECORDER_MONITOR_ENABLE => {
            Atom::from(settings.sample_recorder_monitor_enabled() as isize)
        }

        other => return Err(InvalidActionType(other.to_string()).into()),
    };

    let action_atom = Atom::from(SymbolRef::from(CString::new(action).unwrap()));
    if let Err(_stack_overflow_err) = out.send(&[action_atom, Atom::from(0isize), value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}
