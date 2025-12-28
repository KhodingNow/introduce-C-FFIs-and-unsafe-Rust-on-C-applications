Intro to C FFI and unsafe code
If we want to embed Rust code within an existing application, we need some very well defined semantics for how the two lanvguages communicate, how values are passed bween them, and how memory may or may not be shared between them.Ideally, this interface between the two languages and platforms so we can avoid re-writing code to perform a specific integration.

One well-supported method is  to write functions that behave identically to C functions at run time. They use the same calling conventions, pass parameters and return values in the same way, and use types that can be represented safely in either language.
This method is referred to as the "C Foreign Function Interface (FFI). 
We will discuss how to write such Rust functions and use FFI support in Rust to integrate Rust code into a C application. We will also discuss how to use 'unsafe' blocks and functions to peform some operations that normal Rust code doesn't allow and when and why these blocks are neccessary whn writing FFI code. 
 
Unsafe Rust.

One of Rust's selling points is the memory safety it affords application developers.However, we may want to shed some of that memory safety to improve performance, increase simplicity, or, most interesting  to us, deal with types that Rust compiler can't reson about.
As we know from the discussion of the lifetime and ownership system - the Rust compiler can reason about when memoryis safe to use and discard based on the adherence to a fewrules in Rust code.

However, the Rust compiler is not able to make any assumptions about the ways in which memory is allocated, accessed, or de-allocated in any code other than Rust code.
If we want to deal with dynamic memory that was not created from within Rust code, we need to use unsafe code. 

NOTE: 'Unsafe' is a bit of a misnomer - it does not invalidate the safety concerns that we have in the rest of the our Rust code. It simply means that the developer is responsible for upholding Rust's safety rules without the compiler strickly checking them.

Unsafe code blocks allow a few operations that are forbidden in safe Rust code:

- Dereference raw pointers
- Call functions marked as unsafe
- Implement traits marked as unsafe
- Mutate static values
- Access fields of a union

There really isn't anything beyond these five items. There are no other secret magic or dangerous  operations. Without a doubt, the most fundamental of all of these unsafe operations is the dereferencing of raw pointers.

Raw pointers.

Pointers are values that tell us the memory locations of other values. If we imagine our computer's main memory as a giant array of bytes, pointers are indices into that arry 
The value of a pointer is a memory address, which varies in size depending on your computer's architecture. On the most modern systems, memory is addressed at the byte level using 64-bit addresses, meaning that pointers are 64-bit numbers that point to individual bytes in computer memory.

To dereference a pointer is to access the value that the pointer points to.
In the stack (memory) while a simple C program is running.
Imagine a character variable 'x', a variable that points to the character variable 'y', and a character variable that is assigned the result of de-referencing 'y'.

Imagine running this C program on a theoretical computer that has a single-byte pointer address. 


 C code example:

    int main() {
        char x = 'a';
        char *y = &x;

        char z = *y;
    }

visual of the stack:

[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]

0       1     2     3      4     5     6     7   

 - no code has been executed , so the stack is EMPTY.

    int main() {
        char x = 'a';
        char *y = &x;

        char z = *y;
    }

[ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07 ]

 'a'      1     2     3     4     5     6      7 

we store the character 'a' in the first position on the stack

    int main() {
        char x = 'a';
        char *y = &x;

        char z = *y;
    
    }

[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07 ]

'a'    0x00    2      3     4     5    6     7

The next value placed on the stack is the memory address of the variable 'x'. In this case, it is 0x00.

    int main() {
        char x = 'a';
        char *y = &x;

        char z = *y;
    }

[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07]

'a'    0x00  'a'    3     4      5     6     7

This operation is de-referencing . The variable 'y' holds the memory address '0x00' .We look up th
e value that is stored at the address 0x00 and put it on the stack, referred to by the variable 'z'.   

NB: THIS is a program's stack memory during reference and de-referencing operations!

The reason why this operation needs to be hidden behind 'unsafe' blocks is very simple. Recall, the second rule of Rust references: references must always be valid. At run time, a reference and a raw pointer are identical: they are both values that hold a memory address, which is used to look up a value in memory. 

The only diferrence is their behaviour at compile time. Because Rust references have extra information about them known by the compiler, such as their lifetimes, the compiler knows that they are always valid and that de-referencing them is always safe .

If a raw pointer is created, it is simply an address in memory, it has no lifetime or ownership information attached to it.

The compiler has no way to validate that the memory it points to is valid, so it is up to the programmer to validate it.
One of the most common operations in Rust code operating between languages is reading through a buffer of data, such as a C-style array.

Reading the elements of a vector using pointer arithmetic:

    fn main() {
        let data: Vec<u8> = [5, 10, 15, 20];

        read_u8_slice(data.as_ptr(), data.len());
    }

    fn read_u8_slice(slice_p): *const u8, length: usize {       for index in 0..length {
        unsafe {
            println!("slice[{}] = {index}", 
            *slice_p.offset(index as isize));
            
             }
         }
     }

A Vec is analogous to a C++ std::vector or a Java ArrayList and similar to a list in Python, although lists may hold values of different types. A u8 is an unsigned, 8-bit integer, a single byte. 
Combining these as a Vec<u8>, we get a growable block of memory containing individual byte values.  

The 'as_ptr' method is used to get a pointer to the data buffer inside of the Vec.Getting the pointer is a completely safe operations. We only need to introduce 'unsafe' when we want to de-reference the pointer.

Immutable pointers (*const = aka a raw pointer) and mutable pointers (*mut = another raw pointer) are very similar to immutable and mutable references, respectively.

If a value is behind a '*const', it cannot be mutated. If you need to mutate a value, you must use a (*mut). 
One key difference between pointers and references in this respect is that an immutable pointer can be cast to a mutable pointer.

It is the developer's responsibility to know when this action is safe or not safe. 

C Foreign Function Interface.

With pointer de-referencing having been discussed and understood, we can write Rust code that communicates with C code. Reading from and writing to pointers that Rust codeaccepts from C requires us to apply our knowledge of pointer operations. 

Imagine that we have an existing C application that solves a simple arithmetic expression in Reverse Polish Notation (RPN).Currently, this program accepts expressions containing a single operation. You have been tasked with extending the application to support multiple operations in a single expression.

This extra functionality should be written in Rust, however, the current C code that performs user operations like text input and output should remain in C.
 
RPN is a way to write arithmetic expressions that negate the need for precedence rules for operations.It is essentially a simple programming language that operates on a stack machine.
Elements are separated by spaces, and arithmetic operators work on the previous two items in the expression, instead of the preceeding element and following element, as is the case with the more commonly used infix operations.
Some example expressions written in infix notation and their counterparts in RPN are, respectively,

1.    Infix: 3 + 4 * 12
      RPN  : 4 12 * 3 +
         = 51
2.
    Infix: (3 + 4) * 12
    RPN  : 3 4 + 12 *
         = 84 


RPN avoids the ambiguity of infix notation by always operating in strictly left-to-right order.The orders of operation for the first and second RPN expressions is different because the operations are literally written in a different order.

It is far easier to write a calculator that parses expressions in the RPN format because we can avoid the complications of ordering operations and just work from left to right.

RPN stack usd to calculate 3 4 + 12 *:

 

3 4 + 12 * [] 

no items have been parsed so the stack is empty


3 4 + 12 * [3, ] 
The first number - in this case, 3  is put onto the stack
3 4 + 12 * [3, 4, ]

3 4 + 12 * [3, 4 ] 
-> 3 + 4 = 7 

Upon encountering the addition operator, we pop the previous two values from the stack & add them together. 

 [7] ....Then we push the result onto the stack

3 4 + 12 * [7, 12, ]


3 4 + 12 * [7, 12, ] => 7 * 12 = 84 [84, ]
The pop, pop, operate, push steps are identical for multiplication, with only the operation in the middle being different.

 
3 4+ 12 * [84, ] 
When the end of input is reached, we pop a single value from the stack, and that is the answer.  


Our C application currently takes newline-delimited integer arithmetic expressions from the user on STDN, parses the expression, and then calculates and displays the result on STDOUT.
We need to add support for multiple nested arithmetic expressions; right now, our calculator only could move the string-parsing code out of C and into Rust. Since we've heard some nice things abbout Rust, let's try using it to solve our problem. 

First, lets look at what the C arithmetic calculator code looks like. 


    #include <stdio.h>
    #include <string.h>

    int solve(char *line, int *solution);

    int main() {
        char line [100];
        int solution;

        while (1) {
            printf("> ");
            if (fgets(line, 100, stdin) == NULL) {
               return 0; 
            }

            if (solve(line, &solution)) {
                continue;
            }

            printf("%d\n", solution);

            }

            return 0;
               
            }

       int solve(char *line, int *solution) {
        int num1, num2;
        char operator;

        int values_read = sscanf(
            line, "%d %d %c", &num1, num2, &operator);
        if (values_read != 3) {
            return 1;
        }

        switch (operator) {
        case '+':
            *solution = num1 + num2;
            return 0;
        case '-':
            *solution = num1 - num2;
            return 0;
        case '*':
            *solution = num1 * num2;
            return 0;
        case '/':
            *solution = num1 / num2;
            return 0;
        }

        return 1;
       }

'char line [100];' allocates space on the stack of the main function to store up to 100 characters for the date we're going to read in from the user. Since we 'fgetdon't need to access multiple lines of text at once, we keep reusing the same memory buffer over and over again. The function will clear it when it reads data from STDIN.


'fgets' reads the data from STDIN and takes a char pointer as its first argument, which should point to the allocated memory where the data from the file will be read to. The memory must have allocated space for at least as many characters as the second argument. Because we allocated space for 100 characters, we give 100 as the second argument. 

C pointers and their associated memory don't contain data on where the allocated memory regions ends, so for many functions, the developer needs to explicitly specify the size of memory regions, which ensures that 'fgets' never writes past the end of our buffer.
 
'solve' returns 'int', which is a status code. 0 means the function worked correctly, and 1 means that the string did not parse as expected.

If we put this code ina file called 'calculator.c' and run it , it will solve simple arithmetic problems as expected:

    $ gcc calculator.c -o calculator
    $ ./calculator 
    > 3 40 *
    120
    > 120 3 /
    40 
    > 40 1345 * 
    53800
    > 53800 3 /
    17933

It does great with these simple expressions, but what happens if we try to add extra operations ?

    > 3 40 * 2 -
    120 
    > 10 10 * 10 *
    100
    > 10 10 * hello
    
    100

Anything after the first three item is ignored. Remember that we have been tasked with adding support to multiple operations in a single expression to thhis calculator.Let's see whether we can extract a key component from it and move it into Rust.

The first step is to identify what we want to extract.Given the fact that our program here has two functionsand one of them is the 'main' function., we should start by moving the solve function into Rust.


Let us start a new project with the Cargo command, this will will create a project with a ' main.rs' entry point - something that can run directly as an executable. We are not creating an executable, instead, we want to create a LIBRARY. So, we need to provide an additional flag to 'cargo new' to indicate this desire:

    cargo new --lib calculate
    
Open this newly created calculate/src/lib.rs file, and wecan begin.Recall that when creating an executable, newly created 'main.rs' files include the "Hello World" programby default. 
Similarly, when creating a library, Cargo will fill our 'lib.rs' file with basic unit tests scaffolding, which we can use to validate the functionality of our program.      
When we bring over the functionality of the 'solve' function from C to Rust, we need to provide our C code with a function that has the same SIGNATURE as the old 'solve' function.
The SIGNATURE of a function refers to the types of all the values that a function accepts as parameters and returns, as aweel as the semantic meanings of those values. Recall the signature of our C function:


    int solve(char *line, int *solution)

For our C code to call a Rust function, we need to write a Rust function that accepts a 'char' pointer and an 'int'pointer as parameters and returns an 'int'

Here is what that same signature will look like in Rust:

    fn solve(line: *const c_char, solution: *mut c_int) -> c_int

We can already glean more information from our Rust function's signature than from the signature of the C function.The Rust function tells the value of solutions may be modified inside the function and the value of line will not be modified. The C code provides no indication, other than reading the code, that solution will be modified by the solve function.
A developer can always add comments, of course, but comments may be inaccurate or become out of date.

The 'c_char' and 'c_int' types in the function signature are not built into the Rust standard library; they need to be imported from the libc crate. Crates are the Rust term for packages or libraries - collections of functions and types that can be used by others to perform certain tasks.
The libc crate provides raw FFi bindings to the C standard library.The C standard does provide some relative sizing guarantees.For example, int is always at least as large as 'short int', but beyond that, a C int is platform specific. libc abstracts over some of this platform-specific nature by providing Rust types for the C primitives, whose sizingis determined by the platform on which they were compiled.

Since many Rust programs don't need to intercat with C libraries, this functionality is not included in the standard library and is instead in an external library. 

Including a crate.

When we've used Cargo in the past, its been to create a new Rust package or to compile and run a Rust progrom. However, Cargo can do so much more than that.Cargo can also download, compile, and link dependencies and perform many other functions that would normally require lots of configurations in C or C++ programs.It is an all-in-one program for interacting with Rust. For now, we are going to ask Cargo to include (libc) when compiling our 'calculate' crate.

Cargo's configuration file is (Cargo.toml). All the information that Cargo needs about how to compile a crate is contained herein.It contains compiler feature sets to activate, third-party crates to download/compile and their versions, conditional compilation flags, and information that you need to include if you're creating a crate you want others to be able to use (e.g, your contact info, readme, version information, etc).

The [dependencies] section is the most commonly used section of the file for most Rust developers.Under this line, we type the name and version number of the crate we wish to include. Subsequently, when we use Cargo commands that compile our Rust program, Cargo will download the appropriate version of the creates we requested, compile them, and link them with our crate.
We don't need to worry about setting compiler flags.There is no separeate step; just write the crate you want, and Cargo will get them.

To search for available crates....look at (crates.io).When Cargo is used to build and publish packages, they go (by default) to crates.io.Here you can see all the publicly available crates that you can use when building Rust applications and crates of your own.

To include (libc) in our calculate crate, let's add a line under the [dependencies] section.Dependencies are specified with the name of the package, an equals sig(=), and the version of the package you'd like to use. 

NB: (libc may change versions depending on futher developments and software update)

Creating a dynamic library with Rust.

Libraries are a collection of functions, types, variables, or other things depending on what your programming language supports, which are packaged up together to accomplish some functionalityso you won't need to re-implement it each time you want to use it.

For example, if you want to perform HTTP requests in Python, you might use the 'requests' library, or in C, you could use 'libcurl'.

It's much easier to import a library to make HTTP requests than it is to use raw sockets and read/write system calls.

Different programming languages have different formats for libraries. For example, Python libraries are simply collections of Python source code files, which the Python intepreter reads when imported.
In C, there are a few different types of libraries, but the most commonly used on Unix-like operating systems, andthe type that we'll focus on is the dynamic library.

We need to take SEVERAL steps before our Rust 'solve' function can be called from our C program.

1. Tell Cargo to compile our crate as a dynamic library that the C linker understands.
2. Add our newly created dynamic library to the linker search path.
3. Mark our Rust 'solve' function so that the Rust compiler knows to compile iyt with C calling conventions.
4. Recompile our C program using the 'solve' function from our Rust dynamic library.

Walking through the steps:

CREATING THE DYNAMIC LIBRARY.

When Cargo compiles a Rust crate, by default, it doesn't produce something that a C compiler knows how to use.It generates something called an 'rlib' file, which is a typ of file specific to the Rust compilerand only used as an intermediate artifact that will be later used in some other Rust compilation.

Instead of an rlib, we want Cargo to generate a dynamic library that the C linker knows how to use. We need to make another edit to our Cargo.toml file.
The time we will ytell it to output something compatible with C.   

Cargo can generate many different type of crates, but the most common are the default 'rlib' and the 'cdylib', which will cause Cargo to build a dynamic library compatible with native C programs.

Then, lets run Cargo build again...

    ......(here i encountered trouble, debugging for the rest of the day.Getting Cargo and the dynamic lib (cdylib) for those C pointers was a mission, all of a sudden my byuilds were bugged.... at last, they worked. Huge learnings)

MARKING THE SOLVE FUNCTION AS C-LINKABLE

Even though we asked Rust to compile the 'calculate' crate as a 'cdylib', it does not export every function and type in a C-compatible format. It only exports the specific functions and types that we ask it to. Three steps are required to make a Rust function callable from C. We need to:
- Disable name mangling
- Mark the function as public
- Tell the Rust compiler to use C calling conventions for the function.

Below is a Rust function that can be exported as compatible with C.

    #[no_mangle]
    pub extern "C" fn solve(
        line: *const c_char, solution: *mut c_int) -> c_int {
     0
        }  

What do the new ELEMENTS mean ?

1. #[no_mangle], is a function attribute macro, which instructs the compiler to not perform name mangling on this function.If you've done much C++ development, you may be familiar with the concept of name mangling.
If, not, name mangling refers to a process that the compiler uses to ensure that functions and type names are unique inside of a system library or executable.

On Unix-like systems, executables and system libraries d not have namespaces. Thus, if we define a solve function in our executable, there can only ever be a single solve function across all libraries there we're using and across all files.
If any library has an internal function called solve, it will conflict with the one we're trying to create.

To overcome this problem, the Rust compiler puts extra nformation into the name of the symbols within it, which ensures that no symbol names overlap.If we leave name manglingenabled, our Rust 'solve' function will be given 
a name like: _ZN9calculate5solve17h6ed798464632de3fE.
The method that the compiler uses to create these unique names is unimportant for our purposes here.
It suffices to know that predicting these mangled names is very difficult and unwieldly.Therefore, if we expect to call any Rust functions from C, which has no understanding of Rust's name-mangling scheme, we must uso 'no_mangle" to disable it for those specific functions.

The next new bit of code, (pub), is a very common Rust keyword. It tells the Rust compiler that the symbol should be exported outside of the module in which it is defined.
By default, all symbols in Rust are private and unexpected.The way to export a function or type is to add the (pub) keyword before its definition, as we have done in the example. 

Finally, we have 'extern "C"', which tells Rust to generat to generate the  'solve' function using C-compatible calling conventions.By default, the Rust compiler's calling conventions are not strictly compatible with C's.Rust suports a number of different calling conventions, but the most commonly used is the default Rust convention, followed by "C".

NB: what each oiece of syntax is responsible for.

1. Function attribute macro: ->  #[no_mangle]
                         this disables name mangling


2. Exports function (publicly)

                  pub extern "C" fn solve

                               [ Uses C calling convention] 

This is an anatomy of a C-compatible function declaration

RECOMPILING THE C PROGRAM AGAINST OUR RUST DYNAMIC LIBRARY


 

