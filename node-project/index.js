const ffi = require('ffi-napi');

const lib = ffi.Library('../rust-project/target/release/librust_project.dylib',{
    'add_two_numbers':['int',['int','int']]
});

let result = lib.add_two_numbers(12,34);
console.log(result)

