use crate::val::Value ;
use anyhow::{Result, Context};

#[derive(Default)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn push(&mut self, val: T) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Result<T> {
        self.stack.pop()
            .context("Failed to pop a value when the stack is empty")
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    type I32Stack = Stack<i32>;

    #[test]
    fn new() {
        let stack = I32Stack::default();
        assert!(stack.is_empty());
    }

    #[test]
    fn push() {
        let mut stack = I32Stack::default();
        stack.push(0);

        assert!(!stack.is_empty());
    }

    #[test]
    fn pop() { 
        let mut stack = I32Stack::default();
        stack.push(99);
        let val = stack.pop().unwrap();

        assert!(stack.is_empty());
        assert_eq!(val, 99);
    }

    #[test]
    #[should_panic]
    fn pop_empty() {
        let mut stack = I32Stack::default();
        stack.pop().unwrap();
    }
}
