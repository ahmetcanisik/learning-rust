pub mod game;

#[cfg(test)]
mod tests {
    use super::game::GuessingGame;

    #[test]
    fn it_works() {
        let mut game = GuessingGame {
            game_name: String::from("Test"),
            guess_left: 1,
            alert: String::from("test"),
            distance: (0, 1),
        };
        
        assert_eq!(game.play(), "test passed!");
    }
}