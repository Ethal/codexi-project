// src/logic/utils.rs

use crate::core::ID_MIN_SHORT_LEN;
use nulid::Nulid;

pub trait HasNulid {
    fn id(&self) -> Nulid;
}

// --- Optional name trait ---
// Implement this only for types that have a human-readable name field.
// Not required: if T does not implement HasName, name-based resolution is simply
// unavailable at compile time (no runtime cost, no fallback to not_found).
pub trait HasName {
    fn name(&self) -> &str;
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

// --- Resolution by ID or name ---
// Falls back to name matching only when the ID-suffix search finds zero results.
// Name resolution priority: exact match > prefix match > contains match.
// If multiple items match at the same priority level, returns Err::ambiguous.

pub fn resolve_by_id_or_name<T, E>(input: &str, items: &[T]) -> Result<Nulid, E>
where
    T: HasNulid + HasName,
    E: ResolveError,
{
    // --- Phase 1: full Nulid string (26 chars) ---
    if input.len() == 26 {
        return input.parse().map_err(|_| E::not_found(input.to_string()));
    }

    // --- Phase 2: short ID suffix (must meet minimum length) ---
    if input.len() >= ID_MIN_SHORT_LEN {
        let short = input.to_uppercase();
        let id_matches: Vec<&T> = items
            .iter()
            .filter(|item| item.id().to_string().ends_with(&short))
            .collect();
        match id_matches.len() {
            0 => {} // fall through to name resolution
            1 => return Ok(id_matches[0].id()),
            _ => return Err(E::ambiguous(short)),
        }
    }

    // --- Phase 3: name resolution ---
    // Inputs shorter than ID_MIN_SHORT_LEN are not treated as short IDs,
    // but they may still be valid names (e.g. "ops", "main").
    resolve_by_name(input, items)
}

// --- Internal: name-only resolution ---
// Tries exact match first, then prefix, then substring (case-insensitive).
// Ambiguity is checked independently at each priority level:
//   - If a single exact match exists, it wins regardless of prefix/contains matches.
//   - If multiple exact matches exist, returns Err::ambiguous immediately.
//   - Same logic applies to prefix and contains tiers.

fn resolve_by_name<T, E>(input: &str, items: &[T]) -> Result<Nulid, E>
where
    T: HasNulid + HasName,
    E: ResolveError,
{
    let needle = input.to_lowercase();

    let exact: Vec<&T> = items
        .iter()
        .filter(|item| item.name().to_lowercase() == needle)
        .collect();

    if !exact.is_empty() {
        return match_unique(exact, input.to_string());
    }

    let prefix: Vec<&T> = items
        .iter()
        .filter(|item| item.name().to_lowercase().starts_with(&needle))
        .collect();

    if !prefix.is_empty() {
        return match_unique(prefix, input.to_string());
    }

    let contains: Vec<&T> = items
        .iter()
        .filter(|item| item.name().to_lowercase().contains(&needle))
        .collect();

    match_unique(contains, input.to_string())
}

// Converts a candidate list into Ok(id) or the appropriate error.
fn match_unique<T, E>(candidates: Vec<&T>, input: String) -> Result<Nulid, E>
where
    T: HasNulid,
    E: ResolveError,
{
    match candidates.len() {
        0 => Err(E::not_found(input)),
        1 => Ok(candidates[0].id()),
        _ => Err(E::ambiguous(input)),
    }
}
