pub enum ServiceResult<T,E> {
    Ok(T),
    Err(E),
}