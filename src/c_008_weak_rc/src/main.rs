use std::rc::{Rc, Weak};
use std::cell::RefCell; 

#[derive(Debug)]
struct LinkedList<T>{
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    prev: RefCell<Option<Weak<Node<T>>>>,
    data: T
}

impl<T> LinkedList<T>{
    fn new() -> Self{
        LinkedList { head: None }
    }

    fn append(&mut self, data: T) -> Self{
        let new_node = Rc::new(Node{
            data: data,
            next: self.head.clone(),
            prev: RefCell::new(None)
        });

        match self.head.clone() {
            Some(node) => {
                let mut a = node.prev.borrow_mut();
                *a = Some(Rc::downgrade(&new_node))
            },
            None =>{

            }
        }

        LinkedList { head: Some(new_node) }

    }
}

fn main() {
    let a = LinkedList::new().append(1).append(2).append(3);
    println!("{:?}", a);
}
