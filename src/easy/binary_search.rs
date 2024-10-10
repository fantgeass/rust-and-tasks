/// https://leetcode.com/problems/binary-search/
///
/// El classico

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo: i32 = 0;
    let mut hi: i32 = nums.len() as i32 - 1;

    while lo <= hi {
        let mi = lo + (hi - lo) / 2;
        match nums[mi as usize].cmp(&target) {
            std::cmp::Ordering::Less => lo = mi + 1,
            std::cmp::Ordering::Greater => hi = mi - 1,
            std::cmp::Ordering::Equal => return mi,
        }
    }

    -1
}

#[test]
fn test_search() {
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(search(vec![5], -5), -1);
}
