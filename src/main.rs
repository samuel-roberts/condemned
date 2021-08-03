
struct State {
    secret_word: String,
    guessed_word: String,
    lives: i32
}

impl State {
    fn new(secret_word: &str) -> State { 
        State {
            lives: 8,
            secret_word: secret_word.to_uppercase(),
            guessed_word: std::iter::repeat("*").take(secret_word.len()).collect::<String>()
        }
    }

    fn get_lives(&self) -> i32 {
        self.lives
    }

    fn get_guess(&self) -> &str {
        &self.guessed_word
    }

    fn get_answer(&self) -> &str {
        &self.secret_word
    }

    fn get_has_won(&self) -> bool {
        self.secret_word == self.guessed_word
    }

    fn decremet_lives(&mut self) {
        self.lives -= 1;
    }

    fn guess(&mut self, guess: char) -> bool {
        let guess_str = String::from(guess).to_uppercase();
        let mut success: bool = false;

        for (idx, c) in self.secret_word.chars().enumerate() {
            if guess_str == String::from(c).to_uppercase() {
                success = true;
                self.guessed_word.replace_range(idx..(idx+1), &guess_str);
            }
        }

        success
    }
}

fn get_input() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input error");
    input = input.trim().to_string();
    if input.len() > 0 {
        input.chars().nth(0).unwrap()
    } else {
        ' '
    }
}

fn main() {
    let mut state = State::new("banana");

    while state.get_lives() > 0 {
        println!("{}", state.get_guess());
        println!("{} live(s) left", state.get_lives());
        println!("Guess a letter: ");

        let input = get_input();
        if !state.guess(input) {
            state.decremet_lives();
        } else if state.get_has_won() {
            println!("You won! The correct word was: {}", state.get_answer());
            return;
        }
    }

    println!("Game over! The correct word was: {}", state.get_answer());
}
