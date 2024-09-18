#[derive(Debug, PartialEq)]
struct ParseError;

const MAX_20_BIT: u32 = 1048575;

fn last(word_machine: &Vec<u32>) -> Result<&u32, ParseError> {
    word_machine.last().ok_or(ParseError)
}

fn pop(word_machine: &mut Vec<u32>) -> Result<u32, ParseError> {
    word_machine.pop().ok_or(ParseError)
}

fn parse(ops: &str) -> Result<u32, ParseError> {
    let mut word_machine: Vec<u32> = Vec::new();

    for op in ops.split_whitespace() {
        match op {
            "POP" => {
                pop(&mut word_machine)?;
            }
            "DUP" => {
                let last_el = last(&word_machine)?;
                word_machine.push(*last_el);
            }
            "+" | "-" => {
                let l1 = pop(&mut word_machine)?;
                let l2 = pop(&mut word_machine)?;

                let sum_or_sub = if op == "+" {
                    let sum = l1 + l2;
                    if sum > MAX_20_BIT {
                        return Err(ParseError);
                    }
                    sum
                } else {
                    if l1 < l2 {
                        return Err(ParseError);
                    }
                    l1 - l2
                };

                word_machine.push(sum_or_sub);
            }
            op => {
                let num = op.parse().unwrap();
                word_machine.push(num);
            }
        };
    }

    last(&word_machine).copied()
}

mod tests {
    use super::*;

    #[test]
    fn test_word_machine() {
        assert_eq!(parse("4 5 6 - 7 +".into()), Ok(8));
        assert_eq!(parse("13 DUP 4 POP 5 DUP + DUP + -".into()), Ok(7));
        assert!(parse("5 6 + -".into()).is_err());
        assert!(parse("3 DUP 5 - -".into()).is_err());
        assert!(parse("1048575 DUP +".into()).is_err());
        assert!(parse("15 10 -".into()).is_err());
    }
}
