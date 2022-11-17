//#![allow(unused)]
fn main() {
let x = 2;

match x {
    e @ 1 ..= 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
}
/* 
fn main() {
    let x = "hello";
    
    
    match x {
        e @ "hello" => println!("got a range element {}", e),
        _ => println!("anything"),
    }
    
    println!("x es {}", x);
    }
 */