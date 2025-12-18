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

[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x06, 0x07]

0       1     2     3      4     5     6     7    8

 - no code has been executed , so the stack is EMPTY.

    int main() {
        char x = 'a';
        char *y = &x;

        char z = *y;
    }

[ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07 ]

 'a'      1     2     3     4     5     6      7     8

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


