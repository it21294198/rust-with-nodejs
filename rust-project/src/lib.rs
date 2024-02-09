
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