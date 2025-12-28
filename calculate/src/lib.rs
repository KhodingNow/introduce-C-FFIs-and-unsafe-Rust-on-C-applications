use libc::{c_char, c_int};
use std::ffi::CStr;
use std::collections::VecDeque;


// Updating the solve function to call the new evealuate func

#[derive(Debug)] 
struct RpnStack {
    stack: VecDeque<i32>,
}

enum Error {
    InvalidNumber,
    PopFromEmptyStack,    
}

impl RpnStack {
    fn new() -> RpnStack {
       RpnStack {
        stack: VecDeque::new(),
       }
    }
    fn push(&mut self, value: i32) {
        self.stack.push_front(value);
    }

    fn pop(&mut self) -> Result<i32, Error> {
        match self.stack.pop_front() {
            Some(value) => Ok(value),
            None => Err(Error::PopFromEmptyStack),
        }
    }
}


fn evaluate(problem: &str) -> Result<i32, Error> {
    let mut stack = RpnStack::new(); // new data structure
        /*println!("problem: {:p}", problem.as_ptr());
        
        checks IF memory is borrowed from the C stack as 
        per &str
        */

    for term in problem.trim().split(' ') {
        //println!("problem: {:?}", term.as_ptr()); 
        
       // println!("Term - {:?}", term);
        
        match term {
        
          "+" => {

            let y = stack.pop()?;
            let x = stack.pop()?;

            stack.push(x + y);
              
          }
            "-" => println!("SUB"),
            "*" => println!("MUL"),
            "/" => println!("DIV"),
                
          // other => match other.parse::<i32>(),
            
            other => match other.parse() {
                Ok(value) => stack.push(value),
              // println!("STACK: {:?}", stack);
                
                Err(_) => return Err(Error::InvalidNumber),
                }
            }
        }        
  
    
    let value = stack.pop()?;
    Ok(value)
}


#[no_mangle]

pub extern "C" fn solve(line: *const c_char, solution: *mut c_int) -> c_int {
      if line.is_null() || solution.is_null() {
         return 1;
        } // returning a Cstr AND the solution in int
        
        let c_str = unsafe { CStr::from_ptr(line) };
        let r_str = match c_str.to_str() {
            Ok(s) => s,            
            Err(e) => {
                eprintln!("UTF-8 Error: {}", e);
                return 1;
            },
        };

        match evaluate(r_str) {
            Ok(value) => {
                unsafe {
                    *solution = value as c_int;
                }
                0       
            }
            Err(e) => {
                eprintln!("Error");

                1
            }
        }

   /*  println!("line: {}", r_str);
        
        println!("r_str.as_ptr(): line: {:p}", 
        //r_str.as_ptr(), line);
        
         unsafe {
            *solution = 1024;

        }
                    
       0

       */
}



