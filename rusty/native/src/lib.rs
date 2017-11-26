#[macro_use]
extern crate neon;
extern crate num_cpus;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::js::JsNumber;

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let name: String = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
    let result = format!("hello {}!", &name);
    Ok(JsString::new(scope, &result).unwrap())
}

fn get_cpus(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    Ok(JsNumber::new(scope, num_cpus::get() as f64))
}

fn test_loop(call: Call) -> JsResult<JsNumber> {
    let mut sum = 0;
    for i in 0..1000000000 {
        sum = i;
    }
    Ok(JsNumber::new(call.scope, sum as f64))
}

register_module!(m, {
    try!(m.export("hello", hello));
    try!(m.export("getCpus", get_cpus));
    try!(m.export("testLoop", test_loop));
    Ok(())
});
