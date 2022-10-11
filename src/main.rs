#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
mod console;

use console::Cmd;

pub fn main() {
    let mut terrux = Cmd::new();
    terrux.init("/","/");
    terrux.run();
}