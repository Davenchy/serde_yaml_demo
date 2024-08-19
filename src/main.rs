mod actions;
mod state;

use actions::Action;
use state::State;
use std::path::PathBuf;

const STATE_FILE_PATH: &str = "./state.yml";

fn main() -> std::io::Result<()> {
    let save_path_buf = PathBuf::from(STATE_FILE_PATH);
    let mut state = State::load_or_new(save_path_buf.clone());

    loop {
        println!("{state:?}");
        let action_result = Action::read_stdin();

        match action_result {
            Err(read_error) => match read_error {
                actions::ActionReadError::UnknownAction(value) => {
                    eprintln!("Unknown action: {value}")
                }
            },
            Ok(action) => match action {
                Action::Left => state.position.move_steps(-5, 0),
                Action::Right => state.position.move_steps(5, 0),
                Action::UP => state.position.move_steps(0, -5),
                Action::Down => state.position.move_steps(0, 5),
                Action::Heal => state.heal(10),
                Action::Damage => state.damage(10),
                Action::Quit => break,
                Action::Save => {
                    state.save(save_path_buf.clone())?;
                    println!("Saved to {STATE_FILE_PATH}!");
                }
            },
        }
    }

    Ok(())
}
