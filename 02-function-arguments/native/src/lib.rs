#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsNumber;

fn add(call: Call) -> JsResult<JsNumber> {
    let scope = call.scope;
    // Get and check the types of the first two arguments
    // The inner `try!` is for the `require` call (makes sure the argument is present)
    // The outer `try!` is for the `check` call (makes sure the argument is the right type)
    let first = try!(try!(call.arguments.require(scope, 0)).check::<JsNumber>());
    let second = try!(try!(call.arguments.require(scope, 1)).check::<JsNumber>());
    // Get the actual values (f64), add them, and return a new JavaScript Number
    Ok(JsNumber::new(scope, &first.value() + &second.value()))
}

register_module!(m, {
    m.export("add", add)
});
