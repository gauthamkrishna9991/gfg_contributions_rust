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

fn main() {
    /*
    We make a mutable vector which can hold integers.
    */
    let mut s : Vec<isize> = Vec::new();
    // Push some elemeents into the vector.
    s.push(20);
    s.push(30);
    s.push(40);
    s.push(5);
    s.push(1);
    /*
    Print the stack (it looks in reverse as the elements
    are inserted to the end of the vector. This doesn't
    affect its operations.)
    */
    println!("STACK: {:?}", s);
    // Print Size of the Element.
    println!("SIZE: {}", s.len());
    // Print the top element (last element in s)
    match s.get(s.len() - 1) {
        Some(top_ele) => {
            println!("TOP : {}", top_ele);
        }, // case : Some(some_ele)
        None => {
            /*  
            Since this doesn't execute we keep it blank but 
            Rust compiler forces us to remind ourselves of the 
            existence of edge cases so that we can take action 
            accordingly.
            */
        }
    } // match s.get(s.len() - 1)
    print!("POPPED ELEMENTS: ");
    while !s.is_empty() {
        match s.pop() {
            Some(x) => {
                print!("\t{}", x);
            },
            None => {
                print!("\tERR");
            }
        }
    }
    print!("\n");
}
