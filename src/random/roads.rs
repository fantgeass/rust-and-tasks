/// GIVEN:
/// directed edge list
///
/// RESULT:
/// How many edges should be inverted for all nodes to have path to 0
///
/// SOLUTION:
///  DFS
///
///
use std::collections::HashMap;
use std::collections::HashSet;

fn roads(from: &mut Vec<i32>, to: &mut Vec<i32>) -> i32 {
    0
}

mod tests {
    use super::*;

    #[test]
    fn test_roads() {
        assert_eq!(roads(&mut vec![0, 2, 2, 3], &mut vec![1, 1, 4, 4]), 2);
        assert_eq!(roads(&mut vec![3, 2, 1, 0], &mut vec![4, 3, 2, 1]), 4);
        assert_eq!(roads(&mut vec![0, 1, 1, 1, 1], &mut vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(
            roads(&mut vec![1, 6, 6, 3, 0, 5], &mut vec![6, 2, 0, 0, 4, 0]),
            2
        );
    }
}
