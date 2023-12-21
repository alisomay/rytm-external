use crate::api::settings_action_type::*;
use crate::api::settings_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use crate::util::get_bool_from_0_or_1;
use crate::util::only_allow_numbers_as_identifier_parameter;
use median::atom::Atom;
use median::symbol::SymbolRef;
use rytm_rs::object::Settings;
use std::convert::TryFrom;
use std::convert::TryInto;

pub fn handle_settings_set_enum_value(
    settings: &mut Settings,
    enum_type: &str,
    enum_value: &str,
) -> Result<(), RytmExternalError> {
    match enum_type {
        PARAMETER_MENU_ITEM => {
            settings.set_selected_parameter_menu_item(enum_value.try_into()?);
            Ok(())
        }
        FX_PARAMETER_MENU_ITEM => {
            settings.set_selected_fx_menu_item(enum_value.try_into()?);
            Ok(())
        }
        SEQUENCER_MODE => {
            settings.set_selected_mode(enum_value.try_into()?);
            Ok(())
        }
        PATTERN_MODE => {
            settings.set_selected_pattern_mode(enum_value.try_into()?);
            Ok(())
        }
        SAMPLE_RECORDER_SOURCE => {
            settings.set_sample_recorder_source(enum_value.try_into()?);
            Ok(())
        }
        SAMPLE_RECORDER_RECORDING_LENGTH => {
            settings.set_sample_recorder_recording_length(enum_value.try_into()?);
            Ok(())
        }
        other => Err(InvalidEnumType(other.to_owned()).into()),
    }
}

pub fn handle_settings_set_action(
    settings: &mut Settings,
    action_or_enum_value_str: &str,
    parameter_atom: &Atom,
) -> Result<(), RytmExternalError> {
    let action_or_enum_value = SymbolRef::try_from(action_or_enum_value_str)?;
    let action_or_enum_value_str = action_or_enum_value.to_string()?;

    only_allow_numbers_as_identifier_parameter(parameter_atom)?;

    match action_or_enum_value_str.as_str() {
        BPM_PROJECT => {
            settings.set_bpm(parameter_atom.get_float() as f32)?;
            Ok(())
        }
        SELECTED_TRACK => {
            settings.set_selected_track(parameter_atom.get_int() as usize)?;
            Ok(())
        }
        SELECTED_PAGE => {
            settings.set_selected_page(parameter_atom.get_int() as usize)?;
            Ok(())
        }
        MUTE => {
            settings.mute_sound(parameter_atom.get_int() as usize)?;
            Ok(())
        }
        FIXED_VELOCITY_ENABLE => {
            settings.set_fixed_velocity_enable(get_bool_from_0_or_1(
                parameter_atom,
                FIXED_VELOCITY_ENABLE,
            )?);
            Ok(())
        }
        FIXED_VELOCITY_AMOUNT => {
            settings.set_fixed_velocity_amount(parameter_atom.get_int() as usize)?;
            Ok(())
        }
        SAMPLE_RECORDER_THR => {
            settings.set_sample_recorder_threshold(parameter_atom.get_int() as usize)?;
            Ok(())
        }
        SAMPLE_RECORDER_MONITOR_ENABLE => {
            settings.set_sample_recorder_monitor_enable(get_bool_from_0_or_1(
                parameter_atom,
                SAMPLE_RECORDER_MONITOR_ENABLE,
            )?);
            Ok(())
        }

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}
