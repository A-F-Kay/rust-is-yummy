use std::io;

enum WrongNumber {
    TooBig,
    TooSmall,
}

enum GuessError {
    NaN,
    OutOfRange,
    WrongNumber(WrongNumber),
}

pub struct GuessGame {
    pub win_number: i32,
    pub guess_range: (i32, i32),
    pub hello_message: String,
}

impl GuessGame {
    pub fn play(&self) {
        self.say_hello();

        let mut input = String::new();

        loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading line");

            match self.make_guess(&input) {
                Ok(()) => {
                    println!("Nice one! You guessed it right! :)");
                    break;
                }
                Err(GuessError::OutOfRange) => {
                    println!("Out of range. Valid range: {}", self.range_str());
                    continue;
                }
                Err(GuessError::NaN) => {
                    println!("Please enter a valid number.");
                    continue;
                }
                Err(GuessError::WrongNumber(WrongNumber::TooSmall)) => {
                    println!("Too small value :) Try again.");
                    continue;
                }
                Err(GuessError::WrongNumber(WrongNumber::TooBig)) => {
                    println!("Try smaller value :3");
                    continue;
                }
            };
        }
    }

    fn say_hello(&self) {
        println!("{}", self.hello_message);
    }

    fn range_str(&self) -> String {
        format!("({}-{})", self.guess_range.0, self.guess_range.1)
    }

    fn make_guess(&self, input: &String) -> Result<(), GuessError> {
        let value = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err(GuessError::NaN),
        };

        if self.is_out_of_range(value) {
            Err(GuessError::OutOfRange)
        } else if value < self.win_number {
            Err(GuessError::WrongNumber(WrongNumber::TooSmall))
        } else if value > self.win_number {
            Err(GuessError::WrongNumber(WrongNumber::TooBig))
        } else {
            Ok(())
        }
    }

    fn is_out_of_range(&self, value: i32) -> bool {
        let (start, end) = self.guess_range;

        value < start || value > end
    }
}
