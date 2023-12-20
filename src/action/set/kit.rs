use crate::api::kit_action_type::*;
use crate::api::kit_element_type::*;
use crate::api::kit_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use crate::error::RytmExternalError::NotYetImplemented;
use crate::util::get_bool_from_0_or_1;
use crate::util::only_allow_numbers_as_action_parameter;
use median::atom::AtomType;
use median::atom::{Atom, AtomValue};
use median::symbol::SymbolRef;
use rytm_rs::object::Kit;
use rytm_rs::object::Sound;
use std::convert::TryInto;

pub fn handle_kit_set_action(
    kit: &mut Kit,
    action: SymbolRef,
    parameter: &Atom,
) -> Result<(), RytmExternalError> {
    let action_str = action.to_string()?;
    if action_str.as_str() == NAME {
        match parameter.get_value().unwrap() {
            AtomValue::Symbol(symbol) => return Ok(kit.set_name(symbol.to_string()?.as_str())?),
            _ => return Err("Invalid value: Name must be a symbol with maximum 15 characters long and use only ascii characters.".into()),
        }
    }

    only_allow_numbers_as_action_parameter(parameter)?;

    match action_str.as_str() {
        FX_DELAY_TIME => Ok(kit.fx_delay_mut().set_time(parameter.get_int() as usize)?),
        FX_DELAY_PING_PONG => {
            kit.fx_delay_mut()
                .set_ping_pong(get_bool_from_0_or_1(parameter)?);
            Ok(())
        }
        FX_DELAY_STEREO_WIDTH => Ok(kit.fx_delay_mut().set_stereo_width(parameter.get_int())?),
        FX_DELAY_FEEDBACK => Ok(kit
            .fx_delay_mut()
            .set_feedback(parameter.get_int() as usize)?),
        FX_DELAY_HPF => Ok(kit.fx_delay_mut().set_hpf(parameter.get_int() as usize)?),
        FX_DELAY_LPF => Ok(kit.fx_delay_mut().set_lpf(parameter.get_int() as usize)?),
        FX_DELAY_REVERB_SEND => Ok(kit
            .fx_delay_mut()
            .set_reverb_send(parameter.get_int() as usize)?),
        FX_DELAY_VOLUME => Ok(kit
            .fx_delay_mut()
            .set_volume(parameter.get_int() as usize)?),

        FX_REVERB_PRE_DELAY => Ok(kit
            .fx_reverb_mut()
            .set_pre_delay(parameter.get_int() as usize)?),
        FX_REVERB_DECAY => Ok(kit
            .fx_reverb_mut()
            .set_decay(parameter.get_int() as usize)?),
        FX_REVERB_FREQ => Ok(kit.fx_reverb_mut().set_freq(parameter.get_int() as usize)?),
        FX_REVERB_GAIN => Ok(kit.fx_reverb_mut().set_gain(parameter.get_int() as usize)?),
        FX_REVERB_HPF => Ok(kit.fx_reverb_mut().set_hpf(parameter.get_int() as usize)?),
        FX_REVERB_LPF => Ok(kit.fx_reverb_mut().set_lpf(parameter.get_int() as usize)?),
        FX_REVERB_VOLUME => Ok(kit
            .fx_reverb_mut()
            .set_volume(parameter.get_int() as usize)?),

        FX_COMP_THRESHOLD => Ok(kit
            .fx_compressor_mut()
            .set_threshold(parameter.get_int() as usize)?),
        FX_COMP_GAIN => Ok(kit
            .fx_compressor_mut()
            .set_gain(parameter.get_int() as usize)?),
        FX_COMP_MIX => Ok(kit
            .fx_compressor_mut()
            .set_mix(parameter.get_int() as usize)?),
        FX_COMP_VOLUME => Ok(kit
            .fx_compressor_mut()
            .set_volume(parameter.get_int() as usize)?),

        FX_LFO_SPEED => Ok(kit.fx_lfo_mut().set_speed(parameter.get_int())?),
        FX_LFO_FADE => Ok(kit.fx_lfo_mut().set_fade(parameter.get_int())?),
        FX_LFO_START_PHASE_OR_SLEW => Ok(kit
            .fx_lfo_mut()
            .set_start_phase(parameter.get_int() as usize)?),
        FX_LFO_DEPTH => Ok(kit.fx_lfo_mut().set_depth(parameter.get_float() as f32)?),

        other => Err(InvalidActionType(other.to_string()).into()),
    }
}

pub fn handle_kit_set_enum_value(
    kit: &mut Kit,
    enum_type: &str,
    enum_value: &str,
) -> Result<(), RytmExternalError> {
    match enum_type {
        CONTROL_IN_MOD_TARGET => return Err(NotYetImplemented),
        FX_LFO_DESTINATION => kit.fx_lfo_mut().set_destination(enum_value.try_into()?),
        FX_DELAY_TIME_ON_THE_GRID => kit.fx_delay_mut().set_time_on_grid(enum_value.try_into()?),
        FX_COMP_ATTACK => kit.fx_compressor_mut().set_attack(enum_value.try_into()?),
        FX_COMP_RELEASE => kit.fx_compressor_mut().set_release(enum_value.try_into()?),
        FX_COMP_RATIO => kit.fx_compressor_mut().set_ratio(enum_value.try_into()?),
        FX_COMP_SIDE_CHAIN_EQ => kit
            .fx_compressor_mut()
            .set_side_chain_eq(enum_value.try_into()?),

        other => return Err(InvalidEnumType(other.to_string()).into()),
    };

    Ok(())
}

pub fn handle_kit_set_kit_element(
    kit: &mut Kit,
    element_type: &str,
    element_index: usize,
    element_parameter: &Atom,
) -> Result<(), RytmExternalError> {
    match element_parameter.get_type() {
        Some(AtomType::Object) | None => {
            return Err(RytmExternalError::from(
                "Kit element parameters can be only integers, floats or symbols.",
            ))
        }
        _ => {
            // Pass..
        }
    };

    match element_type {
        TRACK_LEVEL => {
            Ok(kit.set_track_level(element_index, element_parameter.get_int() as usize)?)
        }
        TRACK_RETRIG_RATE => {
            let param_str = element_parameter.get_symbol().to_string()?;
            let (_, enum_value) = param_str.as_str().split_once(':').ok_or_else(|| {
                RytmExternalError::from("Invalid value: kit element requires an enum value.")
            })?;
            kit.track_retrig_settings_mut(element_index)?
                .set_rate(enum_value.try_into()?);
            Ok(())
        }
        TRACK_RETRIG_LENGTH => {
            let param_str = element_parameter.get_symbol().to_string()?;
            let (_, enum_value) = param_str.as_str().split_once(':').ok_or_else(|| {
                RytmExternalError::from("Invalid value: kit element requires an enum value.")
            })?;
            kit.track_retrig_settings_mut(element_index)?
                .set_length(enum_value.try_into()?);
            Ok(())
        }
        TRACK_RETRIG_VEL_OFFSET => Ok(kit
            .track_retrig_settings_mut(element_index)?
            .set_velocity_curve(element_parameter.get_int())?),
        TRACK_RETRIG_ALWAYS_ON => {
            kit.track_retrig_settings_mut(element_index)?
                .set_always_on(get_bool_from_0_or_1(element_parameter)?);
            Ok(())
        }

        other => Err(InvalidActionType(other.to_string()).into()),
    }
}

pub fn handle_kit_set_kit_sound(
    kit: &mut Sound,
    atoms: &[Atom],
    slice_from_index: usize,
) -> Result<(), RytmExternalError> {
    todo!()
}
