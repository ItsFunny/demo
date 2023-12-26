const ffi = require('ffi-napi');
// const ffi = require('ffi');

function sleep(ms) {
    return new Promise(resolve=>setTimeout(resolve, ms))
}
//
// // 指定 Rust 共享库的路径
const myLib = ffi.Library('/Users/lvcong/RustroverProjects/demo/lib/libmylibrary.dylib', {
    'start_processor': []
});


//
myLib.start_processor();
// async function main() {
//     console.log(1)
//     myLib.start_processor();
//     await sleep(1500)
//     console.log(2)
// }
// main()