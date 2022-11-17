//use std::f64;
use std::io;
use std::time::SystemTime;


fn main () {

    println!();
    println!("Determinar si un número es PRIMO o COMPUESTO");
    println!();
    
    loop { 
    
        let mut is_primo: bool = true;

        println!("-----------------------------------------------");
        println!("Ingrese un número entero (otra cosa para salir)");
        
        let mut n = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

            
            let n: u128 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
        
        let n_len = n.to_string().len();
        
        let sys_time = SystemTime::now();
        if n_len > 17 {
            println!();
            println!("El dato ingresado tiene {} cifras", n_len);
            println!();
            println!("Para números de más de 18 cifras que tienden a");
            println!("ser primos puede tardar más de un minuto.");
            println!("Por cada nueva cifra el tiempo de proceso se triplica");
        }
        println!("Procesando");
        println!();

        // Si n es primo, es suficiente demostrar que no es divisible por los
        // números primos menores a su raíz cuadrada. Ya que no se tienen,
        // se divide por todos los números menores a dicha raíz + 1 (por las dudas)
        let x: u128 = (n as f64).sqrt().trunc() as u128;

        let x = x + 1;

        println!("sqrt de x: {} ", &x);

        for i in 2..x {
            if n % i == 0 {
                is_primo = false;
                break;
            }
        }

        match is_primo {
            false => println!("El numero {n} es COMPUESTO"),
            true => println!("El numero {n} es PRIMO")
        }

        println!();

        let new_sys_time = SystemTime::now();
        let difference = new_sys_time.duration_since(sys_time)
        .expect("Clock may have gone backwards");
        println!("Duracion del proceso: {difference:?}");
    }
}