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

///
///  MACROS
///
/// Print basic debug information aka where the error is located
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

/// Print basic debug information aka where the error is located and also panics
macro_rules! PANIC_WITH_DEBUG_INFO {
  ($message:literal) => {
    eprintln!(
      "error in file :{}\ncolumn : {}\nline : {}\n ",
      file!(),
      column!(),
      line!(),
    );
    panic!($message);
  };
}

pub(crate) use PANIC_WITH_DEBUG_INFO;

pub type mainReturn = Result<(), errorTypes>;
