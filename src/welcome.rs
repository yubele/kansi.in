struct Welcome;

impl Welcome {
    pub fn index(&mut self, layout: ContentMustache) -> ContentMustache {
        let mut bytes = vec![];
        let mut data = HashMap::new();
        data.insert("title", "kansi.in");
        let content_template = mustache::compile_path("templates/welcome/comingsoon.mustache").unwrap();
        let _ = content_template.render(&mut bytes, &data);

        ContentMustache {
            path: layout.path,
            content:  str::from_utf8(&bytes).unwrap().to_string()
        }
    }
}