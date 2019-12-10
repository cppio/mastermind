mod code;
mod guesser;

use guesser::CodeGuesser;

use std::io;

fn main() {
    let mut guesser = CodeGuesser::new();

    for guesses in 1.. {
        println!("Guess: {:01$}", u32::from(guesser.guess()), code::PEGS as usize);

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let reply: u8 = buf.trim().parse().unwrap();
        let reply = (reply / 10 % 10, reply % 10);

        if reply == (code::PEGS, 0) {
            println!("I guessed it in {} guesses!", guesses);
            break;
        }

        guesser.filter(reply);
    }
}
