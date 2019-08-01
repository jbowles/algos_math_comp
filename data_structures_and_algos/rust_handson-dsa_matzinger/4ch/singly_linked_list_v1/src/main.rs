use std::cell::RefCell;
use std::rc::Rc;

/*
Linked List: O(n)
Why a singly linked list?
    The Good:
        * grow in size cheaply; low overhead allocation per item
        * always maintain a direction; direction strictly enforced
        * allow acces to items individually
        * mutation will iterating is possible
        * fairly simple (though rust's is more complicated)
        * efficient operations (append, prepend, delete, insert)

    The Bad:
        * indexing not efficient; you have to look at every node to find specific value
        * iteration happens on the heap... lots of jumping around
        * reversing is very innefficient

    rust has std::collections::LinkedList
*/

fn main() {
    println!("Linked List");
    let mut tl = TransactionLog::new_empty();
    println!("New Empty: {:?}", tl);
    tl.append("First".to_string());
    println!("Append: {:?}", tl);
    tl.append("Second".to_string());
    tl.append("Third".to_string());
    println!("After third append: {:?}", tl);
    for i in 0..2 {
        println!("{} pop: {:?}", i + 1, tl.pop());
    }
    println!("End State: {:?}", tl);
}

type SingleLink = Option<Rc<RefCell<Node>>>;

// nodes are linked through next (see append, pop)
#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: SingleLink,
}
// head and tail are not linked
#[derive(Clone, Debug)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
}
/*
    NEED:
        append at end
        remove from front
*/
impl TransactionLog {
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            // the case where tail has an existing value
            // set next on current node to new node
            // the old node will link to next SingleLink
            Some(current) => current.borrow_mut().next = Some(new.clone()),
            // the case where tail has no value, set head to new
            // in this case log is empty and so head|tail have same value
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        //set tail to new
        self.tail = Some(new);
    }

    /*
    Calling this function in sequence returns the Node.value in the order they were inserted, providing a nice replay feature. For a real-world usage, it's important to provide the ability to serialize this state to disk as well, especially since this operation consumes the list entirely.
    */
    pub fn pop(&mut self) -> Option<String> {
        //take value leaving None
        self.head.take().map(|h| {
            //since head is None we need to check next linked node take()
            if let Some(next) = h.borrow_mut().next.take() {
                //if there set head to next
                self.head = Some(next);
            } else {
                //if nothing linked next to head take tail
                self.tail.take();
            }
            self.length -= 1;
            //Returns the contained value, if the Rc has exactly one strong reference.
            //Otherwise, an Err is returned with the same Rc that was passed in
            Rc::try_unwrap(h).expect("Cannot pop").into_inner().value
        })
    }
}
/*
After Use:
Whenever the list needs to be disposed of, Rust calls a drop() method that is automatically implemented. However, since this is an automated process, each member is dropped recursively—which works OK until the level of nested next pointers exceeds the stack for executing the drop() method and crashes the program with an unexpected stack overflow message.

As a consequence, it is a good idea for production usage to also implement the Drop trait and dispose of the list elements iteratively. By the way, a stack overflow also happens while using the derived Debug implementation to print a Node—for the same reason
*/
