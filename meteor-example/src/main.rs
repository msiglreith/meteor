#![feature(proc_macro)]

extern crate meteor_kernel;
use meteor_kernel::foo;

fn main() {
    let x = foo!();
}
