/// https://leetcode.com/problems/merge-strings-alternately/  

pub fn merge_alternately(word1: String, word2: String) -> String {
    word1
        .chars()
        .zip(word2.chars())
        .flat_map(|(char1, char2)| [char1, char2])
        .chain(word1.chars().skip(word2.len()))
        .chain(word2.chars().skip(word1.len()))
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        assert_eq!(merge_alternately("abc".into(), "pqr".into()), "apbqcr");
        assert_eq!(merge_alternately("ab".into(), "pqrs".into()), "apbqrs");
        assert_eq!(merge_alternately("abcd".into(), "pq".into()), "apbqcd");
    }
}
