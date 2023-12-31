use crate::api::kit_action_type::*;
use crate::api::kit_element_type::*;
use crate::api::kit_enum_type::*;
use crate::api::sound_kit::handle_sound_kit_set;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use crate::util::get_bool_from_0_or_1;
use crate::util::only_allow_numbers_as_identifier_parameter;
use median::atom::AtomType;
use median::atom::{Atom, AtomValue};
use median::symbol::SymbolRef;
use rytm_rs::object::Kit;
use rytm_rs::object::Sound;
use std::convert::TryInto;

pub fn handle_kit_set_action(
    kit: &mut Kit,
    action: &SymbolRef,
    parameter: &Atom,
    maybe_next_atom: Option<&Atom>,
) -> Result<(), RytmExternalError> {
    let action_str = action.to_string()?;
    if action_str.as_str() == NAME {
        match parameter.get_value().unwrap() {
            AtomValue::Symbol(symbol) => return Ok(kit.set_name(symbol.to_string()?.as_str())?),
            _ => return Err("Invalid parameter: name must be a symbol with maximum 15 characters long and use only ascii characters.".into()),
        }
    }

    only_allow_numbers_as_identifier_parameter(parameter)?;

    match action_str.as_str() {
        CONTROL_IN_1_MOD_AMT => {
            if let Some(next_atom) = maybe_next_atom {
                if let Some(AtomValue::Int(value)) = next_atom.get_value() {
                    // TODO: Check if this creates a bug
                    match parameter.get_int() {
                        0 => kit.set_control_in_1_mod_amt_1(value)?,
                        1 => kit.set_control_in_1_mod_amt_2(value)?,
                        2 => kit.set_control_in_1_mod_amt_3(value)?,
                        3 => kit.set_control_in_1_mod_amt_4(value)?,
                        other => {
                            return Err(format!(
                            "Invalid range: The index {other} is out of range for ctrlin1modamt."
                        )
                            .into())
                        }
                    }
                    return Ok(());
                }
                return Err("Invalid setter format: ctrlin1modamt should be followed by an integer ctrlin1mod index. Format: ctrlin1modamt <ctrlin1mod index> <amount>. Example: ctrlin1modamt 2 100".into());
            }
            Err("Invalid setter format: ctrlin1modamt should be followed by an integer ctrlin1mod index. Format: ctrlin1modamt <ctrlin1mod index> <amount>. Example: ctrlin1modamt 2 100".into())
        }

        CONTROL_IN_2_MOD_AMT => {
            if let Some(next_atom) = maybe_next_atom {
                if let Some(AtomValue::Int(value)) = next_atom.get_value() {
                    // TODO: Check if this creates a bug
                    match parameter.get_int() {
                        0 => kit.set_control_in_2_mod_amt_1(value)?,
                        1 => kit.set_control_in_2_mod_amt_2(value)?,
                        2 => kit.set_control_in_2_mod_amt_3(value)?,
                        3 => kit.set_control_in_2_mod_amt_4(value)?,
                        other => {
                            return Err(format!(
                            "Invalid range: The index {other} is out of range for ctrlin2modamt."
                        )
                            .into())
                        }
                    }
                    return Ok(());
                }
                return Err("Invalid setter format: ctrlin2modamt should be followed by an integer ctrlin2mod index. Format: ctrlin2modamt <ctrlin2mod index> <amount>. Example: ctrlin2modamt 2 100".into());
            }
            Err("Invalid setter format: ctrlin2modamt should be followed by an integer ctrlin2mod index. Format: ctrlin2modamt <ctrlin2mod index> <amount>. Example: ctrlin2modamt 2 100".into())
        }

        FX_DELAY_TIME => Ok(kit.fx_delay_mut().set_time(parameter.get_int() as usize)?),
        FX_DELAY_PING_PONG => {
            kit.fx_delay_mut()
                .set_ping_pong(get_bool_from_0_or_1(parameter, FX_DELAY_PING_PONG)?);
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

        FX_DISTORTION_DELAY_OVERDRIVE => Ok(kit
            .fx_distortion_mut()
            .set_delay_overdrive(parameter.get_int() as usize)?),
        FX_DISTORTION_DELAY_POST => {
            kit.fx_distortion_mut()
                .set_delay_post(get_bool_from_0_or_1(parameter, FX_DISTORTION_DELAY_POST)?);
            Ok(())
        }
        FX_DISTORTION_REVERB_POST => {
            kit.fx_distortion_mut()
                .set_reverb_post(get_bool_from_0_or_1(parameter, FX_DISTORTION_REVERB_POST)?);
            Ok(())
        }
        FX_DISTORTION_AMOUNT => Ok(kit
            .fx_distortion_mut()
            .set_amount(parameter.get_int() as usize)?),
        FX_DISTORTION_SYMMETRY => Ok(kit.fx_distortion_mut().set_symmetry(parameter.get_int())?),

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}

pub fn handle_kit_set_enum_value(
    kit: &mut Kit,
    enum_type: &str,
    enum_value: &str,
    maybe_parameter_atom: Option<&Atom>,
) -> Result<(), RytmExternalError> {
    match enum_type {
        CONTROL_IN_1_MOD_TARGET => {
            if let Some(atom) = maybe_parameter_atom {
                if let Some(AtomValue::Int(index)) = atom.get_value() {
                    match index {
                        0 => kit.set_control_in_1_mod_target_1(enum_value.try_into()?),
                        1 => kit.set_control_in_1_mod_target_2(enum_value.try_into()?),
                        2 => kit.set_control_in_1_mod_target_3(enum_value.try_into()?),
                        3 => kit.set_control_in_1_mod_target_4(enum_value.try_into()?),
                        other => {
                            return Err(format!(
                                "Invalid range: The index {other} is out of range for {CONTROL_IN_1_MOD_TARGET}."
                            )
                            .into())
                        }
                    }
                    return Ok(());
                }
                return Err("Invalid setter format: ctrlinmod1target should be followed by an integer ctrlinmod1 index. Format: ctrlinmod1target:<target> <ctrlinmod1 index>. Example: ctrlinmod1target:lfophase 2".into());
            }
            return Err("Invalid setter format: ctrlinmod1target should be followed by an integer ctrlinmod1 index. Format: ctrlinmod1target:<target> <ctrlinmod1 index>. Example: ctrlinmod1target:lfophase 2".into());
        }
        CONTROL_IN_2_MOD_TARGET => {
            if let Some(atom) = maybe_parameter_atom {
                if let Some(AtomValue::Int(index)) = atom.get_value() {
                    match index {
                        0 => kit.set_control_in_2_mod_target_1(enum_value.try_into()?),
                        1 => kit.set_control_in_2_mod_target_2(enum_value.try_into()?),
                        2 => kit.set_control_in_2_mod_target_3(enum_value.try_into()?),
                        3 => kit.set_control_in_2_mod_target_4(enum_value.try_into()?),
                        other => {
                            return Err(format!(
                                "Invalid range: The index {other} is out of range for {CONTROL_IN_2_MOD_TARGET}."
                            )
                            .into())
                        }
                    }
                    return Ok(());
                }
                return Err("Invalid setter format: ctrlinmod2target should be followed by an integer ctrlinmod2 index. Format: ctrlinmod2target:<target> <ctrlinmod2 index>. Example: ctrlinmod2target:lfophase 2".into());
            }
            return Err("Invalid setter format: ctrlinmod2target should be followed by an integer ctrlinmod2 index. Format: ctrlinmod2target:<target> <ctrlinmod2 index>. Example: ctrlinmod2target:lfophase 2".into());
        }
        FX_LFO_DESTINATION => kit.fx_lfo_mut().set_destination(enum_value.try_into()?),
        FX_DELAY_TIME_ON_THE_GRID => kit.fx_delay_mut().set_time_on_grid(enum_value.try_into()?),
        FX_COMP_ATTACK => kit.fx_compressor_mut().set_attack(enum_value.try_into()?),
        FX_COMP_RELEASE => kit.fx_compressor_mut().set_release(enum_value.try_into()?),
        FX_COMP_RATIO => kit.fx_compressor_mut().set_ratio(enum_value.try_into()?),
        FX_COMP_SIDE_CHAIN_EQ => kit
            .fx_compressor_mut()
            .set_side_chain_eq(enum_value.try_into()?),

        other => return Err(InvalidEnumType(other.to_owned()).into()),
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
        Some(AtomType::Object) | None => return Err(RytmExternalError::from(
            "Invalid parameter: Kit element parameters can be only integers, floats or symbols.",
        )),
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
                RytmExternalError::from(
                    "Invalid parameter: kit element should be followed by an enum value.",
                )
            })?;
            kit.track_retrig_settings_mut(element_index)?
                .set_rate(enum_value.try_into()?);
            Ok(())
        }
        TRACK_RETRIG_LENGTH => {
            let param_str = element_parameter.get_symbol().to_string()?;
            let (_, enum_value) = param_str.as_str().split_once(':').ok_or_else(|| {
                RytmExternalError::from(
                    "Invalid parameter: kit element should be followed by an enum value.",
                )
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
                .set_always_on(get_bool_from_0_or_1(
                    element_parameter,
                    TRACK_RETRIG_ALWAYS_ON,
                )?);
            Ok(())
        }

        other => Err(IdentifierError::InvalidType(other.to_owned()).into()),
    }
}

pub fn handle_kit_set_kit_sound(
    sound: &mut Sound,
    atoms: &[Atom],
    slice_from_index: usize,
) -> Result<(), RytmExternalError> {
    handle_sound_kit_set(sound, &atoms[slice_from_index..])
}
