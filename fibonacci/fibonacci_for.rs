use std::io;

fn fibonacci(n: u8) -> u128 {

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    
    match n {
        0 => a,
        1 => b,
        _other =>  {
            // Dado que el for va hasta nn-1 solo se resta 1
            let nn = n - 1; 
            for _i in 0..nn {
                        c = a + b;
                        a= b;
                        b= c;
                    }
            c
        }
    }
}

fn main() {

    loop {
        println!("-----------------------------------------------");
        println!("Ingrese un número entero (otra cosa para salir)");
        
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u8 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        if n < 187 {
            println!("El numero final de la serie de Fibonacci para el numero {n} es: {} ", fibonacci(n));
        } else {
            println!("Lo numeros de entrada superiores a 186 arrojan");
            println!("numeros de fibonacci superiores al limite de un u128");
            println!("que es: {} ", u128::MAX);
        }
    }
}