# OxidatedExercises
Code Snippets for Rust language: i.e. Exercises

### Exercises extracted from Linux Pro italia n.172
HTTP Eco Server and Client

### Write to a File
Snippest of code for writing a simple text file. From Website: https://rust-lang-nursery.github.io/rust-cookbook/basics.html#ex-std-read-lines

```Rust
// Manually inserted Error Handling
fn run() -> Result<(), Error> {
    let path = "lines.txt";
```

### Rust2Python
Using Cpython crate to create first library to use within python

```Rust
// The triky part resides here: all the functions mst be initialized one by one.
py_module_initializer!(librust2py, initlibrust2py, PyInit_librust2py, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust."));
    try!(m.add(py, "sum_as_string", py_fn!(py, sum_as_string_py(a: i64, b:i64))));
    try!(m.add(py, "subtrazion_as_string", py_fn!(py, subtrazion_as_string_py(a: i64, b:i64))));
    Ok(())
});
```

