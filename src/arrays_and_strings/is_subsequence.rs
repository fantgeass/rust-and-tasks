/// https://leetcode.com/problems/is-subsequence/

fn is_subsequence(s: String, t: String) -> bool {
    let mut t_iter = t.chars();
    for c in s.chars() {
        match t_iter.find(|&p| p == c) {
            Some(_) => (),
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert!(is_subsequence("abc".into(), "ahbgdc".into()));
        assert!(!is_subsequence("axc".into(), "ahbgdc".into()));
        assert!(is_subsequence("".into(), "ahbgdc".into()));
    }
}
