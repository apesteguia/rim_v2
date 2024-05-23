#[derive(Debug, PartialEq)]
pub enum Lenguaje {
    Rust,
    Elixir,
    C,
    Cpp,
    JavaScript,
    TypeScript,
    Java,
    Lua,
    Python,
    Txt,
    Markdown,
    Jsx,
    Assembly,
    Haskell,
    OCaml,
    Clojure,
    Go,
    Css,
    Html,
    Bash,
    Php,
    Ruby,
    Undefined,
}

pub fn obtener_nombre_lenguaje(c: impl Into<String>) -> Lenguaje {
    let codigo = c.into();
    match codigo.to_lowercase().as_str() {
        "rs" => Lenguaje::Rust,
        "ex" | "exs" => Lenguaje::Elixir,
        "c" | "h" => Lenguaje::C,
        "cpp" | "c++" | "hpp" => Lenguaje::Cpp,
        "js" => Lenguaje::JavaScript,
        "ts" => Lenguaje::TypeScript,
        "java" => Lenguaje::Java,
        "lua" => Lenguaje::Lua,
        "py" => Lenguaje::Python,
        "txt" => Lenguaje::Txt,
        "md" | "mdx" => Lenguaje::Markdown,
        "jsx" | "tsx" => Lenguaje::Jsx,
        "s" | "asm" | "nasm" => Lenguaje::Assembly,
        "hs" => Lenguaje::Haskell,
        "ml" | "mli" => Lenguaje::OCaml,
        "cjl" => Lenguaje::Clojure,
        "go" => Lenguaje::Go,
        "css" => Lenguaje::Css,
        "html" | "htmx" => Lenguaje::Html,
        "sh" => Lenguaje::Bash,
        "php" => Lenguaje::Php,
        "rb" => Lenguaje::Ruby,
        _ => Lenguaje::Undefined,
    }
}

pub fn reserved_words(l: &Lenguaje) -> Vec<String> {
    match l {
        Lenguaje::Rust => vec![
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
            "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
            "return", "Self", "self", "static", "struct", "super", "trait", "true", "type",
            "unsafe", "use", "where", "while", "async", "await", "dyn", "None", "Ok", "Some",
            "Option", "Result", "Err",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
        _ => vec![
            "if", "while", "else", "let", "var", "loop", "pub", "public", "private", "priv",
            "class", "fn", "fun", "func", "const", "import", "try", "catch", "new", "static",
            "void", "int", "return", "char", "string", "String", "float", "double", "number",
            "bool", "boolean", "when", "with", "where", "true", "false", "#include", "#define",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    }
}
