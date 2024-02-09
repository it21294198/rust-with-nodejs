const ffi = require('ffi-napi');

const lib = ffi.Library('../rust-project/target/release/librust_project.dylib',{
    'add_two_numbers':['int',['int','int']],
    'fib':['int',['int']]
});

const add_two_numbers_js = (n1,n2) => {
    return n1+n2;
}

const fib_number = 40
const fib_js = (n) => {
    if(n<=1){
        return n
    }
    return fib_js(n-1)+fib_js(n-2)
}

// calculate execution time using Nodejs
console.time('Execution Time For Nodejs');
let result1 = add_two_numbers_js(12,34);
let result2 = fib_js(fib_number);
console.log(result1)
console.log(result2)
console.timeEnd('Execution Time For Nodejs');

// calculate execution time using rust
console.time('Execution Time For Rust');
let result3 = lib.add_two_numbers(12,34);
let result4 = lib.fib(fib_number);
console.log(result3)
console.log(result4)
console.timeEnd('Execution Time For Rust');
