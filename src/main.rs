#[macro_use] extern crate nickel;
#[warn(unreachable_code)]
use std::collections::HashMap;
use nickel::Nickel;
include!("welcome.rs");

fn main() {
    let mut server = Nickel::new();

    Welcome.index(&mut server);

    let _ = server.listen("127.0.0.1:6767");
}