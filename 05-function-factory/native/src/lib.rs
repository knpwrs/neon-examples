#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::{JsFunction, JsString};

fn my_function(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    Ok(JsString::new(scope, "hello world").unwrap())
}

fn factory(call: Call) -> JsResult<JsFunction> {
    let scope = call.scope;
    JsFunction::new(scope, my_function)
}

register_module!(m, {
    m.export("factory", factory)
});
