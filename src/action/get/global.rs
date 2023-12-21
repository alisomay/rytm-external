use crate::api::global_action_type::*;
use crate::api::global_enum_type::*;
use crate::error::ActionError::InvalidActionType;
use crate::error::EnumError::InvalidEnumType;
use crate::error::RytmExternalError;
use median::atom::{Atom, AtomValue};
use median::outlet::OutAnything;
use median::symbol::SymbolRef;
use rytm_rs::object::Global;
use std::convert::TryFrom;
use std::ffi::CString;

pub fn handle_global_get_enum_value(
    global: &Global,
    enum_type: &str,
    enum_value: &str,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: &str = match enum_type {
        METRONOME_TIME_SIGNATURE => global.metronome_settings().time_signature().into(),

        ROUTING_USB_IN_OPTIONS => global.routing().usb_in().into(),
        ROUTING_USB_OUT_OPTIONS => global.routing().usb_out().into(),
        ROUTING_USB_TO_MAIN_DB => global.routing().usb_to_main_db().into(),

        OUT_PORT_FUNCTION => global
            .midi_config()
            .port_config()
            .output_port_function()
            .into(),
        THRU_PORT_FUNCTION => global
            .midi_config()
            .port_config()
            .thru_port_function()
            .into(),
        INPUT_FROM => global.midi_config().port_config().input_transport().into(),
        OUTPUT_TO => global.midi_config().port_config().output_transport().into(),
        PARAM_OUTPUT => global
            .midi_config()
            .port_config()
            .parameter_output_type()
            .into(),
        PAD_DEST => global
            .midi_config()
            .port_config()
            .pad_parameter_destination()
            .into(),
        PRESSURE_DEST => global
            .midi_config()
            .port_config()
            .pressure_parameter_destination()
            .into(),
        ENCODER_DEST => global
            .midi_config()
            .port_config()
            .encoder_parameter_destination()
            .into(),
        MUTE_DEST => global
            .midi_config()
            .port_config()
            .mute_parameter_destination()
            .into(),
        PORTS_OUTPUT_CHANNEL => global
            .midi_config()
            .port_config()
            .ports_output_channel()
            .into(),

        AUTO_CHANNEL => global.midi_config().channels().auto_channel().into(),

        TRACK_CHANNELS => {
            let track_index = enum_value.parse::<usize>().map_err(|_| "Invalid getter format: trackchannels: should include an integer track index. Example: trackchannels:1")?;
            global
                .midi_config()
                .channels()
                .track_channel(track_index)?
                .into()
        }
        TRACK_FX_CHANNEL => global.midi_config().channels().track_fx_channel().into(),
        PROGRAM_CHANGE_IN_CHANNEL => global
            .midi_config()
            .channels()
            .program_change_in_channel()
            .into(),
        PROGRAM_CHANGE_OUT_CHANNEL => global
            .midi_config()
            .channels()
            .program_change_out_channel()
            .into(),
        PERFORMANCE_CHANNEL => global.midi_config().channels().performance_channel().into(),

        other => return Err(InvalidEnumType(other.to_string()).into()),
    };

    let enum_type_atom = Atom::from(SymbolRef::try_from(enum_type).unwrap());
    let index_atom = Atom::from(AtomValue::Int(global.index() as isize));
    let enum_value_atom = Atom::from(SymbolRef::try_from(value).unwrap());

    if let Err(_stack_overflow_err) = out.send(&[enum_type_atom, index_atom, enum_value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}

pub fn handle_global_get_action(
    global: &Global,
    action: &str,
    maybe_next_atom: Option<&Atom>,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    let value: isize = match action {
        VERSION => global.structure_version() as isize,
        INDEX => global.index() as isize,
        IS_WORK_BUFFER => global.is_work_buffer() as isize,

        KIT_RELOAD_ON_CHANGE => global.sequencer_config().kit_reload_on_chg().into(),
        QUANTIZE_LIVE_REC => global.sequencer_config().quantize_live_rec().into(),
        AUTO_TRACK_SWITCH => global.sequencer_config().auto_trk_switch().into(),

        ROUTE_TO_MAIN => {
            if let Some(atom) = maybe_next_atom {
                if let Some(AtomValue::Int(value)) = atom.get_value() {
                    global
                        .routing()
                        .is_track_routed_to_main(value as usize)
                        .into()
                } else {
                    return Err(
                        "Invalid getter format: routetomain requires to be followed by an integer track index."
                            .into(),
                    );
                }
            } else {
                return Err(
                    "Invalid getter format: routetomain requires to be followed by an integer track index.".into(),
                );
            }
        }

        SEND_TO_FX => {
            if let Some(atom) = maybe_next_atom {
                if let Some(AtomValue::Int(value)) = atom.get_value() {
                    global.routing().is_track_sent_to_fx(value as usize).into()
                } else {
                    return Err(
                        "Invalid format: sendtofx requires to be followed by a track index (integer)"
                            .into(),
                    );
                }
            } else {
                return Err(
                    "Invalid format: sendtofx requires to be followed by a track index (integer)"
                        .into(),
                );
            }
        }

        CLOCK_RECEIVE => global.midi_config().sync().clock_receive().into(),
        CLOCK_SEND => global.midi_config().sync().clock_send().into(),
        TRANSPORT_RECEIVE => global.midi_config().sync().transport_receive().into(),
        TRANSPORT_SEND => global.midi_config().sync().transport_send().into(),
        PROGRAM_CHANGE_RECEIVE => global.midi_config().sync().program_change_receive().into(),
        PROGRAM_CHANGE_SEND => global.midi_config().sync().program_change_send().into(),

        RECEIVE_NOTES => global.midi_config().port_config().receive_notes().into(),
        RECEIVE_CC_NRPN => global.midi_config().port_config().receive_cc_nrpn().into(),
        TURBO_SPEED => global.midi_config().port_config().turbo_speed().into(),

        METRONOME_ACTIVE => global.metronome_settings().is_active().into(),
        METRONOME_PRE_ROLL_BARS => global.metronome_settings().pre_roll_bars() as isize,
        METRONOME_VOLUME => global.metronome_settings().volume() as isize,

        other => return Err(InvalidActionType(other.to_string()).into()),
    };

    let action_atom = Atom::from(SymbolRef::from(CString::new(action).unwrap()));
    let index_atom = Atom::from(AtomValue::Int(global.index() as isize));
    let value_atom = Atom::from(value);

    if let Err(_stack_overflow_err) = out.send(&[action_atom, index_atom, value_atom][..]) {
        // Stack overflow ignore
    }

    Ok(())
}
