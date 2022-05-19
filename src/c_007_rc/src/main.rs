use std::rc::Rc;


#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    data: T,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
        }
    }
}

fn main() {
    let list_of_numbers= 
        LinkedList::new()
        .append(1)
        .append(2)
        .append(3);
    println!("{:?}", list_of_numbers);

    let list_of_string= 
        LinkedList::new()
        .append("Atul")
        .append("Sachin")
        .append("Om");
    println!("{:?}", list_of_string);
}
