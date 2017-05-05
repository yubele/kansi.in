#[macro_use] extern crate nickel;
#[warn(unreachable_code)]
extern crate mustache;
extern crate rustc_serialize;
use std::str;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};


include!("welcome.rs");

fn main() {
    let mut server = Nickel::new();

    #[allow(unreachable_code)]
    server.get("/", middleware! { |_req, res|

        let template = Welcome.index();
        let mut bytes = vec![];
        let content_template = mustache::compile_path(template.path).unwrap();
        let mut data = HashMap::new();
        data.insert("testｌ", "ok");
        let _ = content_template.render(&mut bytes, &data);

        let mut data = HashMap::new();
        data.insert("content", str::from_utf8(&bytes).unwrap());
        data.insert("copyright_year", "2017");
        return res.render("templates/layouts/default.mustache", &data);
    });

    let _ = server.listen("127.0.0.1:6767");
}