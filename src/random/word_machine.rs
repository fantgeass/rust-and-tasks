#[derive(Debug, PartialEq)]
struct ParseError;

fn last(word_machine: &Vec<u16>) -> Result<&u16, ParseError> {
    word_machine.last().ok_or(ParseError)
}

fn pop(word_machine: &mut Vec<u16>) -> Result<u16, ParseError> {
    word_machine.pop().ok_or(ParseError)
}

fn parse(ops: &str) -> Result<u16, ParseError> {
    let mut word_machine: Vec<u16> = Vec::new();

    for op in ops.split(' ') {
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
                    l1.checked_add(l2)
                } else {
                    l1.checked_sub(l2)
                }
                .ok_or(ParseError)?;

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
        assert!(parse("60000 DUP +".into()).is_err());
        assert!(parse("15 10 -".into()).is_err());
    }
}
