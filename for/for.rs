fn main() {
    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let mut s = String::with_capacity(10);

    s.push_str("baraaaaaaaaaaaaaaaaaqqqq");

    println!("s len {}", &s.len());
    println!("s capacity {}", &s.capacity());
    println!("s es {}", &s);

    dbg!(assert_eq!(24, s.len()));

    for _ in 0..1 {
        s.push('z');
    }


    println!("ss len {}", &s.len());
    println!("ss capacity {}", &s.capacity());
    println!("ss es {}", &s);

    s = String::with_capacity(30);

    println!("sss len {}", &s.len());
    println!("sss capacity {}", &s.capacity());
    println!("sss es {}", &s);

    s = "hello world rrrrrr".to_string();

    println!("hello len {}", &s.len());
    println!("hello capacity {}", &s.capacity());
    println!("s es {}", &s);

    let t: &str = "@";

    println!("t es {}", t);
    println!("len de t es {}", t.len());
    println!("t puntero {:#?}", t.as_ptr());
    dbg!(t.as_ptr());
    let bytes = t.as_bytes();
    dbg!("bytes es", bytes);
    println!("bytes t es {:#?}", bytes)
}