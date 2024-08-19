mod actions;
mod template;

use actions::Action;
use std::path::PathBuf;
use template::Template;

const SAVE_PATH: &str = "./conda.yml";

fn main() -> std::io::Result<()> {
    let save_path_buf = PathBuf::from(SAVE_PATH);
    let mut template = Template::load_or_new(save_path_buf.clone());

    loop {
        println!("{template}");
        let action_result = Action::read_stdin();

        match action_result {
            Err(read_error) => match read_error {
                actions::ActionReadError::UnknownAction(value) => {
                    eprintln!("Unknown action: {value}")
                }
            },
            Ok(action) => match action {
                Action::Left => template.position.move_steps(-5, 0),
                Action::Right => template.position.move_steps(5, 0),
                Action::UP => template.position.move_steps(0, -5),
                Action::Down => template.position.move_steps(0, 5),
                Action::Heal => template.heal(10),
                Action::Damage => template.damage(10),
                Action::Quit => break,
                Action::Save => {
                    template.save(save_path_buf.clone())?;
                    println!("Saved to {SAVE_PATH}!");
                }
            },
        }
    }

    Ok(())
}
