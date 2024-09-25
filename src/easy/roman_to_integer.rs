/// https://leetcode.com/problems/roman-to-integer/
use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    s.chars()
        .rev()
        .fold((0, 0), |(res, prev), char| {
            let num = *map.get(&char).unwrap();

            if num < prev {
                (res - num, num)
            } else {
                (res + num, num)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".into()), 3);
        assert_eq!(roman_to_int("IV".into()), 4);
        assert_eq!(roman_to_int("IX".into()), 9);
        assert_eq!(roman_to_int("LVIII".into()), 58);
        assert_eq!(roman_to_int("MCMXCIV".into()), 1994);
    }
}
