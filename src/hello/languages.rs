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
