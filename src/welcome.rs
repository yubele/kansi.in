struct Welcome;

impl Welcome {
    pub fn index(&mut self) -> String {
        let data: HashMap<&'static str, String> = HashMap::new();
        let form_element = NickelView.element("templates/welcome/_form.mustache", &data);

        let mut hash: HashMap<&'static str, String> =  HashMap::new();
        hash.insert("title", "kansi.in".to_string());
        hash.insert("form", form_element);
        NickelView.element("templates/welcome/index.mustache", &hash)
    }
}