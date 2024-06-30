// inside lib.rs, on the following line should be in here
pub mod entrypoint;
//registering modules in Rust is done by adding the line pub mod <module_name>;.


//you register the new module by adding the line pub mod entrypoint;.
//This tells the Rust compiler that there is a module named entrypoint and it 
// should be included in the compilation process.
pub mod error;