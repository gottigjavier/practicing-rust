
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        if let Some(i) = x {
            Some(i + 1)
        } else {
            None
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five.unwrap()); // valor de T. Devuelve 5 porque no pasa por la función
    dbg!(six.unwrap_or(0)); // valor de T ó 0 si es None
    dbg!(none.unwrap_or(0)); // valor de T ó 0 sies None. En este caso en None
    }
    