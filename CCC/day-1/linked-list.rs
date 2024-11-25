use std::cell::RefCell;
use std::rc::Rc;
use std::io;
type Link = Option<Rc<RefCell<Node>>>;
struct Node{
    data : i32,
    next: Link
}
struct LinkedList{
    head:Link,
}
impl LinkedList{
    fn new() -> Self{
        LinkedList{
            head:None
        }
    }
    fn add(&mut self, data: i32) {
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));
        match self.head {
            Some(ref head) => {
                let mut current = Rc::clone(head);        
                loop {
                    let next_option = current.borrow().next.clone(); 
                    if let Some(ref next) = next_option {
                        current = Rc::clone(next);
                    } else {
                        break; 
                    }
                }
                current.borrow_mut().next = Some(Rc::clone(&new_node));
            }
            None => {
                self.head = Some(new_node);
            }
        }
    }
    
    fn display(&self){
        let mut current = self.head.as_ref().map(Rc::clone);
        while let Some(node) = current {
            print!("{} -> ",node.borrow().data);
            current = node.borrow().next.as_ref().map(Rc::clone);
        }
        println!("None");
    }
}
fn main(){
    let mut list:LinkedList = LinkedList::new();
    let mut input:String = String::new();
    println!("Enter numbers to add to linked list (-1 to stop)");
    loop{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let value:i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };
        if value == -1{
            break;
        }
        list.add(value);
    }
    list.display();
}