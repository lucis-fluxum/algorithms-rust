use ch1::stack::Stack;

pub fn evaluate_expression(expression: &str) -> Option<f64> {
    let mut ops: Stack<&str> = Stack::new();
    let mut vals: Stack<f64> = Stack::new();

    for item in expression.split_whitespace() {
        match item {
            "(" => {}
            "+" | "-" | "*" | "/" | "sqrt" => ops.push(item),
            ")" => {
                let result = evaluate_subexpression(&mut ops, &mut vals);
                vals.push(result);
            }
            _ => vals.push(item.parse().unwrap()),
        }
    }

    vals.pop()
}

fn evaluate_subexpression(ops: &mut Stack<&str>, vals: &mut Stack<f64>) -> f64 {
    let operand: f64 = vals.pop().unwrap();
    match ops.pop().unwrap() {
        "+" => (vals.pop().unwrap() + operand),
        "-" => (vals.pop().unwrap() - operand),
        "*" => (vals.pop().unwrap() * operand),
        "/" => (vals.pop().unwrap() / operand),
        "sqrt" => (operand.sqrt()),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_single_expression() {
        let sqrt2 = (2 as f64).sqrt();
        assert_eq!(sqrt2, evaluate_expression("( sqrt 2.0 )").unwrap());
        assert_eq!(16 as f64, evaluate_expression("( 3.2 * 5.0 )").unwrap());
    }

    #[test]
    fn evaluates_compound_expression() {
        assert_eq!(
            (2 as f64).sqrt() / 2.0,
            evaluate_expression("( ( ( sqrt 2 ) - 0 ) / ( 2 + 0 ) )").unwrap()
        );
        assert_eq!(
            120 as f64,
            evaluate_expression("( 5 * ( 4 * ( 3 * ( 2 * ( sqrt 1 ) ) ) ) )").unwrap()
        );
    }
}
