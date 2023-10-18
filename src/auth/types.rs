pub enum SessionResult<T,E,F> {
    Ok(T),
    Err(E),
    FatalErr(F),
}