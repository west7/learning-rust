use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=500);
    
    loop {

        println!("Advinhe o número: ");
        
        let mut input = String::new();
        
        io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada");
    
        let input: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu chute foi: {input}");

        match input.cmp(&secret_number) {
            Ordering::Greater => println!("Menor que isso!\n"),
            Ordering::Less => println!("Maior que isso!\n"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
