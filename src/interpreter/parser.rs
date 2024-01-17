use super::Stack;

pub fn interpret(input: &str, stack: &mut Stack) -> Result<(), String> {
    println!("Interpreting: {}", input);
    for command in input.split_whitespace() {
        match command.parse::<i32>() {
            Ok(number) => stack.push(number),
            Err(_) => match command {
                "+" => stack.add(),
                "-" => stack.subtract(),
                "*" => stack.multiply(),
                "/" => stack.divide(),
                _ => return Err(format!("Unknown command: '{}'", command)),
            },
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.peek(), Some(10));
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.pop(), Some(10));
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_pop_on_empty() {
       let mut stack = Stack::new();
       stack.pop();
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.peek(), Some(10));
        assert_eq!(stack.peek(), Some(10));
    }

    #[test]
    fn test_add() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.add();
        assert_eq!(stack.peek(), Some(30));
    }

    #[test]
    fn test_subtract() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.subtract();
        assert_eq!(stack.peek(), Some(-10));
    }

    #[test]
    fn test_multiply() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.multiply();
        assert_eq!(stack.peek(), Some(200));
    }

    #[test]
    fn test_divide() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        stack.divide();
        assert_eq!(stack.peek(), Some(0));
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut stack = Stack::new();
        // Test addition
        let _ = interpret("2 3 +", &mut stack);
        assert_eq!(stack.pop(), Some(5));
        // Test subtraction
        let _ = interpret("10 5 -", &mut stack);
        assert_eq!(stack.pop(), Some(5));
        // Test multiplication
        let _ = interpret("3 7 *", &mut stack);
        assert_eq!(stack.pop(), Some(21));
        // Test division
        let _ = interpret("15 3 /", &mut stack);
        assert_eq!(stack.pop(), Some(5));
    }

    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_stack_underflow() {
        let mut stack = Stack::new();
        let _ = interpret("1 +", &mut stack);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_division_by_zero() {
        let mut stack = Stack::new();
        let _ = interpret("5 0 /", &mut stack);
    }

    #[test]
    #[should_panic(expected = "Unknown command: 'foo'")]
    fn test_unknown_command() {
        let mut stack = Stack::new();
        interpret("foo", &mut stack).unwrap();
    }
}

