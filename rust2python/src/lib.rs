#[macro_use] extern crate cpython;
/* 

    THIS CODE IS THE BASE CODE FOR CREATING A LIBRARY FOR PYTHON

 */
use cpython::{PyResult, Python};

// add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
py_module_initializer!(librust2py, initlibrust2py, PyInit_librust2py, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust."));
    try!(m.add(py, "sum_as_string", py_fn!(py, sum_as_string_py(a: i64, b:i64))));
    try!(m.add(py, "subtrazion_as_string", py_fn!(py, subtrazion_as_string_py(a: i64, b:i64))));
    Ok(())
});

// logic implemented as a normal rust function
fn sum_as_string(a:i64, b:i64) -> String {
    format!("{}", a + b).to_string()
}

fn subtrazion_as_string(a:i64, b:i64) -> String {
    format_args!("{}", a - b).to_string()
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn sum_as_string_py(_: Python, a:i64, b:i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}

fn subtrazion_as_string_py(_: Python, a:i64, b:i64) -> PyResult<String> {
    let out = subtrazion_as_string(a, b);
    Ok(out)
}