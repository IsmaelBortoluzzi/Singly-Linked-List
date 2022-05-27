pub struct LinkedList {
    pub head: Link, // Link = Option<Box<Node>>;
}

impl LinkedList {
    pub fn new_empty_sll() -> LinkedList {
        LinkedList {
            head: None,
        }
    }

    pub fn push(&mut self, new_element: i32) {

        let old_head = std::mem::replace(&mut self.head, None);

        match old_head {
            None => { 
                self.head = Some(Box::new(Node {
                    element: new_element,
                    next: None,
                }));
            }

            Some(n) => {
                
                let new_head = Some(Box::new(Node {
                    element: new_element,
                    next: Some(n),
                }));

                self.head = new_head;
            }
        }


        // O jeito melhor seria:

        /* 
        
        let old_head = self.head.take()

         let new_head = Box::new(Node {
             element: new_element,
             next: old_head,
         });

         self.head = Some(new_head);
        
        */

    }

    // fn append(&mut self, new_element: i32) {
    //     match self.head {
    //         None => {
    //             self.head = Some(Box::new(Node {
    //                 element: new_element,
    //                 next: None,
    //             }));
    //         }

    //         Some(n) => {
    //             let mut aux = self.head.as_deref().unwrap().next.as_deref().unwrap();  // we already know head is not None
    //             while self.head.as_deref().unwrap().next.is_some() {
    //                 aux = aux.next.as_deref().unwrap();
    //             }
    //         }
    //     }
    // }


    pub fn pop_head(&mut self) -> Option<i32> {
        let old_head = self.head.take();

        match old_head {
            None => { 
                None 
            }
            
            Some(n) => {
                self.head = n.next;
                Some(n.element)
            }
        }

        // O jeito melhor
        
        // self.head.take().map(|n|, {
        //     self.head = n.next;
        //     n.element
        // })
    }

}



// PRIVATE

pub type Link = Option<Box<Node>>;  // Tipo o typedef

pub struct Node {
    pub element: i32, 
    pub next: Link,
}
