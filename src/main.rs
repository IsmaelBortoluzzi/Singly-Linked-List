#[path = "enums/linked_list.rs"] mod linked_list;
use linked_list::*;

fn it_works() {
    
    let list: LinkedList = LinkedList::new_empty_sll();

    println!("{}", list.element);

}

fn main() {

    it_works();

    println!("Hello, world!");

}
