use std::io;
use rand::Rng;

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let numero: i32 = rng.gen_range(0..=100);

    loop {
        println!("Digite um número de 0 a 100: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)?;

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, digite um número válido!");
                return Ok(());
            }
        };

        println!("Você digitou: {}", guess);

        if guess < numero {
            println!("Muito baixo! Tente novamente");
        } else if guess > numero {
            println!("Muito alto! Tente novamente");
        } else {
            println!("Parabéns! Você acertou");
            break;
        }
    }   
    Ok(())
}
