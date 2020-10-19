#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[link(name="badmath", kind ="dylib")]

fn main() {
    println!("Hello, from RUST!");
    let res = unsafe{bad_add(9.,12.)};
    println!("{}?? Are you sure that's right?", res);
}
