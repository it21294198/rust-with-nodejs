const ffi = require('ffi-napi');

const lib = ffi.Library('../rust-project/target/release/librust_project.dylib',{
    'add_two_numbers':['int',['int','int']]
});

const add_two_numbers_js = (n1,n2) => {
    return n1+n2;
}

// calculate execution time using Nodejs
console.time('Execution Time For Nodejs');
let result2 = add_two_numbers_js(12,34);
console.log(result2)
console.timeEnd('Execution Time For Nodejs');

// calculate execution time using rust
console.time('Execution Time For Rust');
let result1 = lib.add_two_numbers(12,34);
console.log(result1)
console.timeEnd('Execution Time For Rust');
