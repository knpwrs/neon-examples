#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsObject, JsString, Object};

fn factory(call: Call) -> JsResult<JsObject> {
    let scope = call.scope;
    // Get and check the type of the first argument
    // The inner `try!` is for the `require` call (makes sure the argument is present)
    // The outer `try!` is for the `check` call (makes sure the argument is the right type)
    let msg = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
    // Return an object with a msg property.
    let obj = JsObject::new(scope);
    try!(obj.set("msg", msg));
    Ok(obj)
}

register_module!(m, {
    m.export("factory", factory)
});
