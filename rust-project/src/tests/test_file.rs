// *Outer file need to be put "!" to mantion this is apply for whole main file.
// src/tests/main_test.rs

#![cfg(test)]
mod testing {
    use rust_project::fib;
    #[test]
    fn test_fib() {
        // Test case 1: First few Fibonacci numbers
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
    }
}


