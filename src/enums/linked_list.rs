pub struct LinkedList {
    pub head: Link,
}

impl LinkedList {
    pub fn new_empty_sll() -> LinkedList {
        LinkedList {
            head: None,
        }
    }

    pub fn push(&mut self, element: i32) {
        match self.head {
            None => { 
                self.head = Some(Box::new(Node {
                    element: element,
                    next: None,
                }))
            }

            Some(n: Box<Node>) => {
                
                let new_head = Some(Box::new(Node {
                    element: element,
                    next: n,
                }));

                self.head = new_head;
            }
        }
    }

}



// PRIVATE

type Link = Option<Box<Node>>;  // Tipo o typedef

struct Node {
    element: i32, 
    next: Link,
}
