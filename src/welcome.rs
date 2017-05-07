struct Welcome;

impl Welcome {
    pub fn index(&mut self, layout: ContentMustache) -> ContentMustache {
        let mut bytes = vec![];
        let mut form_bytes = vec![];
        let mut data = HashMap::new();
        let content_template = mustache::compile_path("templates/welcome/index.mustache").unwrap();
        let form_element_template = mustache::compile_path("templates/welcome/_form.mustache").unwrap();
        let _ = form_element_template.render(&mut form_bytes, &data);
        data.insert("title", "kansi.in");
        data.insert("form", str::from_utf8(&form_bytes).unwrap());
        let _ = content_template.render(&mut bytes, &data);

        ContentMustache {
            path: layout.path,
            content:  str::from_utf8(&bytes).unwrap().to_string()
        }
    }
}