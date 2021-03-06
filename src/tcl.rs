use std::ffi::{CString};
use std::env::args;
use std::ptr;
use std::sync;

use ll::*;
use interpreter::Interpreter;
use object::Object;

static INIT_TCL: sync::Once = sync::ONCE_INIT;

/// Acts as proof that Tcl has been correctly initialized for other parts of the code,
/// and wraps all code that must 
pub struct TclEnvironment;

/// Initialize Tcl
pub fn init() -> TclEnvironment {
    let ptr = match args().next() {
        Some(path) => {
            CString::new(path.as_bytes()).unwrap().as_ptr()
        },
        None => ptr::null()
    };

    INIT_TCL.call_once(|| {
        unsafe { Tcl_FindExecutable(ptr) };
    });

    TclEnvironment
}

impl TclEnvironment {

    /// Create a new Tcl interpreter
    pub fn interpreter(&self) -> Result<Interpreter, &str> {
       Interpreter::new(self)
    }

    /// Create an untyped Tcl value
    pub fn object(&self) -> Object {
       Object::new(self)
    }

    /// Create a Boolean Tcl value
    pub fn boolean(&self, val: bool) -> Object {
       Object::new_boolean(self, val)
    }

    /// Create an Integer Tcl value
    pub fn integer(&self, val: i32) -> Object {
       Object::new_integer(self, val)
    }

    /// Create a Long Tcl value
    pub fn long(&self, val: i64) -> Object {
       Object::new_long(self, val)
    }

    //TODO: WideInt
    //TODO: BigNum


    /// Create a Double Tcl value
    pub fn double(&self, val: f64) -> Object {
       Object::new_double(self, val)
    }

    /// Create a String Tcl value
    pub fn string(&self, val: &str) -> Object {
        Object::new_string(self, val)
    }

    /// Create a Byte Array Tcl value
    pub fn byte_array(&self, val: &[u8]) -> Object {
        Object::new_byte_array(self, val)
    }
}

/// Represents the possible states an interpreter can be in after an evaluation
#[derive(Debug)]
pub enum TclResult {
    /// The command completed successfully
    Ok,
    /// There was an error
    Error(String),
    /// The last command was a Return from a function
    Return,
    /// The last command was a Break from a loop
    Break,
    /// The last command was a Continue from a loop
    Continue
}

impl TclResult {
    /// Convert a low-level representation of a Tcl result into a TclResult enum instance
    pub fn from_ll(result: i32, interpreter: &Interpreter) -> TclResult {
        match result {
            TCL_OK => TclResult::Ok,
            TCL_ERROR => TclResult::Error(interpreter.string_result()),
            TCL_RETURN => TclResult::Return,
            TCL_BREAK => TclResult::Break,
            TCL_CONTINUE => TclResult::Continue,
            _ => TclResult::Error("Unknown result".to_string())
        }
    }

    pub fn is_ok(&self) -> bool {
        match *self {
            TclResult::Ok => true,
            _ => false
        }
    }
}
