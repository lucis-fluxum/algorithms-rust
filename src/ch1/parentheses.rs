use ch1::stack::Stack;

pub fn check_balance(input: &str) -> bool {
    let mut opens: Stack<char> = Stack::new();

    for c in input.chars() {
        match c {
            '(' | '[' | '{' => opens.push(c),
            ')' | ']' | '}' => match opens.pop() {
                Some(open) => {
                    if !open_matches_close(open, c) {
                        return false;
                    }
                }
                None => return false,
            },
            _ => {}
        }
    }
    opens.is_empty()
}

fn open_matches_close(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positives() {
        assert!(check_balance("()"));
        assert!(check_balance("((((()))))"));
        assert!(check_balance("([]{[()]})"));
        assert!(check_balance("[()]{}{[()()]()}"));
    }

    #[test]
    fn negatives() {
        assert!(!check_balance("["));
        assert!(!check_balance("{[(])}"));
        assert!(!check_balance("[{[)()]"));
        assert!(!check_balance("[[()]()[()]()[()])]"));
    }
}
