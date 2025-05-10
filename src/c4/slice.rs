pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &e) in bytes.iter().enumerate() {
        if e == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}