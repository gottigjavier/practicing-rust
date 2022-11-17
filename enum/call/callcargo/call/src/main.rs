fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
        fn call(&self) {
                dbg!(self);
                println!("Message es {:#?}", self);
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move{x: 34, y: 56};
    println!("Message es aqui {:#?}", m);
    // DEbe llerse: si m se puede desestructurar, ejecutar el bloque
    // Se podría desestructurar individualmente: Message::Move {x, ..} = m
    if let Message::Move {x, y} = m {
        dbg!(x);
        dbg!(y);
    }
    let m = Message::Quit;
    m.call();
    let m = Message::ChangeColor(12, 34, 89);
    m.call();
    }
    