struct View;
impl View {
    pub fn element(&mut self, path: &str, data: &HashMap<&'static str, String>) -> String {
        let mut bytes = vec![];
        let template = mustache::compile_path(path).unwrap();
        let _ = template.render(&mut bytes, data);
        str::from_utf8(&bytes).unwrap().to_string()
    }
}