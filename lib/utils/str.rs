/// Convert a name to a canonical form to avoid compatibility issues
pub fn to_macro_name(s: impl AsRef<str>) -> String {
    let s = s.as_ref().to_string().to_ascii_uppercase();
    return s.replace("-", "_");
}

/// Wrap a String in quotes
pub fn quote(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    return "\"".to_string() + s + "\"";
}
