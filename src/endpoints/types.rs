use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum EndpointResult<T,E> {
    Ok(T),
    Err(E),
}