use colored::Colorize;
use core::num;
use rand::{rngs::ThreadRng, Rng};
use std::{
    io::{self, Write},
    rc::Rc,
    thread,
    time::Duration,
};

fn main() {
    let mut slot = SlotMachine::new(500);
    slot.print_ui();
    println!("Spin slot: (y/n) !");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            println!("Thanks for playing!");
            // Exit the game loop
            std::process::exit(0);
        }

        slot.spin();
        println!("Spin slot again: (y/n) !");
    }
}

struct Reel {
    number: i32,
}
impl Reel {
    fn advance(&mut self) {
        self.number = (self.number + 1) % 10;
    }
}

struct SlotMachine {
    rng: ThreadRng,
    reels: Vec<Reel>,
    balance: u64,
}

impl SlotMachine {
    fn new(balance: u64) -> SlotMachine {
        let mut rng = rand::thread_rng();
        let reels: Vec<Reel> = vec![
            Reel {
                number: rng.gen_range(0..=9),
            },
            Reel {
                number: rng.gen_range(0..=9),
            },
            Reel {
                number: rng.gen_range(0..=9),
            },
        ];

        SlotMachine {
            rng,
            reels: reels,
            balance,
        }
    }
    fn add(a: i32, b: i32) -> i32 {
        (a + b) % 10
    }

    fn spin(&mut self) {
        self.balance -= 1;
        self.print_ui();
        self.update_reel_ui(3);
        let win = self.rng.gen::<f64>() <= 0.2;
        let reel_1_spin_number = self.rng.gen_range(10..=45);
        let mut reel_2_spin_number = self.rng.gen_range(10..=55);
        let mut reel_3_spin_number = self.rng.gen_range(5..=55);
        if win {
            let win_number = SlotMachine::add(self.reels[0].number, reel_1_spin_number);
            reel_2_spin_number = reel_2_spin_number + win_number
                - SlotMachine::add(
                    self.reels[1].number,
                    reel_2_spin_number + reel_1_spin_number,
                );
            reel_3_spin_number = reel_3_spin_number + win_number
                - SlotMachine::add(
                    self.reels[2].number,
                    reel_2_spin_number + reel_1_spin_number + reel_3_spin_number,
                );
        }

        for _ in 0..reel_1_spin_number {
            self.reels[0].advance();
            self.reels[1].advance();
            self.reels[2].advance();
            self.update_reel_ui(3);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        for _ in 0..reel_2_spin_number {
            self.reels[1].advance();
            self.reels[2].advance();
            self.update_reel_ui(2);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(150));
        }
        for _ in 0..reel_3_spin_number {
            self.reels[2].advance();
            self.update_reel_ui(1);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
        }
        self.print_ui();
        if self.result() {
            self.balance += 1000;
            self.update_reel_ui(5);
            println!("you win {}", "$1000".green())
        } else {
            self.update_reel_ui(4);
        }
    }
    fn numbers(&self) -> (i32, i32, i32) {
        (
            self.reels[0].number,
            self.reels[1].number,
            self.reels[2].number,
        )
    }
    fn result(&self) -> bool {
        let numbers = self.numbers();
        numbers.0 == numbers.1 && numbers.0 == numbers.2
    }
    fn print_ui(&self) {
        let numbers = self.numbers();
        print!("\x1B[2J"); // clear all line
        print!("\x1B[1;1H"); // cursor go to top left

        print!("______________\n");
        print!("|  {} | {} | {} |\n", numbers.0, numbers.1, numbers.2);
        print!("--------------\n");

        print!("{} {}\n\n", "$".green(), self.balance.to_string().green());
    }
    fn update_reel_ui(&self, changes: i32) {
        print!("\r\x1B[4A\x1B[K");
        let numbers = self.numbers();
        if changes == 5 {
            print!(
                "|  {} | {} | {} |",
                numbers.0.to_string().green(),
                numbers.1.to_string().green(),
                numbers.2.to_string().green()
            );
        } else if changes == 4 {
            print!(
                "|  {} | {} | {} |",
                numbers.0.to_string().red(),
                numbers.1.to_string().red(),
                numbers.2.to_string().red()
            );
        } else if changes == 3 {
            print!(
                "|  {} | {} | {} |",
                numbers.0.to_string().yellow(),
                numbers.1.to_string().yellow(),
                numbers.2.to_string().yellow()
            );
        } else if changes == 2 {
            print!(
                "|  {} | {} | {} |",
                numbers.0.to_string(),
                numbers.1.to_string().yellow(),
                numbers.2.to_string().yellow()
            );
        } else if changes == 1 {
            print!(
                "|  {} | {} | {} |",
                numbers.0.to_string(),
                numbers.1.to_string(),
                numbers.2.to_string().yellow()
            );
        }

        print!("\r\x1B[4B");
    }
}
