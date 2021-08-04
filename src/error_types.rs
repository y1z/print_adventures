#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum errorTypes {
  cannot_init,
  opt_failed,
  no_error,
}

impl errorTypes {
  pub fn desc(&self) -> String {
    match *self {
      errorTypes::cannot_init => return String::from("could not initialize"),
      errorTypes::opt_failed => return String::from("operation failed"),
      errorTypes::no_error => return String::from("no error detected"),

      _ => return String::from("unknown case"),
    }
  }
}
