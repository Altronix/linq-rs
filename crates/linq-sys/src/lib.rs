#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// TODO this macro erases IDE type completion
// Should probably add post build script to copy manually
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
