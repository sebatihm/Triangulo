use std::io;
fn main() {
    println!("Bienvenido al programa del triangulo");
    println!("El programa acepta 3 enteros como entradas: a, b, y c. Estos serán los lados de un triángulo.");

    let a : i32 = input();
    let b : i32 = input();
    let c : i32 = input();

    if (a < (b + c))&& (b < (a + c))&&(c < (a + b)){
        
        if a == b && b == c{
            println!("Equilatero");
        }else if a == b || b == c || a ==c{
            println!("Isoceles");
        }else{
            println!("Escaleno")
        }

    }else{
        println!("No es un triangulo");
    }





}

fn input() -> i32{
    
    loop {
        println!("Ingrese la variable (Rango 1 - 200) : ");
        let mut result: String = String::new(); 
        io::stdin()
            .read_line(&mut result)
            .expect("Salio mal")
        ;

        let result: i32 =  match result.trim().parse() {
            Ok(num) =>  num,
            Err(_) => -1

        };

        if result >= 1 && result <= 200 {
            return result;
        }else {
            println!("Ingrese datos dentro del rango valido");
        }
    }

}

