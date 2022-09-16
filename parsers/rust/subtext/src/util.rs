use anyhow::Result;
use tendril::{SubtendrilError, Tendril};

/// Cut a tendril at the given index, returning the RHS of the cut
pub fn cut<T, A>(tendril: &Tendril<T, A>, at: usize) -> Result<Tendril<T, A>, SubtendrilError>
where
    T: tendril::Format,
    A: tendril::Atomicity,
{
    tendril.try_subtendril(at as u32, tendril.len32() - at as u32)
}

/// Slug-ify an arbitrary input string to make it compatible with
/// Subconscious' notion of slashlink slugs
pub fn to_slug(input: &str) -> Result<String> {
    let mut slug = input
        .trim()
        .trim_start_matches('/')
        .to_lowercase()
        .replace('\n', " ")
        .replace('\t', " ")
        .split(' ')
        .filter_map(|part| {
            if part.len() == 0 {
                return None;
            }

            Some(
                part.chars()
                    .filter(|char| match char {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '/' => true,
                        _ => false,
                    })
                    .collect::<String>(),
            )
        })
        .collect::<Vec<String>>()
        .join("-")
        .split('/')
        .filter(|part| part.len() > 0)
        .collect::<Vec<&str>>()
        .join("/");

    slug.truncate(200);

    Ok(slug)
}

#[cfg(test)]
pub fn assert_round_trip(input: &str) {
    let blocks: Vec<crate::block::Block<crate::primitive::Entity>> =
        crate::parse(input.as_bytes()).unwrap().collect();

    let actual_string = blocks
        .iter()
        .map(|block| block.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    assert_eq!(input, actual_string);
}
