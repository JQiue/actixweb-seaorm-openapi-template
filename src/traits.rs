pub trait LoggingResultErr<T, E> {
  #[track_caller]
  fn log_err(self) -> Result<T, E>;
}

impl<T, E: std::fmt::Debug> LoggingResultErr<T, E> for Result<T, E> {
  #[track_caller]
  fn log_err(self) -> Result<T, E> {
    if let Err(ref e) = self {
      // tracing::error!("{:?}", e);
      let location = std::panic::Location::caller();
      tracing::error!("Error at {}:{} - {:?}", location.file(), location.line(), e);
    }
    self
  }
}
