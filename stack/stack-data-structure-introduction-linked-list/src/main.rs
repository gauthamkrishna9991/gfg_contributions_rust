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

/* 
    The Structure for StackNode:
        - data : the member which holds the data,
        - next : the member which holds the next element
*/
struct StackNode {
    /*
        isize is a special integer in rust which scales 
        based on the target architecture
    */
    data: isize,
    /*
        next is stored as Option<Box<T>>.
        Option handles nullability elegantly, Box<T> is a smart pointer.
    */
    next: Option<Box<StackNode>>
}

impl StackNode {
    /*
        Implementing constructor for StackNode. In Rust there's no 
        concept of classes so a function builds the structure,
        though methods can be implemented associated to a structure.
        To return an element we either write `return ele;` or `ele`
        as in Rust, both mean the same.
    */
    fn create(ele: isize, next_ele: Option<Box<StackNode>>) -> StackNode {
        StackNode {
            data: ele,
            next: next_ele
        }
    }
}

/*
    Stack class.
*/
struct Stack {
    /*
        Here the top is of type Option<Box<StackNode>>,
        where the Option handles nullability elegantly
        and Box<T> is a smart-pointer.
    */
    top: Option<Box<StackNode>>,
    length: usize
}

impl Stack {
    /*
        Constructor function for Stack.
    */
    fn create() -> Stack {
        /*
            We create a stack with an empty top
            and length as 0.
        */
        Stack {
            top: Option::None,
            length: 0
        }
    }

    /*
        is_empty function for Stack.
    */
    fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    /*
        push an integer element into a Stack.
    */
    fn push(&mut self, x: isize) {
        // increase the size by 1
        self.length += 1;
        /*
            We transfer ownership of self.top to
            the `match` construct, using the take()
            method. After this function, self.top does
            not contain the top element, we transferred
            it's ownership to the body of `match`.
            Also, we can see how elegantly both the cases are
            managed in Rust.
        */
        match self.top.take() {
            /*
                If the top element is None, there's
                nothing to own. So the new element is
                created with None as it's next element.
            */
            None => {
                self.top = Option::from(
                    Box::new(
                        StackNode::create(x, Option::None)
                    ) // Box::new
                ); // Option::from
            },
            Some(t) => {
                /*
                    Remember, we transferred the ownership
                    of self.top, to `match`, it is passed to
                    Some(t). We'll pass the new passed
                    ownership to the updated self.top which
                    would now owns the old self.top, as the
                    `next` element of the new StackNode created.
                */
                self.top = Option::from(
                    Box::new(
                        StackNode::create(x, Option::Some(t))
                    // Ownership of old top passed here - ^
                    ) // Box::new
                ); // Option::from
            }
        }
    }

    /*
        pop an element, along with handling all errors for it.
    */
    fn pop(&mut self) -> Result<isize, String> {
        /*
            We transfer the ownership of the object of self.top,
            to the `match` construct to elegantly manage all cases
            of the `top` element.
            We'll manage the taken value inside the construct.
            
        */
        match self.top.take() {
            /*
                If self.top is None, then return an error.
            */
            None => {
                Err(String::from("Stack Empty."))
            }, // case : None
            /*
                Remember, now that we have defined Some(x),
                the top element is now owned by some symbol x.
                Here, we update self.top with the `next`
                of `x`. 
                The ownership of x.next is changed to
                self.top, and the data existed in `x` is 
                returned and the element `x` of type Box<T> 
                is automatically deallocated as it 
                leaves it's scope.
                The x.data is now owned by Result::Ok now,
                which is returned and would now be owned
                by the calling funcion (main in this case).
            */
            Some(x) => {
                self.length -= 1;
                self.top = x.next;
                Result::Ok(x.data)
            } // case : Some(x)
        } // match self.top.take()
    }

    /*
        peek into top of the stack
    */
    fn top(&self) -> Result<isize, String> {
        /* borrow top of the stack as immutable */
        match &self.top {
            /* if no element exists: */
            None => {
                /* return error of "Stack Empty" */
                Err(String::from("Stack Empty"))
            }, // case: None
            Some(t) => {
                /*
                    Return the data variable back
                    on successful access of element.
                */
                Ok(t.data)
            } // case : Some(t)
        }
    }
    
    /*
        print the stack given
    */
    fn print_stack(&mut self) {
        /*
            Borrow the element self.top for the lifetime 
            of match construct as `immutable` to read from it.
        */
        match &self.top {
            /*
                If self.top is None, the stack is empty.
                print "Empty Stack" & return back to the
                calling function.
            */
            None => {
                println!("Empty Stack");
            } // case: None
            /*
                For the case Some(top_ele) for some top_ele,
                we do the following:
            */
            Some(top_ele) => {
                print!("STACK: {{");
                /*
                    Assign current as the top - element.
                */
                let mut cur = top_ele;
                /*
                    loop until a break statement is encountered.
                */
                loop {
                    /* print the current data */
                    print!(" {}", cur.data);
                    /*
                        borrow an immutable reference to next,
                        and analyze:
                    */
                    match &cur.next {
                        /*
                            If the next element exists, we 
                            reassign cur to the next element,
                            and let the loop continue.
                        */
                        Some(next_ele) => {
                            cur = next_ele
                        }, // case: Some(next_ele)
                        /*
                            If the next element does not exist,
                            we break the loop, thereby continuing the execution.
                        */
                        None => break
                    } // match &cur.next
                } // loop
                print!(" }}\n");
            } // case: Some(top_ele)
        } // match &self.top
    }

    fn size(&self) -> usize {
        // For size(), return the length element back.
        self.length
    }
}

fn show_stack(s : &mut Stack) {
    print!("STACK :");
    /* while stack is not empty, do: */
    while !s.is_empty() {
        /* pop the stack and analyze */
        match s.pop() {
            /* 
                on `OK` result, for some element x:
            */
            Ok(x) => {
                // print x
                print!("\t{}", x)
            }, // case : Ok(x)
            /* on an `Err` result */
            Err(_) => {
                print!("\tERR");
            } // case : Err(_)
        } // match s.pop()
    } // while
    print!("\n");
}


fn main() {
    // Create a stack (mutability should be mentioned
    // specifically in Rust).
    let mut root = Stack::create();
    // Push some elements.
    root.push(10);
    root.push(20);
    root.push(30);
    root.push(5);
    root.push(1);
    // Print the stack
    root.print_stack();
    // Output Size of the stack.
    println!("SIZE: {}", root.size());
    // Show the top element.
    match root.top() {
        Ok(t) => {
            println!("TOP: {}", t);
        },
        Err(error_message) => {
            println!("ERROR: {}", error_message);
        }
    }
    show_stack(&mut root);
}
