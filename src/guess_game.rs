pub enum WrongNumber {
    TooMuch,
    TooLess,
}

pub enum GuessError {
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
    pub fn say_hello(&self) {
        println!("{}", self.hello_message);
    }

    pub fn range_str(&self) -> String {
        format!("({}-{})", self.guess_range.0, self.guess_range.1)
    }

    pub fn make_guess(&self, input: &String) -> Result<(), GuessError> {
        let value = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err(GuessError::NaN),
        };

        if self.is_out_of_range(value) {
            Err(GuessError::OutOfRange)
        } else if value < self.win_number {
            Err(GuessError::WrongNumber(WrongNumber::TooLess))
        } else if value > self.win_number {
            Err(GuessError::WrongNumber(WrongNumber::TooMuch))
        } else {
            Ok(())
        }
    }

    fn is_out_of_range(&self, value: i32) -> bool {
        let (start, end) = self.guess_range;

        value < start || value > end
    }
}
