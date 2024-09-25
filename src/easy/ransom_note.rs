// https://leetcode.com/problems/ransom-note/

use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map = HashMap::new();

    for char in magazine.chars() {
        *map.entry(char).or_insert(0) += 1;
    }

    for char in ransom_note.chars() {
        match map.get_mut(&char) {
            Some(c) if *c != 0 => {
                map.entry(char).and_modify(|c| *c -= 1);
            }
            _ => return false,
        }
    }

    true
}

#[test]
fn test_can_construct() {
    assert_eq!(can_construct("a".into(), "b".into()), false);
    assert_eq!(can_construct("aa".into(), "ab".into()), false);
    assert_eq!(can_construct("aa".into(), "aab".into()), true);
}
