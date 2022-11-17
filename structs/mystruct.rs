// https://doc.rust-lang.org/book/ch05-03-method-syntax.html

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// square devuelve una instancia de Rectangle. Funciona como una especie de constructor
// no es un método del struct sino una funcion asociada
// Por eso al instanciar se usa ::
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size * 2, // no necesariamente tiene que ser un cuadrado
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// A modo constructor de Rectangle
impl Rectangle {
    fn rec(w: u32, h: u32) -> Self {
        Self {
            width: w,
            height: h,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    let rec11 = Rectangle::rec(3,5);
    /* let rec1 = Rectangle {
        width: 30,
        height: 50,
    }; */
    println!("cuadrado, o no: {:#?} ", sq);
    println!("rectangulo constr: {:#?} ", rec11);
    println!("Area sq: {} ", sq.area());
    println!("Area rec11: {} ", rec11.area());
    dbg!(sq);
}
