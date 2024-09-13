#![allow(unused_imports)]
#![allow(dead_code)]

#[derive(Debug, PartialEq)]
struct ParseError;

fn parse(ops: &str) -> Result<u32, ParseError> {
  let mut word_machine: Vec<u32> = Vec::new();
  let err_fn = || ParseError;
  // let mut word_machine_pop = || word_machine.pop().ok_or_else(|| ParseError);

  for op in ops.split(' ') {
    match op {
      "POP" => {
        word_machine.pop().ok_or_else(err_fn)?;
      },
      "DUP" => {
        let last_el = word_machine.last().ok_or_else(err_fn)?;
        word_machine.push(last_el.to_owned());
      },
      "+" => {
        let l1 = word_machine.pop().ok_or_else(err_fn)?;
        let l2 = word_machine.pop().ok_or_else(err_fn)?;
  
        match l1.overflowing_add(l2) {
          (sum, false) => word_machine.push(sum),
          _ => return Err(ParseError)
        };
      },
      "-" => {
        let l1 = word_machine.pop().ok_or_else(err_fn)?;
        let l2 = word_machine.pop().ok_or_else(err_fn)?;
  
        match l1.overflowing_sub(l2) {
          (sub, false) => word_machine.push(sub),
          _ => return Err(ParseError)
        };
      },
      op => {
        let val: u32 = op.parse().unwrap();
        word_machine.push(val);
      }
    };
  }

  word_machine.last().ok_or_else(err_fn).copied()
}


mod tests {
  use super::*;

  #[test]
  fn test_word_machine() {
    assert_eq!(parse("4 5 6 - 7 +".into()), Ok(8));
    assert_eq!(parse("13 DUP 4 POP 5 DUP + DUP + -".into()), Ok(7));
    assert!(parse("5 6 + -".into()).is_err());
    assert!(parse("3 DUP 5 - -".into()).is_err());
  }
}