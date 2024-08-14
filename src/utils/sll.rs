
use std::io;


pub struct Node{
    pub value : i64,
    pub next: Option<Box<Node>>

}

pub struct SSL{
    pub head : Option<Box<Node>>
}

impl SSL{

    pub fn new() -> Self{
        SSL { head: None }
    }

    pub fn append(&mut self, val : i64){
        let mut new_node: Option<Box<Node>> = Some(Box::new(Node{
            value : val, next : None
        }));
        if self.head.is_none(){
            self.head = new_node;
            return;
        }
        let mut current = self.head.as_mut().unwrap();

        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        current.next = new_node;
    }

    pub fn print(&mut self){
        if self.head.is_none(){
            return;
        }
        let mut current = self.head.as_mut().unwrap();
        while current.next.is_some(){
            println!("{}", current.value);
            current = current.next.as_mut().unwrap();
        }
        println!("{}", current.value);
    }
}


fn main(){
    let mut list = SSL::new();
    list.append(5);
    list.print();

}

