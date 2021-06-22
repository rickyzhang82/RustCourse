use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();
    // Use `game` to initialize starting state.
    let race_car: &mut Actor = game.add_actor("race car", ActorPreset::RacingCarYellow);
    race_car.translation = Vec2::new(-100.0, -100.0);
    race_car.rotation = NORTH_EAST;
    race_car.scale = 2.0;
    // Then do `game.run()` to start the game.
    game.run(game_logic);
}

// This function is called once per frame
fn game_logic(game_state: &mut GameState) {
    // Your game logic goes here
}