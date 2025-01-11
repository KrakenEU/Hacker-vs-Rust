use std::{io::{self, stdin, Read}};
use std::cmp::Ordering;
// cargo add rand
use rand::Rng;

fn game(correcto: Vec<u32>) -> u32 {
    let mut n: u32 = 0;

    while n < 3 {
        println!("Cual es tu numerin?");
        let mut intento = String::new();
        stdin()
            .read_line(&mut intento)
            .expect("Error leyendo numerin");

        // casting + manejo de errores 
        let intento: u32 = match intento.trim().parse(){
            Ok(o) => o,
            Err(e) => {
                println!("Error: {e}");
                continue;
            },
        };
        
        match intento.cmp(&correcto[n as usize]) {
            Ordering::Less => println!("Mas alto perro"),
            Ordering::Greater => println!("Mas bajo gato"),
            Ordering::Equal => {
                println!("Eso es loquete");
                n+=1;
            } 
        }
    }
    println!("Has ganado!");

    0

}

fn main() {
    
    let name = "Kr4K3n";

    println!("Hello {}", name.to_ascii_lowercase());

    let mut entrada = String::new();
    println!("Como te encuentras hoy: ");
    
    // input del usuario
    stdin()
        .read_line(&mut entrada)
        .expect("Error al leer input");

    println!("No me importa que estes {}", entrada);

    println!("Que comience el juego de adivinar numeritos");
    
    let mut correcto = Vec::new();
    for _ in 1..4 {
        correcto.push(rand::thread_rng().gen_range(1..=10));
    }

    // debuig hint
    println!("Hint {correcto:?}");

    let estado = game(correcto);

    if estado == 0 {
        println!("Hasta pronto");
    }


}
