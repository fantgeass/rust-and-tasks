/// https://leetcode.com/problems/valid-perfect-square/
///
/// PARAMS:
/// num integer
/// 1 <= num <= 231 - 1
///
///
/// RETURN:
/// boolean
/// return true if num is a perfect square or false otherwise
///
///
/// SOLUTION:
/// binary search over numbers
/// checked for overflows

fn is_perfect_square(num: i32) -> bool {
    let mut le = 1;
    let mut ri = num;

    while le <= ri {
        let mi = le + (ri - le) / 2;

        match mi.checked_mul(mi) {
            Some(sq) => match num.cmp(&sq) {
                std::cmp::Ordering::Less => ri = mi - 1,
                std::cmp::Ordering::Greater => le = mi + 1,
                std::cmp::Ordering::Equal => return true,
            },
            None => ri = mi - 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert!(is_perfect_square(1)); // 1 is a perfect square (1^2)
        assert!(is_perfect_square(4)); // 4 is a perfect square (2^2)
        assert!(is_perfect_square(9)); // 9 is a perfect square (3^2)
        assert!(is_perfect_square(16)); // 16 is a perfect square (4^2)
        assert!(is_perfect_square(16384)); // 16 is a perfect square (128^2)

        assert!(!is_perfect_square(2)); // 2 is not a perfect square
        assert!(!is_perfect_square(3)); // 3 is not a perfect square
        assert!(!is_perfect_square(5)); // 5 is not a perfect square
        assert!(!is_perfect_square(6)); // 6 is not a perfect square
        assert!(!is_perfect_square(2147483647)); // 2147483647 is not a perfect square
    }
}
