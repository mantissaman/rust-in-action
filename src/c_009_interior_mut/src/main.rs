use std::cell::Cell;
use std::cell::RefCell;

#[derive(Debug)]
struct Bag {
    item: Box<u32>,
}

fn main() {
    let bag = RefCell::new(Bag{item: Box::new(1)});
    let hand1 = &bag;
    let hand2 = &bag;

    *hand1.borrow_mut() = Bag { item: Box::new(2)};
    *hand2.borrow_mut() = Bag { item: Box::new(3)};

    let x = hand1.borrow();

    println!("{:?}", x);
}
