const rusty = require('rusty');
console.log(`${rusty.hello('Rob')}, your PC has ${rusty.getCpus()} cups`);

function testLoop() {
  let sum = 0;
  for (let i = 0; i < 1000000000; i++) {
    sum = i;
  }
  return sum;
}

console.time('Js loop');
console.log(testLoop());
console.timeEnd('Js loop');

console.time('Rust loop');
console.log(rusty.testLoop());
console.timeEnd('Rust loop');

