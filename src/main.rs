#[macro_use] extern crate nickel;
#[warn(unreachable_code)]
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

include!("welcome.rs");

fn main() {
    let mut server = Nickel::new();

    #[allow(unreachable_code)]
    server.get("/", middleware! { |_req, res|
        let template = Welcome.index();
        return res.render(template.path, &template.data);
    });

    let _ = server.listen("127.0.0.1:6767");
}