// https://leetcode.com/problems/jewels-and-stones/

use std::collections::HashSet;

fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let set: HashSet<char> = jewels.chars().collect();

    stones
        .chars()
        .filter(|s| set.contains(s))
        .count()
        .try_into()
        .unwrap()
}

mod tests {
    use super::*;

    #[test]
    fn test_num_jewels_in_stones() {
        assert_eq!(
            num_jewels_in_stones("aA".to_string(), "aAAbbbbbB".to_string()),
            3
        );

        assert_eq!(num_jewels_in_stones("z".to_string(), "ZZ".to_string()), 0);
    }
}
