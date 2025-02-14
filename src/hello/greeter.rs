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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_with_name() {
        let greeter = Greeter::new("en", "John");
        assert_eq!(greeter.greet(), "Hello, John!");
    }

    #[test]
    fn test_greet_without_name() {
        let greeter = Greeter::new("en", "");
        assert_eq!(greeter.greet(), "Hello, world!");
    }

    #[test]
    fn test_greet_different_languages() {
        let greeter_es = Greeter::new("es", "Juan");
        assert_eq!(greeter_es.greet(), "Hola, Juan!");

        let greeter_cn = Greeter::new("cn", "张三");
        assert_eq!(greeter_cn.greet(), "Hello, 张三!");
    }

    #[test]
    fn test_greet_unknown_language() {
        let greeter = Greeter::new("xx", "John");
        assert_eq!(greeter.greet(), "Hello, John!");
    }

    #[test]
    fn test_new_greeter() {
        let greeter = Greeter::new("en", "John");
        assert_eq!(greeter.language, "en");
        assert_eq!(greeter.name, "John");
    }

    #[test]
    fn test_new_greeter_empty_strings() {
        let greeter = Greeter::new("", "");
        assert_eq!(greeter.language, "");
        assert_eq!(greeter.name, "");
    }

    #[test]
    fn test_greet_empty_language() {
        let greeter = Greeter::new("", "John");
        assert_eq!(greeter.greet(), "Hello, John!");
    }
}
