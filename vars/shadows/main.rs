fn five() -> i32{
    return 5
}

fn main() {
    let u: i32 = five();
    let v = {
        let u = 3;
        u * 2
    };
    
    println!("The value of v is: {v}");

    println!("The value of u is: {u}");

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    println!("The value of x is: {x}");

    println!("The value of y is: {y}");
    
    println!("The value of z is: {z}");
}