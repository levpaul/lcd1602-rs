#[derive(Debug)]
pub enum Error<GPIO> {
    GPIOError(GPIO),
    TimerError,
    InvalidAddr,
    InvalidCursorPos,
}

impl<E> From<E> for Error<E> {
    fn from(gpio_err: E) -> Self {
        Self::GPIOError(gpio_err)
    }
}
