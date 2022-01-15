use std::ops::Deref;
use std::rc::Rc;
use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
fn hello(str1:&str){
    println!("Hello {}",str1)
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
/*   let x = 5;
    let y = MyBox::new(x);
    let z = y.deref();
    assert_eq!(x,*z);
    let k = MyBox::new(String::from("ketan mer"));
    hello(&**k) */// hello(&k)  or hello(&(*m)[..]) both are valid

 /*   let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");*/

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
