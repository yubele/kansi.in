#[macro_use] extern crate nickel;
#[warn(unreachable_code)]
extern crate mustache;
extern crate rustc_serialize;
use std::str;
use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

include!("layout.rs");
include!("welcome.rs");

fn main() {
    let mut server = Nickel::new();
    #[allow(unreachable_code)]
    server.get("/", middleware! { |_req, res|
        let default_layout = Layout.get_layout();
        let content_mustache = Welcome.index(default_layout);
        let mut data = HashMap::new();

        data.insert("content", &*content_mustache.content);
        data.insert("copyright_year", "2017");
        return res.render(content_mustache.path, &data);
    });

    let _ = server.listen("127.0.0.1:6767");
}