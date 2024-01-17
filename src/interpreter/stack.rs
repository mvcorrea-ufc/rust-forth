
#[derive(Debug)]
pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data: Vec::new() }
    }

    // Basic operations:

    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.data.pop() {
            Some(value) => Some(value),
            None => panic!("Stack underflow"),
        }
    }

    pub fn peek(&self) -> Option<i32> {
        self.data.last().copied()
    }

    // Arithmetic operations:

    pub fn add(&mut self) {
        let second = self.data.pop().expect("Stack underflow");
        let first = self.data.pop().expect("Stack underflow");
        self.push(first + second);
    }

    pub fn subtract(&mut self) {
        let second = self.data.pop().expect("Stack underflow");
        let first = self.data.pop().expect("Stack underflow");
        self.push(first - second);
    }

    pub fn multiply(&mut self) {
        let second = self.data.pop().expect("Stack underflow");
        let first = self.data.pop().expect("Stack underflow");
        self.push(first * second);
    }

    pub fn divide(&mut self) {
        let second = self.data.pop().expect("Stack underflow");
        if second == 0 { panic!("Division by zero"); }
        let first = self.data.pop().expect("Stack underflow");
        self.push(first / second);
    }

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
        assert_eq!(stack.pop(), Some(10));  // Test if we got the element we just pushed out
    }


    #[test]
    #[should_panic(expected = "Stack underflow")]
    fn test_pop_empty_stack() {
        let mut stack = Stack::new();
        stack.pop(); // This should panic
    }


    #[test]
    fn test_add() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.add();
        assert_eq!(stack.peek(), Some(3)); // 1 + 2 = 3
    }

    #[test]
    fn test_subtract() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.subtract();
        assert_eq!(stack.peek(), Some(-1)); // 1 - 2 = -1
    }

    #[test]
    fn test_multiply() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.multiply();
        assert_eq!(stack.peek(), Some(2)); // 1 * 2 = 2
    }

    #[test]
    fn test_multiply_by_zero() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(0);
        stack.multiply();
        assert_eq!(stack.peek(), Some(0)); // 1 * 0 = 0
    }

    #[test]
    fn test_divide() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.divide();
        assert_eq!(stack.peek(), Some(0)); // 1 / 2 = 0
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(0);
        stack.divide(); // This should panic
    }

}

