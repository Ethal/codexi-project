// src/logic/utils.rs

use nulid::Nulid;

use crate::core::ID_MIN_SHORT_LEN;

pub trait HasNulid {
    fn id(&self) -> Nulid;
}

pub trait ResolveError {
    fn not_found(input: String) -> Self;
    fn ambiguous(input: String) -> Self;
    fn invalid(input: String, min: usize) -> Self;
}

pub fn resolve_id<T, E>(input: &str, items: &[T]) -> Result<Nulid, E>
where
    T: HasNulid,
    E: ResolveError,
{
    if input.len() == 26 {
        return input.parse().map_err(|_| E::not_found(input.to_string()));
    }

    if input.len() < ID_MIN_SHORT_LEN {
        return Err(E::invalid(input.to_string(), ID_MIN_SHORT_LEN));
    }

    let short = input.to_uppercase();

    let mut matches = items
        .iter()
        .filter(|item| item.id().to_string().ends_with(&short));

    let first = matches.next();

    match (first, matches.next()) {
        (None, _) => Err(E::not_found(short)),
        (Some(item), None) => Ok(item.id()),
        (Some(_), Some(_)) => Err(E::ambiguous(short)),
    }
}
