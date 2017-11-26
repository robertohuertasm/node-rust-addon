var addon = require('../native');
module.exports = addon;

// function testLoop() {
//   let sum = 0;
//   for (let i = 0; i < 1000000000; i++) {
//     sum = i;
//   }
//   return sum;
// }

// console.time('Js loop');
// console.log(testLoop());
// console.timeEnd('Js loop');

// console.time('Rust loop');
// console.log(addon.testLoop());
// console.timeEnd('Rust loop');