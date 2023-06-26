pub use eyre;

pub enum ErrorEnum {
    StateMachineError(&'static str),
}

pub enum Result<T> {
    Ok(T),
    Err(ErrorEnum),
}

