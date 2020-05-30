/*
 *   Copyright (c) 2020 Goutham Krishna K V <gauthamkrishna9991@live.com>
 *   All rights reserved.

 *   Permission is hereby granted, free of charge, to any person obtaining a copy
 *   of this software and associated documentation files (the "Software"), to deal
 *   in the Software without restriction, including without limitation the rights
 *   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *   copies of the Software, and to permit persons to whom the Software is
 *   furnished to do so, subject to the following conditions:
 
 *   The above copyright notice and this permission notice shall be included in all
 *   copies or substantial portions of the Software.
 
 *   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *   SOFTWARE.
 */

// This is used to derive printing properties for vec into Stack.
#[derive(Debug)]
struct Stack {
    elements: Vec<isize>,
    length: usize
}

// Implementation Methods for struct Stack: This is similar to methods in C++.
impl Stack {
    // Create a stack structure.
    fn create() -> Stack {
        Stack {
            elements: Vec::new(),
            length: 0
        }
    }

    // Push an integer into the stack.
    fn push(&mut self, insert_element: isize) {
        self.length += 1;
        self.elements.push(insert_element);
    }

    // Function to pop an element. We have taken care of both cases
    fn pop(&mut self) -> Result<isize, String> {
        // Pop from vector
        match self.elements.pop() {
            // If an element is popped:
            Some(x) => {
                // Reduce the length.
                self.length -= 1;
                // Return an 'Ok' result along with the element back.
                return Result::Ok(x);
            }
            // If no element is popped, return an `Err` result of type String.
            None => return Result::Err(String::from("stack is empty"))
        }
    }

    // Function to return the topmost element
    fn top(&self) -> Result<isize, String> {
        // get the last element (last pushed) and match with following:
        match self.elements.get(self.length - 1) {
            // If element exists, dereference the element from
            // the vector and pass it as result.
            Some(&x) => Result::Ok(x),
            // If element does not exist, trigger `Err`.
            None => Result::Err(String::from("stack is empty"))
        }
    }

    // Function to determine if the stack is empty or not.
    fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    // Function to return the size of the stack
    fn size(&self) -> usize {
        self.length
    }
}

fn show_stack(s : &mut Stack) {
    // While the stack is not empty, do:
    while !s.is_empty() {
        // Try to pop an element. When you try it, you need to cover all cases.
        match s.pop() {
            // If the element exists (expected output).
            Ok(x) => print!("\t{}", x),
            // If the element does not exist (unexpected output).
            Err(error) => println!("Error when popping : {}", error)
        }
    }
    // Put a new line to seperate outputs.
    print!("\n");
}

fn main() {
    // Create a mutable stack.
    let mut s = Stack::create();
    // Push some elements
    s.push(10);
    s.push(30);
    s.push(20);
    s.push(5);
    s.push(1);
    // Print the stack.
    println!("STACK: {:?}", s);
    // Query the length of the stack.
    println!("SIZE: {}", s.size());
    // Try to access the top value.
    match s.top() {
        // On successful access: print the element.
        Ok(x) => println!("TOP: {}", x),
        // On failed access: do nothing.
        Err(_) => { }
    }
    print!("POPPED ELEMENTS: ");
    // Print all elements inside the stack.
    show_stack(&mut s);
}