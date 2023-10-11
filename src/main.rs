use console::{Key, Term};
use game::{Game, QueenBee};

mod game;

fn main() {
    let mut game = Game::default();
    game.add_honey(10);
    let term = Term::stdout();
    let _ = term.clear_screen();
    // Core game loop
    loop {
        let _ = term.write_line("Hello! Welcome to bee game!");
        let _ = term.write_line("Press 'q' to quit.");
        let _ = term.write_line("Press '1' to view your honey count");
        let _ = term.write_line("Press '2' to view your beehives.");
        let _ = term.write_line("Press '3' to buy a beehive for 10 honey.");

        let user_input = term.read_key();
        let _ = term.clear_screen();
        match user_input.unwrap() {
            Key::Char('q') => break,
            Key::Char('1') => println!("You have {} honey.", game.get_total_honey()),
            Key::Char('2') => println!("These are your beehives {:?}", game.get_all_beehives()),
            Key::Char('3') => {
                println!("{}", game.buy_beehive());
            }
            _ => continue,
        }
    }
}
