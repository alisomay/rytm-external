pub mod global;
pub mod global_wb;
pub mod kit;
pub mod kit_wb;
pub mod pattern;
pub mod pattern_wb;
pub mod settings;
pub mod sound;
pub mod sound_kit;
pub mod sound_wb;

use lazy_static::lazy_static;
use median::symbol::SymbolRef;
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

/*** Object Element Types ***/

pub mod kit_element_type {
    pub const TRACK_LEVEL: &str = "tracklevel";
    pub const TRACK_RETRIG_RATE: &str = "trackretrigrate";
    pub const TRACK_RETRIG_LENGTH: &str = "trackretriglength";
    pub const TRACK_RETRIG_VEL_OFFSET: &str = "trackretrigveloffset";
    pub const TRACK_RETRIG_ALWAYS_ON: &str = "trackretrigalwayson";
    pub const SOUND: &str = "sound";

    pub const KIT_ELEMENTS: &[&str] = &[
        TRACK_LEVEL,
        TRACK_RETRIG_RATE,
        TRACK_RETRIG_LENGTH,
        TRACK_RETRIG_VEL_OFFSET,
        TRACK_RETRIG_ALWAYS_ON,
        SOUND,
    ];
}

pub mod plock_type {
    pub const PLOCK_GET: &str = "plockget";
    pub const PLOCK_SET: &str = "plockset";
    pub const PLOCK_CLEAR: &str = "plockclear";

    pub const ALL_PLOCK_TYPES: &[&str] = &[PLOCK_GET, PLOCK_SET, PLOCK_CLEAR];
}

// TODO:
pub mod machine_parameter_type {
    // TODO: For the first version we'll omit machine parameters.
}

/*** Action Types ***/

pub mod settings_action_type {
    pub const VERSION: &str = "version";
    pub const BPM_PROJECT: &str = "projectbpm";
    pub const SELECTED_TRACK: &str = "selectedtrack";
    pub const SELECTED_PAGE: &str = "selectedpage";
    pub const MUTE: &str = "mute";
    pub const FIXED_VELOCITY_ENABLE: &str = "fixedvelocity";
    pub const FIXED_VELOCITY_AMOUNT: &str = "fixedvelocityamt";
    pub const SAMPLE_RECORDER_THR: &str = "samplerecorderthr";
    pub const SAMPLE_RECORDER_MONITOR_ENABLE: &str = "samplerecordermonitor";
}

pub mod global_action_type {
    pub const VERSION: &str = "version";
    pub const INDEX: &str = "index";
    pub const IS_WORK_BUFFER: &str = "iswb";

    pub const KIT_RELOAD_ON_CHANGE: &str = "kitreloadonchg";
    pub const QUANTIZE_LIVE_REC: &str = "quantizeliverec";
    pub const AUTO_TRACK_SWITCH: &str = "autotrackswitch";

    pub const ROUTE_TO_MAIN: &str = "routetomain";
    pub const SEND_TO_FX: &str = "sendtofx";

    pub const CLOCK_RECEIVE: &str = "clockreceive";
    pub const CLOCK_SEND: &str = "clocksend";
    pub const TRANSPORT_RECEIVE: &str = "transportreceive";
    pub const TRANSPORT_SEND: &str = "transportsend";
    pub const PROGRAM_CHANGE_RECEIVE: &str = "programchangereceive";
    pub const PROGRAM_CHANGE_SEND: &str = "programchangesend";

    pub const RECEIVE_NOTES: &str = "receivenotes";
    pub const RECEIVE_CC_NRPN: &str = "receiveccnrpn";
    // Only get will be implemented for this one
    pub const TURBO_SPEED: &str = "turbospeed";

    pub const METRONOME_ACTIVE: &str = "metronomeactive";
    pub const METRONOME_PRE_ROLL_BARS: &str = "metronomeprerollbars";
    pub const METRONOME_VOLUME: &str = "metronomevolume";
}

pub mod kit_action_type {
    pub const VERSION: &str = "version";
    pub const INDEX: &str = "index";
    pub const NAME: &str = "name";

    pub const FX_DELAY_TIME: &str = "fxdelaytime";
    pub const FX_DELAY_PING_PONG: &str = "fxdelaypingpong";
    pub const FX_DELAY_STEREO_WIDTH: &str = "fxdelaystereowidth";
    pub const FX_DELAY_FEEDBACK: &str = "fxdelayfeedback";
    pub const FX_DELAY_HPF: &str = "fxdelayhpf";
    pub const FX_DELAY_LPF: &str = "fxdelaylpf";
    pub const FX_DELAY_REVERB_SEND: &str = "fxdelayreverbsend";
    pub const FX_DELAY_VOLUME: &str = "fxdelayvolume";

    pub const FX_REVERB_PRE_DELAY: &str = "fxreverbpredelay";
    pub const FX_REVERB_DECAY: &str = "fxreverbdecay";
    pub const FX_REVERB_FREQ: &str = "fxreverbfreq";
    pub const FX_REVERB_GAIN: &str = "fxreverbgain";
    pub const FX_REVERB_HPF: &str = "fxreverbhpf";
    pub const FX_REVERB_LPF: &str = "fxreverblpf";
    pub const FX_REVERB_VOLUME: &str = "fxreverbvolume";

    pub const FX_COMP_THRESHOLD: &str = "fxcompthreshold";
    pub const FX_COMP_GAIN: &str = "fxcompgain";
    pub const FX_COMP_MIX: &str = "fxcompmix";
    pub const FX_COMP_VOLUME: &str = "fxcompvolume";

    pub const FX_LFO_SPEED: &str = "fxlfospeed";
    pub const FX_LFO_FADE: &str = "fxlfofade";
    pub const FX_LFO_START_PHASE_OR_SLEW: &str = "fxlfostartphase";
    pub const FX_LFO_DEPTH: &str = "fxlfodepth";

    // TODO: Enable and revise after fixing the dist in the SDK
    // pub const FX_DISTORTION_REVERB_SEND: &str = "fxdistortionreverbsend";
    // pub const FX_DISTORTION_DELAY_OVERDRIVE: &str = "fxdistortiondelayoverdrive";
    // pub const FX_DISTORTION_REVERB_POST: &str = "fxdistortionreverbpost";
    // pub const FX_DISTORTION_AMOUNT: &str = "fxdistortionamount";
    // pub const FX_DISTORTION_SYMMETRY: &str = "fxdistortionsymmetry";

    pub const KIT_ACTION_TYPES: &[&str] = &[
        VERSION,
        INDEX,
        NAME,
        FX_DELAY_TIME,
        FX_DELAY_PING_PONG,
        FX_DELAY_STEREO_WIDTH,
        FX_DELAY_FEEDBACK,
        FX_DELAY_HPF,
        FX_DELAY_LPF,
        FX_DELAY_REVERB_SEND,
        FX_DELAY_VOLUME,
        FX_REVERB_PRE_DELAY,
        FX_REVERB_DECAY,
        FX_REVERB_FREQ,
        FX_REVERB_GAIN,
        FX_REVERB_HPF,
        FX_REVERB_LPF,
        FX_REVERB_VOLUME,
        FX_COMP_THRESHOLD,
        FX_COMP_GAIN,
        FX_COMP_MIX,
        FX_COMP_VOLUME,
        FX_LFO_SPEED,
        FX_LFO_FADE,
        FX_LFO_START_PHASE_OR_SLEW,
        FX_LFO_DEPTH,
    ];
}

pub mod trig_action_type {

    pub const ENABLE: &str = "enable";
    pub const RETRIG: &str = "retrig";
    pub const MUTE: &str = "mute";
    pub const ACCENT: &str = "accent";
    pub const SWING: &str = "swing";
    pub const SLIDE: &str = "slide";

    // TODO: I need to understand how these behave first.
    // Also maybe we expose them in the parameter lock set action?
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
    pub const RETRIG_VELOCITY_OFFSET: &str = "retrigveloffset";
    pub const SOUND_LOCK: &str = "soundlock";
}

pub mod track_action_type {
    pub const IS_WORK_BUFFER: &str = "iswb";
    pub const OWNER_INDEX: &str = "parentindex";
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
    pub const MASTER_CHANGE: &str = "masterchg";
    pub const KIT_NUMBER: &str = "kitnumber";
    pub const SWING_AMOUNT: &str = "swingamount";
    pub const GLOBAL_QUANTIZE: &str = "globalquantize";
    pub const BPM: &str = "patternbpm";
}

pub mod sound_action_type {
    pub const VERSION: &str = "version";
    pub const INDEX: &str = "index";
    pub const NAME: &str = "name";

    pub const IS_POOL: &str = "ispool";
    pub const IS_KIT: &str = "iskit";
    pub const IS_WORK_BUFFER: &str = "iswb";

    pub const KIT_NUMBER: &str = "kitnumber";

    pub const SOUND_TYPE: &str = "type";

    pub const ACCENT_LEVEL: &str = "accentlevel";

    pub const MACHINE: &str = "machine";

    pub const AMP_ATTACK: &str = "ampattack";
    pub const AMP_HOLD: &str = "amphold";
    pub const AMP_DECAY: &str = "ampdecay";
    pub const AMP_OVERDRIVE: &str = "ampoverdrive";
    pub const AMP_DELAY_SEND: &str = "ampdelaysend";
    pub const AMP_REVERB_SEND: &str = "ampreverbsend";
    pub const AMP_PAN: &str = "amppan";
    pub const AMP_VOLUME: &str = "ampvolume";

    pub const FILT_ATTACK: &str = "filtattack";
    pub const FILT_HOLD: &str = "filthold";
    pub const FILT_DECAY: &str = "filtdecay";
    pub const FILT_RELEASE: &str = "filtrelease";
    pub const FILT_CUTOFF: &str = "filtcutoff";
    pub const FILT_RESONANCE: &str = "filtres";
    pub const FILT_ENVELOPE_AMOUNT: &str = "filtenvamt";

    pub const LFO_SPEED: &str = "lfospeed";
    pub const LFO_FADE: &str = "lfofade";
    pub const LFO_START_PHASE_OR_SLEW: &str = "lfostartphase";
    pub const LFO_DEPTH: &str = "lfodepth";

    pub const SAMP_TUNE: &str = "samptune";
    pub const SAMP_FINE_TUNE: &str = "sampfinetune";
    pub const SAMP_NUMBER: &str = "sampnumber";
    pub const SAMP_BIT_REDUCTION: &str = "sampbitreduction";
    pub const SAMP_START: &str = "sampstart";
    pub const SAMP_END: &str = "sampend";
    pub const SAMP_LOOP_FLAG: &str = "samploopflag";
    pub const SAMP_VOLUME: &str = "sampvolume";

    pub const VEL_MOD_AMT: &str = "velmodamt";
    pub const AT_MOD_AMT: &str = "atmodamt";

    pub const ENV_RESET_FILTER: &str = "envresetfilter";
    pub const VELOCITY_TO_VOLUME: &str = "veltovol";
    pub const LEGACY_FX_SEND: &str = "legacyfxsend";

    pub const SOUND_ACTION_TYPES: &[&str] = &[
        VERSION,
        INDEX,
        NAME,
        IS_POOL,
        IS_KIT,
        IS_WORK_BUFFER,
        KIT_NUMBER,
        SOUND_TYPE,
        ACCENT_LEVEL,
        MACHINE,
        AMP_ATTACK,
        AMP_HOLD,
        AMP_DECAY,
        AMP_OVERDRIVE,
        AMP_DELAY_SEND,
        AMP_REVERB_SEND,
        AMP_PAN,
        AMP_VOLUME,
        FILT_ATTACK,
        FILT_HOLD,
        FILT_DECAY,
        FILT_RELEASE,
        FILT_CUTOFF,
        FILT_RESONANCE,
        FILT_ENVELOPE_AMOUNT,
        LFO_SPEED,
        LFO_FADE,
        LFO_START_PHASE_OR_SLEW,
        LFO_DEPTH,
        SAMP_TUNE,
        SAMP_FINE_TUNE,
        SAMP_NUMBER,
        SAMP_BIT_REDUCTION,
        SAMP_START,
        SAMP_END,
        SAMP_LOOP_FLAG,
        SAMP_VOLUME,
        VEL_MOD_AMT,
        AT_MOD_AMT,
        ENV_RESET_FILTER,
        VELOCITY_TO_VOLUME,
        LEGACY_FX_SEND,
    ];
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

    pub const KIT_ENUM_TYPES: &[&str] = &[
        CONTROL_IN_MOD_TARGET,
        FX_LFO_DESTINATION,
        FX_DELAY_TIME_ON_THE_GRID,
        FX_COMP_ATTACK,
        FX_COMP_RELEASE,
        FX_COMP_RATIO,
        FX_COMP_SIDE_CHAIN_EQ,
    ];
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
    pub const LFO_DESTINATION: &str = "lfodestination";
    pub const VELOCITY_MOD_TARGET: &str = "velmodtarget";
    pub const AFTER_TOUCH_MOD_TARGET: &str = "atmodtarget";
    pub const FILTER_TYPE: &str = "filtertype";
    pub const LFO_MULTIPLIER: &str = "lfomultiplier";
    pub const LFO_WAVEFORM: &str = "lfowaveform";
    pub const LFO_MODE: &str = "lfomode";
    pub const SOUND_SETTINGS_CHROMATIC_MODE: &str = "chromaticmode";

    pub const SOUND_ENUM_TYPES: &[&str] = &[
        MACHINE_PARAMETERS,
        MACHINE_TYPE,
        LFO_DESTINATION,
        VELOCITY_MOD_TARGET,
        AFTER_TOUCH_MOD_TARGET,
        FILTER_TYPE,
        LFO_MULTIPLIER,
        LFO_WAVEFORM,
        LFO_MODE,
        SOUND_SETTINGS_CHROMATIC_MODE,
    ];
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
    pub const METRONOME_TIME_SIGNATURE: &str = "metronometimesignature";

    pub const ROUTING_USB_IN_OPTIONS: &str = "routingusbin";
    pub const ROUTING_USB_OUT_OPTIONS: &str = "routingusbout";
    pub const ROUTING_USB_TO_MAIN_DB: &str = "routingusbtomaindb";

    pub const OUT_PORT_FUNCTION: &str = "outportfunction";
    pub const THRU_PORT_FUNCTION: &str = "thruportfunction";
    pub const INPUT_FROM: &str = "inputfrom";
    pub const OUTPUT_TO: &str = "outputto";
    pub const PARAM_OUTPUT: &str = "paramoutput";
    pub const PAD_DEST: &str = "paddest";
    pub const PRESSURE_DEST: &str = "pressuredest";
    pub const ENCODER_DEST: &str = "encoderdest";
    pub const MUTE_DEST: &str = "mutedest";
    pub const PORTS_OUTPUT_CHANNEL: &str = "portsoutputchannel";

    pub const AUTO_CHANNEL: &str = "autochannel";
    pub const TRACK_CHANNELS: &str = "trackchannels";
    pub const TRACK_FX_CHANNEL: &str = "trackfxchannel";
    pub const PROGRAM_CHANGE_IN_CHANNEL: &str = "programchangeinchannel";
    pub const PROGRAM_CHANGE_OUT_CHANNEL: &str = "programchangeoutchannel";
    pub const PERFORMANCE_CHANNEL: &str = "performancechannel";
}
