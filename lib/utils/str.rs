pub fn to_macro_name(s: impl AsRef<str>) -> String {
    let s = s.as_ref().to_string().to_ascii_uppercase();
    return s.replace("-", "_");
}
pub fn quote(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    return "\"".to_string() + s + "\"";
}
