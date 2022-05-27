#[path = "enums/linked_list.rs"] mod linked_list;
use linked_list::*;

fn it_works() {
    
    let mut list: LinkedList = LinkedList::new_empty_sll();
    list.push(10);
    list.push(20);

    println!("{}", list.head.as_deref().unwrap().element);
    println!("{}", list.head.as_deref().unwrap().next.as_deref().unwrap().element);
    list.pop_head();
    println!("{}", list.head.as_deref().unwrap().element);
    // println!("{}", list.head.as_deref().unwrap().next.as_deref().unwrap().element);  // panicks here
    println!("{}", *(list.peek().unwrap()));

}

fn main() {

    it_works();

    println!("Hello, world!");

}
