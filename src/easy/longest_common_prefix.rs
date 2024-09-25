fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = &strs[0][..];

    for str in &strs[1..] {
        while !str.starts_with(prefix) {
            prefix = &prefix[..prefix.len() - 1];
        }
    }

    prefix.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
            String::from("fl")
        );

        assert_eq!(
            longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
            String::from("")
        );
    }
}
