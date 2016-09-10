#[macro_use]
extern crate neon;

use neon::mem::Handle;
use neon::vm::{Call, JsResult};
use neon::js::{JsFunction, JsNull, JsString, JsUndefined};

fn default(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    // Get and check the type of the first argument
    // The inner `try!` is for the `require` call (makes sure the argument is present)
    // The outer `try!` is for the `check` call (makes sure the argument is the right type)
    let cb = try!(try!(call.arguments.require(scope, 0)).check::<JsFunction>());
    // Call the function with a string argument
    let args: Vec<Handle<JsString>> = vec![JsString::new(scope, "Hello, World!").unwrap()];
    try!(cb.call(scope, JsNull::new(), args)).check::<JsUndefined>()
}

register_module!(m, {
    m.export("default", default)
});
