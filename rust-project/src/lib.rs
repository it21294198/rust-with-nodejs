
// extern allwos to call this from C tyle interface
#[no_mangle]
pub extern fn add_two_numbers(n1:i32,n2:i32)->i32{
    n1+n2
}

#[no_mangle]
pub extern fn fib(n:i32)->i32{
    if n <= 1 {
        return n;
    }
    fib(n-1)+fib(n-2)
}


// !unit testing for rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        // Test case 1: Positive numbers
        assert_eq!(add_two_numbers(5, 3), 8);

        // Test case 2: Negative numbers
        assert_eq!(add_two_numbers(-5, -3), -8);

        // Test case 3: Positive and negative numbers
        assert_eq!(add_two_numbers(5, -3), 2);

        // Test case 4: Zero
        assert_eq!(add_two_numbers(0, 0), 0);
    }

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
