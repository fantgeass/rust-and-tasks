struct WordMachine(Vec<u32>);

impl WordMachine {
  fn parse(ops: &str) -> Result<u32, String> {
    let mut word_machine = WordMachine(Vec::new());

    for op in ops.split(' ') {
      match op {
        "POP" => {word_machine.pop()?;},
        "DUP" => word_machine.dup()?,
        "+" => word_machine.addition_last()?,
        "-" => word_machine.substraction_last()?,
        op => {
          let val: u32 = op.parse().unwrap();
          word_machine.push(val);
        }
      }
    }

    Ok(word_machine.0.last().unwrap().to_owned())
  }

  fn push(&mut self, value: u32) {
    self.0.push(value);
  }

  fn pop(&mut self) -> Result<u32, String> {
    if let Some(n) = self.0.pop() {
      Ok(n)
    } else {
      Err(String::from("Parse Error"))
    }
  }

  fn dup(&mut self) -> Result<(), String> {
    if let Some(value) = self.0.last() {
      self.push(value.to_owned());
      Ok(())
    } else {
      Err(String::from("Parse Error"))
    }
  }

  fn addition_last(&mut self) -> Result<(), String> {
    if self.0.len() >= 2 {
      let l1 = self.pop().unwrap();
      let l2 = self.pop().unwrap();

      match l1.overflowing_add(l2) {
        (r, false) => Ok(self.push(r)),
        _ => Err(String::from("Parse Error"))
      }
    } else {
      Err(String::from("Parse Error"))
    }
  }

  fn substraction_last(&mut self) -> Result<(), String> {
    if self.0.len() >= 2 {
      let l1 = self.pop().unwrap();
      let l2 = self.pop().unwrap();

      match l1.overflowing_sub(l2) {
        (r, false) => Ok(self.push(r)),
        _ => Err(String::from("Parse Error"))
      }
    } else {
      Err(String::from("Parse Error"))
    }
  }
}


mod tests {
  use super::*;


  #[test]
  fn test_word_machine() {
    assert_eq!(WordMachine::parse("4 5 6 - 7 +".into()), Ok(8));
    assert_eq!(WordMachine::parse("13 DUP 4 POP 5 DUP + DUP + -".into()), Ok(7));
    assert!(WordMachine::parse("5 6 + -".into()).is_err());
    assert!(WordMachine::parse("3 DUP 5 - -".into()).is_err());
  }
}