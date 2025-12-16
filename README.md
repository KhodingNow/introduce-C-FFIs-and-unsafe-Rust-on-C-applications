Intro to C FFI and unsafe code
If we want to embed Rust code within an existing application, we need some very well defined semantics for how the two lanvguages communicate, how values are passed bween them, and how memory may or may not be shared between them.Ideally, this interface between the two languages and platforms so we can avoid re-writing code to perform a specific integration.

One well-supported method is  to write functions that behave identically to C functions at run time. They use the same calling conventions, pass parameters and return values in the same way, and use types that can be represented safely in either language.
This method is referred to as the "C Foreign Function Interface (FFI). 
We will discuss how to write such Rust functions and use FFI support in Rust to integrate Rust code into a C application. We will also discuss how to use 'unsafe' blocks and functions to peform some operations that normal Rust code doesn't allow and when and why these blocks are neccessary whn writing FFI code. 
 
