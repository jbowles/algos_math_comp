/*
use std::cell::RefCell;
use std::rc::Rc;

//interior mutability
fn main(){}

//RefCell maintains single ownership of a value but allows mutable borrowing checked at runtime. Instead of compiler errors, violating the rules of borrowing will lead to a runtime panic!, crashing the program.
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

// this is not threadsafe; can only be accessed by 1 thread
// create new node, if tail match
pub fn append(&mut self, value: String) {
    let new = Rc::new(RefCell::new(Node::new(value))):
}
*/
