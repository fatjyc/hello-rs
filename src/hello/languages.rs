pub fn languages(lang: &str) -> &str {
    match lang {
        "en" => "Hello",
        "es" => "Hola",
        "fr" => "Bonjour",
        "de" => "Hallo",
        "it" => "Ciao",
        "pt" => "Olá",
        "ru" => "Привет",
        "ja" => "こんにちは",
        "zh" => "你好",
        "ko" => "안녕하세요",
        "ar" => "مرحبا",
        "hi" => "नमस्ते",
        "bn" => "হ্যালো",
        "sw" => "Habari",
        "am" => "ሰላም",
        "ha" => "Sannu",
        "yo" => "Pẹlẹ",
        "ig" => "Ndewo",
        "zu" => "Sawubona",
        _ => "Hello",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english() {
        assert_eq!(languages("en"), "Hello");
    }

    #[test]
    fn test_spanish() {
        assert_eq!(languages("es"), "Hola");
    }

    #[test]
    fn test_french() {
        assert_eq!(languages("fr"), "Bonjour");
    }

    #[test]
    fn test_chinese() {
        assert_eq!(languages("zh"), "你好");
    }

    #[test]
    fn test_japanese() {
        assert_eq!(languages("ja"), "こんにちは");
    }

    #[test]
    fn test_arabic() {
        assert_eq!(languages("ar"), "مرحبا");
    }

    #[test]
    fn test_default_unknown_language() {
        assert_eq!(languages("xx"), "Hello");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(languages(""), "Hello");
    }

    #[test]
    fn test_german() {
        assert_eq!(languages("de"), "Hallo");
    }

    #[test]
    fn test_italian() {
        assert_eq!(languages("it"), "Ciao");
    }

    #[test]
    fn test_portuguese() {
        assert_eq!(languages("pt"), "Olá");
    }

    #[test]
    fn test_korean() {
        assert_eq!(languages("ko"), "안녕하세요");
    }

    #[test]
    fn test_hindi() {
        assert_eq!(languages("hi"), "नमस्ते");
    }

    #[test]
    fn test_bengali() {
        assert_eq!(languages("bn"), "হ্যালো");
    }

    #[test]
    fn test_swahili() {
        assert_eq!(languages("sw"), "Habari");
    }

    #[test]
    fn test_amharic() {
        assert_eq!(languages("am"), "ሰላም");
    }

    #[test]
    fn test_hausa() {
        assert_eq!(languages("ha"), "Sannu");
    }

    #[test]
    fn test_yoruba() {
        assert_eq!(languages("yo"), "Pẹlẹ");
    }

    #[test]
    fn test_igbo() {
        assert_eq!(languages("ig"), "Ndewo");
    }

    #[test]
    fn test_zulu() {
        assert_eq!(languages("zu"), "Sawubona");
    }

    #[test]
    fn test_case_sensitive() {
        assert_eq!(languages("EN"), "Hello");
        assert_eq!(languages("En"), "Hello");
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(languages(" en "), "Hello");
        assert_eq!(languages("\ten\n"), "Hello");
    }
}
