struct Template {
    path: &'static str,
    data: HashMap<&'static str, &'static str>
}
struct Welcome;

impl Welcome {
    pub fn index(&mut self) -> Template {
        let mut data = HashMap::new();
        data.insert("copyright_year", "2017");
        Template{ path: "templates/welcome/comingsoon.mustache", data: data }
    }
}