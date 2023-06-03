pub enum RepositoryResult<T,E> {
    Ok(T),
    Err(E),
}