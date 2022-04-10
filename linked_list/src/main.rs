use std::cmp::PartialEq;
use std::fmt::Debug;

struct List<Value: Debug + PartialEq> {
    value: Value,
    next: Option<Box<List<Value>>>,
}

struct LinkedList<Value: Debug + PartialEq> {
    head: Option<List<Value>>,
    tail: Option<List<Value>>,
}


impl<Value: Debug + PartialEq> LinkedList<Value> {
    
    fn new () -> Self {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn insert(&mut self, value: Value) {
       self.head = Some(List {
            value,
            next: None
        });
    }

    fn debug_all(&self)  {
       let mut actualValue = self.head;
        
       println!("Valor {:?}", actualValue.unwrap().value);

       while actualValue.unwrap().next != None {
         actualValue = actualValue.next.unwrap(); 
       }

    }
   // fn get(&self, value: Value) -> Value {
        //let mut actualValue = self.head;


       // while actualValue.unwrap().value != value {
       //    actualValue = actualValue.next; 
      //  }

      //  return actualValue;
    //}
}

fn main() {
   let mut linked = LinkedList::<u32>::new();     
    linked.insert(5);

  // println!("Get value {}", linked.get(5));
}
