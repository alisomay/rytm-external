pub mod pattern;
pub mod pattern_wb;

use crate::{
    action::{
        get::{
            handle_get_action,
            pattern::{handle_pattern_enum_get_action, PatternGetAction},
            track::{handle_track_enum_get_action, TrackGetAction},
            trig::{handle_trig_enum_get_action, TrigGetAction},
            GetAction,
        },
        set::{
            handle_set_action,
            pattern::{handle_pattern_enum_set_action, PatternSetAction},
            track::{handle_track_enum_set_action, TrackSetAction},
            trig::{handle_trig_enum_set_action, TrigSetAction},
            SetAction,
        },
    },
    error::RytmExternalError,
    traits::*,
    util::try_get_action_value_from_atom_slice,
};
use lazy_static::lazy_static;
use median::{
    atom::Atom,
    max_sys::t_atom_long,
    object::MaxObj,
    outlet::{OutAnything, OutInt},
    symbol::SymbolRef,
};
use rytm_rs::object::Pattern;
use std::convert::TryFrom;

/*** Object Types ***/

pub mod object_type {
    use super::*;

    lazy_static! {
        pub static ref PATTERN: SymbolRef = SymbolRef::try_from("pattern").unwrap();
        pub static ref PATTERN_WORK_BUFFER: SymbolRef = SymbolRef::try_from("pattern_wb").unwrap();
        pub static ref KIT: SymbolRef = SymbolRef::try_from("kit").unwrap();
        pub static ref KIT_WORK_BUFFER: SymbolRef = SymbolRef::try_from("kit_wb").unwrap();
        pub static ref SOUND: SymbolRef = SymbolRef::try_from("sound").unwrap();
        pub static ref SOUND_WORK_BUFFER: SymbolRef = SymbolRef::try_from("sound_wb").unwrap();
        pub static ref GLOBAL: SymbolRef = SymbolRef::try_from("global").unwrap();
        pub static ref GLOBAL_WORK_BUFFER: SymbolRef = SymbolRef::try_from("global_wb").unwrap();
        pub static ref SETTINGS: SymbolRef = SymbolRef::try_from("settings").unwrap();
    }
}

pub mod object_sub_type {
    use super::*;

    lazy_static! {
        pub static ref TRACK: SymbolRef = SymbolRef::try_from("track").unwrap();
        pub static ref TRIG: SymbolRef = SymbolRef::try_from("trig").unwrap();
    }
}

/*** Action Types ***/

pub mod trig_action_type {

    pub const ENABLE: &str = "enable";
    pub const RETRIG: &str = "retrig";
    pub const MUTE: &str = "mute";
    pub const ACCENT: &str = "accent";
    pub const SWING: &str = "swing";
    pub const SLIDE: &str = "slide";

    // TODO: I need to understand how these behave first.
    // pub const PARAMETER_LOCK_LFO_SWITCH: &str = "parameterlocklfoswitch";
    // pub const PARAMETER_LOCK_LFO: &str = "parameterlocklfo";
    // pub const PARAMETER_LOCK_SYNTH_SWITCH: &str = "parameterlocksynthswitch";
    // pub const PARAMETER_LOCK_SYNTH: &str = "parameterlocksynth";
    // pub const PARAMETER_LOCK_SAMPLE_SWITCH: &str = "parameterlocksampleswitch";
    // pub const PARAMETER_LOCK_SAMPLE: &str = "parameterlocksample";
    // pub const PARAMETER_LOCK_ENV_SWITCH: &str = "parameterlockenvswitch";
    // pub const PARAMETER_LOCK_ENV: &str = "parameterlockenv";

    pub const NOTE: &str = "note";
    pub const VELOCITY: &str = "velocity";
    pub const RETRIG_VELOCITY_OFFSET: &str = "retrigvelocityoffset";
    pub const SOUND_LOCK: &str = "soundlock";
}

pub mod track_action_type {
    pub const IS_WORK_BUFFER: &str = "iswb";
    pub const OWNER_INDEX: &str = "ownerindex";
    pub const INDEX: &str = "index";
    pub const DEF_TRIG_NOTE: &str = "deftrignote";
    pub const DEF_TRIG_VELOCITY: &str = "deftrigvel";
    pub const DEF_TRIG_PROB: &str = "deftrigprob";
    pub const NUMBER_OF_STEPS: &str = "steps";
    pub const QUANTIZE_AMOUNT: &str = "quantizeamount";
    pub const SENDS_MIDI: &str = "sendsmidi";
    pub const EUCLIDEAN_MODE: &str = "euc";
    pub const EUCLIDEAN_PL1: &str = "pl1";
    pub const EUCLIDEAN_PL2: &str = "pl2";
    pub const EUCLIDEAN_RO1: &str = "ro1";
    pub const EUCLIDEAN_RO2: &str = "ro2";
    pub const EUCLIDEAN_TRO: &str = "tro";
}

pub mod pattern_action_type {
    pub const IS_WORK_BUFFER: &str = "iswb";
    pub const INDEX: &str = "index";
    pub const VERSION: &str = "version";
    pub const MASTER_LENGTH: &str = "masterlength";
    pub const MASTER_CHANGE: &str = "masterchange";
    pub const KIT_NUMBER: &str = "kitnumber";
    pub const SWING_AMOUNT: &str = "swingamount";
    pub const GLOBAL_QUANTIZE: &str = "globalquantize";
    pub const BPM: &str = "bpm";
}

/*** Enum Types ***/

pub mod pattern_enum_type {
    pub const SPEED: &str = "speed";
    pub const TIME_MODE: &str = "timemode";
}

pub mod track_enum_type {
    pub const ROOT_NOTE: &str = "rootnote";
    pub const PAD_SCALE: &str = "padscale";
    pub const DEFAULT_NOTE_LENGTH: &str = "defaultnotelength";
}

pub mod trig_enum_type {
    pub const MICRO_TIME: &str = "microtime";
    pub const NOTE_LENGTH: &str = "notelength";
    pub const RETRIG_LENGTH: &str = "retriglength";
    pub const RETRIG_RATE: &str = "retrigrate";
    pub const TRIG_CONDITION: &str = "trigcondition";
}

pub mod kit_enum_type {
    pub const CONTROL_IN_MOD_TARGET: &str = "controlinmodtarget";
    pub const FX_LFO_DESTINATION: &str = "fxlfodestination";
    pub const FX_DELAY_TIME_ON_THE_GRID: &str = "fxdelaytimeonthegrid";
    pub const FX_COMP_ATTACK: &str = "fxcompattack";
    pub const FX_COMP_RELEASE: &str = "fxcomprelease";
    pub const FX_COMP_RATIO: &str = "fxcompratio";
    pub const FX_COMP_SIDE_CHAIN_EQ: &str = "fxcompsidechaineq";
}

pub mod settings_enum_type {
    pub const PARAMETER_MENU_ITEM: &str = "parametermenuitem";
    pub const FX_PARAMETER_MENU_ITEM: &str = "fxparametermenuitem";
    pub const SEQUENCER_MODE: &str = "sequencermode";
    pub const PATTERN_MODE: &str = "patternmode";
    pub const SAMPLE_RECORDER_SOURCE: &str = "samplerecordersource";
    pub const SAMPLE_RECORDER_RECORDING_LENGTH: &str = "samplerecorderrecordinglength";
}

pub mod sound_enum_type {
    pub const MACHINE_PARAMETERS: &str = "machineparameters";
    pub const MACHINE_TYPE: &str = "machinetype";
    pub const LO_DESTINATION: &str = "lodestination";
    pub const SOUND_MOD_TARGET: &str = "soundmodtarget";
    pub const FILTER_TYPE: &str = "filtertype";
    pub const LFO_MULTIPLIER: &str = "lfomultiplier";
    pub const LFO_WAVEFORM: &str = "lfowaveform";
    pub const LFO_MODE: &str = "lfomode";
    pub const SOUND_SETTINGS_CHROMATIC_MODE: &str = "chromaticmode";
}

pub mod sound_machine_enum_type {
    pub const BD_ACOUSTIC_WAVEFORM: &str = "bdacousticwaveform";
    pub const BD_SHARP_WAVEFORM: &str = "bdsharpwaveform";
    pub const SY_CHIP_WAVEFORM: &str = "sychipwaveform";
    pub const SY_CHIP_SPEED: &str = "sychipspeed";
    pub const SY_RAW_WAVEFORM_1: &str = "syrawwaveform1";
    pub const SY_RAW_WAVEFORM_2: &str = "syrawwaveform2";
}

pub mod global_enum_type {
    pub const TIME_SIGNATURE: &str = "timesignature";
    pub const MIDI_CHANNEL: &str = "midichannel";
    pub const MIDI_PORT_FUNCTION: &str = "midiportfunction";
    pub const MIDI_TRANSPORT_LAYER: &str = "miditransportlayer";
    pub const PARAMETER_DESTINATION: &str = "parameterdestination";
    pub const ROUTING_USB_TO_MAIN_DB: &str = "routingusbtomaindb";
    pub const MIDI_PARAMETER_OUTPUT: &str = "midiparameteroutput";
    pub const MIDI_PORTS_OUTPUT_CHANNEL: &str = "midiportsoutputchannel";
    pub const HARDWARE_TRACK: &str = "hardwaretrack";
    pub const ROUTING_USB_IN_OPTIONS: &str = "routingusbinoptions";
    pub const ROUTING_USB_OUT_OPTIONS: &str = "routingusboutoptions";
}

pub fn pattern_get(
    action_or_enum_type: SymbolRef,
    pattern: &Pattern,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, _)) = action_or_enum_type.to_string()?.split_once(':') {
        handle_pattern_enum_get_action(pattern, enum_type, out)
    } else {
        handle_get_action(GetAction::Pattern(PatternGetAction {
            pattern,
            action: action_or_enum_type,
            out,
        }))
    }
}

pub fn track_get(
    action_or_enum_type: SymbolRef,
    track: &rytm_rs::object::pattern::track::Track,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, _)) = action_or_enum_type.to_string()?.split_once(':') {
        handle_track_enum_get_action(track, enum_type, out)
    } else {
        handle_get_action(GetAction::Track(TrackGetAction {
            track,
            action: action_or_enum_type,
            out,
        }))
    }
}

pub fn trig_get(
    action_or_enum_type: SymbolRef,
    trig: &rytm_rs::object::pattern::track::trig::Trig,
    out: &OutAnything,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, _)) = action_or_enum_type.to_string()?.split_once(':') {
        handle_trig_enum_get_action(trig, enum_type, out)
    } else {
        handle_get_action(GetAction::Trig(TrigGetAction {
            trig,
            action: action_or_enum_type,
            out,
        }))
    }
}

pub fn pattern_set(
    action_or_enum_value: SymbolRef,
    pattern: &mut Pattern,
    atoms: &[Atom],
    select: usize,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, enum_variant)) = action_or_enum_value.to_string()?.split_once(':') {
        handle_pattern_enum_set_action(pattern, enum_type, enum_variant)
    } else {
        handle_set_action(SetAction::Pattern(PatternSetAction {
            pattern,
            action: action_or_enum_value,
            parameter: try_get_action_value_from_atom_slice(select, atoms)?,
        }))
    }
}

pub fn track_set(
    action_or_enum_value: SymbolRef,
    track: &mut rytm_rs::object::pattern::track::Track,
    atoms: &[Atom],
    select: usize,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, enum_variant)) = action_or_enum_value.to_string()?.split_once(':') {
        handle_track_enum_set_action(track, enum_type, enum_variant)
    } else {
        handle_set_action(SetAction::Track(TrackSetAction {
            track,
            action: action_or_enum_value,
            parameter: try_get_action_value_from_atom_slice(select, atoms)?,
        }))
    }
}

pub fn trig_set(
    action_or_enum_value: SymbolRef,
    trig: &mut rytm_rs::object::pattern::track::trig::Trig,
    atoms: &[Atom],
    select: usize,
) -> Result<(), RytmExternalError> {
    if let Some((enum_type, enum_variant)) = action_or_enum_value.to_string()?.split_once(':') {
        handle_trig_enum_set_action(trig, enum_type, enum_variant)
    } else {
        handle_set_action(SetAction::Trig(TrigSetAction {
            trig,
            action: action_or_enum_value,
            parameter: try_get_action_value_from_atom_slice(select, atoms)?,
        }))
    }
}
