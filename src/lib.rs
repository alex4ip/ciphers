pub fn caesar(cipher: &str, shift: u8) -> String {
    cipher
        .chars()
        .map(|c|{
            let first = { b'a' };
            (first + (c as u8 - first + shift) % 26) as char
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(caesar("", 10), "");
    }

    #[test]
    fn caesar_shifted_1() {
        assert_eq!(caesar("bitcoin", 1), "cjudpjo");
    }

    #[test]
    fn caesar_shifted_2() {
        assert_eq!(caesar("cycled", 26), "cycled");
    }
}
