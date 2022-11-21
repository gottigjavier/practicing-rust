fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s es: {} ", s);
    
    let a = 2;
    //let ptr_a = &a;
    let x = &a;
    //let ptr_y = &x;
    println!("address a: {:p},  address x: {:p}", &a, &x);
    
    

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
