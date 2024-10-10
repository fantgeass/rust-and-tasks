/// https://leetcode.com/problems/search-insert-position/
///
/// PARAMS:
/// nums array of integers
/// 1 <= nums.length <= 104
/// -104 <= nums[i] <= 104
/// nums contains distinct values sorted in ascending order.
///
/// target integer
/// -104 <= target <= 104
///
///
/// RETURN:
/// Return the index if the target is found
/// If not, return the index where it would be if it were inserted in order
///
///
/// SOLUTION:
/// should be O(log n)
///
/// Condition "Return the index if the target is found":
/// binary search
///
/// Condition "If not, return the index where it would be if it were inserted in order":
/// return lo
///
/// Example:
/// nums = [1, 3, 5, 6], target = 4
/// lo=0, hi=3, mi=0+(3-0)/2=1, 3 < 4, lo = mi + 1
/// lo=2, hi=3, mi=2+(3-2)/2=3, 6 > 4, hi = mi - 1
/// lo=2, hi=2, mo=2+(2-2)/2=2, 5 > 4, hi = mi - 1
/// lo=2, hi=1, mi=2, lo <= hi breaks
/// return lo

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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

    lo
}

#[test]
fn test_search_insert() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(search_insert(vec![1, 3, 5, 6], -1), 0);
}
