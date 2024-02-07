use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Palpite");

    println!("Por Favor, insira o seu palpite.");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        
        let mut guess: String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
    
        println!("Seu palpite : {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {println!("Acertou!!"); break; },
        }
    }
}
