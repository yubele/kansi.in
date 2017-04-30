struct Welcome;
#[allow(unreachable_code)]
impl Welcome {
    pub fn index(self, server: &mut nickel::Nickel) {
        server.utilize(router! {
            get "/" => |_req, res| {
                let mut data = HashMap::new();
                data.insert("copyright_year", "2017");
                return res.render("templates/welcome/comingsoon.mustache", &data);
            }
        });
    }
}