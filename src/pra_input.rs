use crate::vector2::Vector2i;
use std::option::Option;

#[derive(Clone, Copy)]
pub struct CommandData {
  key: &'static str,
  value: Vector2i,
}

const g_command_strings: [CommandData; 4] = [
  CommandData {
    key: "w",
    value: Vector2i { x: 0, y: 1 },
  },
  CommandData {
    key: "s",
    value: Vector2i { x: 0, y: -1 },
  },
  CommandData {
    key: "a",
    value: Vector2i { x: -1, y: 0 },
  },
  CommandData {
    key: "d",
    value: Vector2i { x: 1, y: 1 },
  },
];

/// @returns a string that contain the input of the user
pub fn read_user_input_string() -> String {
  let mut buffer = String::new();
  let mut input = std::io::stdin();
  input.read_line(&mut buffer);
  buffer
}

/// @returns a string that contain the input of the user
pub fn recieve_input_command_from_string(input_string: &String) -> Option<CommandData> {
  for commond in g_command_strings.iter() {
    let index = input_string.find(commond.key);
    if index.is_some() {
      return Some(*commond);
    }
  }

  None
}
