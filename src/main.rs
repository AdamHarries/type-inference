use std::collections::LinkedList;

mod infer;

fn main() {
    println!("Hello, world!");

    let mut d = LinkedList::new();

    d.push_front(7);
    d.push_front(6);
    d.push_front(5);
    d.push_front(4);
    d.push_front(3);
    d.push_front(2);
    d.push_front(1);

    let mut splitted = d.split_off(2);
    d.pop_back();

    println!("Elements of d: ");
    for a in d.iter() {
        println!("a: {:?}", a);
    }

    println!("Elements of splitted: ");
    for a in splitted.iter() {
        println!("a: {:?}", a);
    }
}
