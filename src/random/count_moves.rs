#![allow(unused_imports)]
#![allow(dead_code)]

pub fn solution(s: &mut String) -> i32 {
  let mut c = 0;
  let mut prev_safe = true;

  for (i, char) in s.chars().enumerate() {
    match char {
      '^' | 'v' => {
        c += 1;
        prev_safe = true;
      },
      '>' if i+1 == s.len() => c += 1,
      '>' => prev_safe = false,
      '<' if prev_safe => {
        c += 1;
        prev_safe = true;
      },
      '<' => {
        prev_safe = false;
      }
      _ => {
        panic!("Unknown move")
      }
    }
  }

  c
}

mod test {
  use super::*;

  #[test]
  fn test_solution() {
    assert_eq!(solution(&mut String::from("><^v")), 2);
    assert_eq!(solution(&mut String::from("<<^<v>>")), 6);
    assert_eq!(solution(&mut String::from("><><")), 0);
  }
}