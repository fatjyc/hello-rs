use super::languages;

pub struct Greeter {
    language: String,
    name: String,
}

impl Greeter {
    pub fn new(language: &str, name: &str) -> Greeter {
        Greeter {
            language: language.to_string(),
            name: name.to_string(),
        }
    }

    pub fn greet(&self) -> String {
        let greeting = languages::languages(&self.language);
        if self.name.is_empty() {
            format!("{}, world!", greeting)
        } else {
            format!("{}, {}!", greeting, self.name)
        }
    }
}
