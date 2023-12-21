use crate::api::global_action_type::*;
use crate::api::global_enum_type::*;
use crate::error::EnumError::InvalidEnumType;
use crate::error::IdentifierError;
use crate::error::RytmExternalError;
use crate::util::get_bool_from_0_or_1;
use crate::util::only_allow_numbers_as_identifier_parameter;
use median::atom::{Atom, AtomValue};
use median::symbol::SymbolRef;
use rytm_rs::object::Global;
use std::convert::TryFrom;
use std::convert::TryInto;

pub fn handle_global_set_enum_value(
    global: &mut Global,
    enum_type: &str,
    enum_value: &str,
    maybe_parameter_atom: Option<&Atom>,
) -> Result<(), RytmExternalError> {
    match enum_type {
        METRONOME_TIME_SIGNATURE => global
            .metronome_settings_mut()
            .set_time_signature(enum_value.try_into()?),

        ROUTING_USB_IN_OPTIONS => global.routing_mut().set_usb_in(enum_value.try_into()?),
        ROUTING_USB_OUT_OPTIONS => global.routing_mut().set_usb_out(enum_value.try_into()?),
        ROUTING_USB_TO_MAIN_DB => global
            .routing_mut()
            .set_usb_to_main_db(enum_value.try_into()?),

        OUT_PORT_FUNCTION => global
            .midi_config_mut()
            .port_config_mut()
            .set_output_port_function(enum_value.try_into()?),
        THRU_PORT_FUNCTION => global
            .midi_config_mut()
            .port_config_mut()
            .set_thru_port_function(enum_value.try_into()?),
        INPUT_FROM => global
            .midi_config_mut()
            .port_config_mut()
            .set_input_transport(enum_value.try_into()?),
        OUTPUT_TO => global
            .midi_config_mut()
            .port_config_mut()
            .set_output_transport(enum_value.try_into()?),
        PARAM_OUTPUT => global
            .midi_config_mut()
            .port_config_mut()
            .set_parameter_output_type(enum_value.try_into()?),
        PAD_DEST => global
            .midi_config_mut()
            .port_config_mut()
            .set_pad_parameter_destination(enum_value.try_into()?),
        PRESSURE_DEST => global
            .midi_config_mut()
            .port_config_mut()
            .set_pressure_parameter_destination(enum_value.try_into()?),
        ENCODER_DEST => global
            .midi_config_mut()
            .port_config_mut()
            .set_encoder_parameter_destination(enum_value.try_into()?),
        MUTE_DEST => global
            .midi_config_mut()
            .port_config_mut()
            .set_mute_parameter_destination(enum_value.try_into()?),
        PORTS_OUTPUT_CHANNEL => global
            .midi_config_mut()
            .port_config_mut()
            .set_ports_output_channel(enum_value.try_into()?),

        AUTO_CHANNEL => global
            .midi_config_mut()
            .channels_mut()
            .set_auto_channel(enum_value.try_into()?)?,

        TRACK_CHANNELS => {
            if let Some(atom) = maybe_parameter_atom {
                if let Some(AtomValue::Int(track_index)) = atom.get_value() {
                    global
                        .midi_config_mut()
                        .channels_mut()
                        .set_track_channel(track_index as usize, enum_value.try_into()?)?;
                } else {
                    return Err("Invalid setter format: trackchannels should be followed by an integer track index. Format: trackchannels:<channel> <track index>. Example: trackchannels:1 2".into());
                }
            }
            return Err("Invalid setter format: trackchannels should be followed by an integer track index. Format: trackchannels:<channel> <track index>. Example: trackchannels:1 2".into());
        }
        TRACK_FX_CHANNEL => global
            .midi_config_mut()
            .channels_mut()
            .set_track_fx_channel(enum_value.try_into()?)?,
        PROGRAM_CHANGE_IN_CHANNEL => global
            .midi_config_mut()
            .channels_mut()
            .set_program_change_in_channel(enum_value.try_into()?)?,
        PROGRAM_CHANGE_OUT_CHANNEL => global
            .midi_config_mut()
            .channels_mut()
            .set_program_change_out_channel(enum_value.try_into()?)?,
        PERFORMANCE_CHANNEL => global
            .midi_config_mut()
            .channels_mut()
            .set_performance_channel(enum_value.try_into()?)?,

        other => return Err(InvalidEnumType(other.to_owned()).into()),
    }

    Ok(())
}

pub fn handle_global_set_action(
    global: &mut Global,
    action_or_enum_value_str: &str,
    parameter_atom: &Atom,
) -> Result<(), RytmExternalError> {
    let action_or_enum_value = SymbolRef::try_from(action_or_enum_value_str)?;
    let action_or_enum_value_str = action_or_enum_value.to_string()?;

    only_allow_numbers_as_identifier_parameter(parameter_atom)?;

    match action_or_enum_value_str.as_str() {
        KIT_RELOAD_ON_CHANGE => {
            global
                .sequencer_config_mut()
                .set_kit_reload_on_chg(get_bool_from_0_or_1(parameter_atom, KIT_RELOAD_ON_CHANGE)?);
            Ok(())
        }
        QUANTIZE_LIVE_REC => {
            global
                .sequencer_config_mut()
                .set_quantize_live_rec(get_bool_from_0_or_1(parameter_atom, QUANTIZE_LIVE_REC)?);
            Ok(())
        }
        AUTO_TRACK_SWITCH => {
            global
                .sequencer_config_mut()
                .set_auto_trk_switch(get_bool_from_0_or_1(parameter_atom, AUTO_TRACK_SWITCH)?);
            Ok(())
        }

        ROUTE_TO_MAIN => {
            global
                .routing_mut()
                .route_track_to_main(parameter_atom.get_int() as usize)?;
            Ok(())
        }
        SEND_TO_FX => {
            global
                .routing_mut()
                .send_track_to_fx(parameter_atom.get_int() as usize)?;
            Ok(())
        }

        CLOCK_RECEIVE => {
            global
                .midi_config_mut()
                .sync_mut()
                .set_clock_receive(get_bool_from_0_or_1(parameter_atom, CLOCK_RECEIVE)?);
            Ok(())
        }
        CLOCK_SEND => {
            global
                .midi_config_mut()
                .sync_mut()
                .set_clock_send(get_bool_from_0_or_1(parameter_atom, CLOCK_SEND)?);
            Ok(())
        }
        TRANSPORT_RECEIVE => {
            global
                .midi_config_mut()
                .sync_mut()
                .set_transport_receive(get_bool_from_0_or_1(parameter_atom, TRANSPORT_RECEIVE)?);
            Ok(())
        }
        TRANSPORT_SEND => {
            global
                .midi_config_mut()
                .sync_mut()
                .set_transport_send(get_bool_from_0_or_1(parameter_atom, TRANSPORT_SEND)?);
            Ok(())
        }
        PROGRAM_CHANGE_RECEIVE => {
            global
                .midi_config_mut()
                .sync_mut()
                .set_program_change_receive(get_bool_from_0_or_1(
                    parameter_atom,
                    PROGRAM_CHANGE_RECEIVE,
                )?);
            Ok(())
        }
        PROGRAM_CHANGE_SEND => {
            global
                .midi_config_mut()
                .sync_mut()
                .set_program_change_send(get_bool_from_0_or_1(
                    parameter_atom,
                    PROGRAM_CHANGE_SEND,
                )?);
            Ok(())
        }

        RECEIVE_NOTES => {
            global
                .midi_config_mut()
                .port_config_mut()
                .set_receive_notes(get_bool_from_0_or_1(parameter_atom, RECEIVE_NOTES)?);
            Ok(())
        }
        RECEIVE_CC_NRPN => {
            global
                .midi_config_mut()
                .port_config_mut()
                .set_receive_cc_nrpn(get_bool_from_0_or_1(parameter_atom, RECEIVE_CC_NRPN)?);
            Ok(())
        }

        _ => Err(IdentifierError::InvalidType(action_or_enum_value_str).into()),
    }
}
