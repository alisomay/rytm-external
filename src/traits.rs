use median::{max_sys, outlet::SendValue, symbol::SymbolRef};

pub trait Post {
    fn obj_post(&self, obj: *mut max_sys::t_object);
    fn obj_error(&self, obj: *mut max_sys::t_object);
    fn post(&self);
    fn error(&self);
}

impl Post for &str {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for String {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for &String {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(obj, self.as_bytes());
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(obj, self.as_bytes());
    }

    fn post(&self) {
        median::post(self.as_bytes());
    }

    fn error(&self) {
        median::error(self.as_bytes());
    }
}

impl Post for SymbolRef {
    fn obj_post(&self, obj: *mut max_sys::t_object) {
        median::object::post(
            obj,
            self.to_string().expect("Couldn't post symbol.").as_bytes(),
        );
    }

    fn obj_error(&self, obj: *mut max_sys::t_object) {
        median::object::error(
            obj,
            self.to_string().expect("Couldn't post symbol.").as_bytes(),
        );
    }

    fn post(&self) {
        median::post(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }

    fn error(&self) {
        median::error(self.to_string().expect("Couldn't post symbol.").as_bytes());
    }
}

pub trait SerialSend {
    fn serial_send_int(&self, out: &Box<dyn SendValue<isize> + Sync>);
}

impl SerialSend for Vec<u8> {
    fn serial_send_int(&self, outlet: &Box<dyn SendValue<isize> + Sync>) {
        for byte in self {
            // This is what I could think of :) Since it needs a long :).
            match outlet.send(max_sys::t_atom_long::from(*byte)) {
                Ok(_) => {}
                Err(_stack_overflow_err) => {
                    // Let's just ignore this for now.
                    // Since when there is a stack overflow max crashes anyway.
                }
            }
        }
    }
}
