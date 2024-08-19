# Serde YAML Demo

This Rust project demonstrates the use of the `serde` crate to serialize and deserialize state data to and from YAML files. The project features a simple state management system where the state can be saved, loaded, and modified through basic commands.

## Features

- **State Management**: Track the position and health of an entity.
- **Serialization**: Save the current state to a YAML file.
- **Deserialization**: Load a previously saved state from a YAML file.
- **Interactive Commands**: Move the entity, adjust health, and save or quit the application through user input.

## Code Structure

- **main.rs**: The entry point of the application. Manages the main loop, user input, and state updates.
- **state.rs**: Defines the `State` and `Point` structures, along with methods to manipulate and serialize/deserialize them.
- **actions.rs**: Handles user input and defines the various actions that can be performed (e.g., move left, right, heal, etc.).

## Usage

1. **Clone the Repository**:

   ```bash
   git clone --depth=1 https://github.com/Davenchy/serde_yaml_demo.git
   cd serde_yaml_demo
   ```

2. **Run the Application**:

   ```bash
   cargo run
   ```

3. **Available Commands**:

   - `Left` or `l`: Move the entity 5 units to the left.
   - `Right` or `r`: Move the entity 5 units to the right.
   - `Up` or `u`: Move the entity 5 units up.
   - `Down` or `d`: Move the entity 5 units down.
   - `Damage` or `a`: Reduce the entity’s health by 10.
   - `Heal` or `h`: Increase the entity’s health by 10.
   - `Save` or `s`: Save the current state to `state.yml`.
   - `Quit` or `q`: Exit the application.

4. **YAML State File**:
   - The state is saved to `state.yml` in the project’s root directory.
   - On application start, if `state.yml` exists, the state will be loaded from it; otherwise, a new state will be created on the `Save` action.

## Dependencies

- [serde](https://crates.io/crates/serde) for serialization and deserialization.
- [serde_yaml_ng](https://crates.io/crates/serde_yaml_ng) for working with YAML files.

## Notes

This project was created as a learning exercise to explore Rust’s serialization capabilities with `serde`. It is archived and not under active development.

## License

This project is licensed under the MIT License.
