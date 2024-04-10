pub fn strtok<'a>(s: &'a mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtok() {
        let mut x = "hello world";
        assert_eq!(x, "world");
        {
            let hello = strtok(&mut x, ' ');
            assert_eq!(hello, "world");
        }
    }
}
