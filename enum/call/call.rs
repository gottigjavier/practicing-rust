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
        }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Move{x: 34, y: 56};
    if let Message::Move {x, ..} = m {
        dbg!(x);
    }
    if let Message::Move {y, ..} = m {
        dbg!(y);
    }
//    dbg!(b);
    let m = Message::Quit;
    m.call();
    let m = Message::ChangeColor(12, 34, 89);
    m.call();
    }
    