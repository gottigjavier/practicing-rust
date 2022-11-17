use std::io;
//use std::any::type_name;


//fn type_of<T>(_: T) -> &'static str {
//    type_name::<T>()
//}

// celcius to fahrenheit
fn c_to_f (c: f64, term: f64, fact: f64) -> f64 {
    c * fact + term
}

// fahrenheit to celcius
fn f_to_c (f: f64, term: f64, fact: f64) -> f64 {
    (f - term) / fact
}

// choose convert
fn main() {

    let term: f64 = 32.0;
    let fact: f64 = 1.8;
    let result;

    println!("Conversion de temperatura");
    println!("Elija el tipo de conversion");
    println!("Celcius a Fahrenheit: f - Fahrenheit a Celcius: c");

    let mut choose = String:: new();
    let mut temp= String:: new();
    let c: String = "c".to_string();
    let f: String = "f".to_string();

    io::stdin()
        .read_line(&mut choose)
        .expect("No se pudo leer su eleccion");

        // trim_end() boora el final de línea \n
        choose = (&choose.trim_end()).to_string();

        //println!("choose es de tipo: {},    valor de choose: {choose}", type_of(&choose));

        //println!("c es de tipo  {} el valor es: {c} ", type_of(&c));

        //println!("Match de &choose == &c :  {}", (choose == c));

    if choose == c {
        println!("ingrese la temperatura en Fahrenheit");

        io::stdin()
        .read_line(&mut temp)
        .expect("No se pudo leer la temperatura en Fahrenheit");

        let temp: f64 = temp
            .trim()
            .parse()
            .expect("Por favor, ingrese un número");

        result = f_to_c(temp, term, fact);
        println!("");
        println!("{temp} Fº son {result} Cº");

    } else if choose == f {
        println!("ingrese la temperatura en Celcius");

        io::stdin()
        .read_line(&mut temp)
        .expect("No se pudo leer la temperatura en Celcius");

        let temp: f64 = temp
            .trim()
            .parse()
            .expect("Por favor, ingrese un número");

        result = c_to_f(temp, term, fact);
        println!("");
        println!("{temp} Cº son {result} Fº");

    } else {
        println!("Error de eleccion");
    }


} 