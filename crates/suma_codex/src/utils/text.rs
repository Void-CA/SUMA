pub fn unquote(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') {
        s[1..s.len()-1].to_string()
    } else {
        s.to_string() // si es ident, queda igual
    }
}
