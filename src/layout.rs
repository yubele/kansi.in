struct ContentMustache {
    path: &'static str,
    content: String
}
struct Layout;
impl Layout {
    pub fn default_layout(&mut self) -> ContentMustache {
        ContentMustache {
            path: "templates/layouts/default.mustache",
            content: String::new()
        }
    }
}