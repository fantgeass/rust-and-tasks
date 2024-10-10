/// https://leetcode.com/problems/find-if-path-exists-in-graph/
///
/// Solution:
/// 1. Create adjacency list from edges list
/// 2. Recursevly DFS on it
///

fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut al = vec![vec![]; n as usize];

    for edge in edges {
        al[edge[0] as usize].push(edge[1] as usize);
        al[edge[1] as usize].push(edge[0] as usize);
    }

    fn dfs(al: &Vec<Vec<usize>>, seen: &mut Vec<bool>, source: usize, destination: usize) -> bool {
        if source == destination {
            return true;
        }

        seen[source] = true;

        for i in al[source].iter() {
            if !seen[*i] && dfs(al, seen, *i, destination) {
                return true;
            }
        }

        false
    }

    dfs(
        &al,
        &mut vec![false; n as usize],
        source as usize,
        destination as usize,
    )
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
            true
        );

        assert_eq!(
            valid_path(
                6,
                vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
                0,
                5
            ),
            false
        );

        assert_eq!(
            valid_path(
                10,
                vec![
                    vec![4, 3],
                    vec![1, 4],
                    vec![4, 8],
                    vec![1, 7],
                    vec![6, 4],
                    vec![4, 2],
                    vec![7, 4],
                    vec![4, 0],
                    vec![0, 9],
                    vec![5, 4]
                ],
                5,
                9
            ),
            true
        );
    }
}
