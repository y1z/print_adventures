#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum errorTypes {
  cannot_init,
  opt_failed,
  no_error,
}

impl errorTypes {
  pub fn desc(&self) -> String {
    match *self {
      errorTypes::cannot_init => String::from("could not initialize"),
      errorTypes::opt_failed => String::from("operation failed"),
      errorTypes::no_error => String::from("no error detected"),

      _ => String::from("unknown case"),
    }
  }
}

macro_rules! PRINT_DEBUG_INFO {
  () => {
    eprintln!(
      "error in file :{}\ncolumn : {}\nline : {}\n ",
      file!(),
      column!(),
      line!(),
    );
  };
}
pub(crate) use PRINT_DEBUG_INFO;

pub type mainReturn = Result<(), errorTypes>;
