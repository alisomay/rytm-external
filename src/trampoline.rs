use crate::traits::*;
use median::max_sys::{t_atom, t_atom_long, t_symbol};
use median::wrapper::MaxObjWrapper;
use std::os::raw::c_long;

use median::method;
use median::wrapper::WrapperWrapped;

use crate::rytm::Rytm;

impl Rytm {
    pub extern "C" fn int_tramp(wrapper: &::median::wrapper::MaxObjWrapper<Rytm>, v: t_atom_long) {
        WrapperWrapped::wrapped(wrapper).int(v)
    }

    pub extern "C" fn query_tramp(
        wrapper: &MaxObjWrapper<Rytm>,
        sel: *mut t_symbol,
        ac: c_long,
        av: *const t_atom,
    ) {
        method::sel_list(sel, ac, av, |sym, atoms| {
            WrapperWrapped::wrapped(wrapper).query(&sym, atoms);
        });
    }

    pub extern "C" fn send_tramp(
        wrapper: &MaxObjWrapper<Rytm>,
        sel: *mut t_symbol,
        ac: c_long,
        av: *const t_atom,
    ) {
        method::sel_list(sel, ac, av, |sym, atoms| {
            WrapperWrapped::wrapped(wrapper).query(&sym, atoms);
        });
    }
}
