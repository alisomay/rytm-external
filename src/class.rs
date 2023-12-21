use median::{
    builder::MaxWrappedBuilder,
    class::Class,
    wrapper::{MaxObjWrapped, MaxObjWrapper},
};
use rytm_rs::prelude::*;
use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::rytm::Rytm;
use median::method::*;

// This trait is for Max specific objects, there is another one for MSP objects.
impl MaxObjWrapped<Self> for Rytm {
    // The constructor for your object
    fn new(builder: &mut dyn MaxWrappedBuilder<Self>) -> Self {
        // You can also add inlets/outlets here modifying the builder
        builder.with_default_inlet_assist("sysex input (connect sysexin)");

        Self {
            project: Arc::new(Mutex::new(RytmProject::default())),
            buffering_sysex: AtomicBool::new(false),
            sysex_in_buffer: Arc::new(Mutex::new(Vec::with_capacity(1024 * 18))),
            sysex_out: builder.add_int_outlet_with_assist("sysex output (connect to midiout)"),
            query_out: builder.add_anything_outlet_with_assist("get query results (list)"),
        }
    }

    // Setup your class here
    fn class_setup(class: &mut Class<MaxObjWrapper<Self>>) {
        // TODO: Add attribute for setting the device id
        class.add_method(Method::Int(Self::int_tramp)).unwrap();
        class
            .add_method(Method::Anything(Self::anything_with_selector_tramp))
            .unwrap();
    }
}
