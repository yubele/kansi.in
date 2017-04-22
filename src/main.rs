#[macro_use] extern crate nickel;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            let mut data = HashMap::new();
            data.insert("copyright_year", "2017");
            return _res.render("templates/welcome/comingsoon.mustache", &data);
        }
    });

    server.listen("127.0.0.1:6767");
}