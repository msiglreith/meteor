#![feature(proc_macro)]
extern crate proc_macro;
#[macro_use]
extern crate quote;

extern crate meteor;
extern crate meteor_macro;

use meteor::*;
use meteor_macro::staged;
use proc_macro::TokenStream;

#[staged]
pub fn foo() {
    let x = 2;
    /*
    let x = unsafe { Repr::<u32>::new(quote!{a}) };
    let y = unsafe { Repr::<u32>::new(quote!{b}) };
    let z = unsafe { Repr::<bool>::new(quote!{c}) };

    z.assign(x.eq(y));
    */
}
