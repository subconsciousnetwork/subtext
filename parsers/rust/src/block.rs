use crate::{primitive, util::cut};
use tendril::{StrTendril, SubtendrilError};

#[derive(Debug, Clone)]
pub enum Entity {
    Header(Vec<primitive::Entity>),
    Paragraph(Vec<primitive::Entity>),
    Quote(Vec<primitive::Entity>),
    List(Vec<Vec<primitive::Entity>>),
    Link(primitive::Entity),
    Seperator(primitive::Entity),
}

pub fn parse_empty_space(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let (primitive, steps) = primitive::parse_empty_space(input)?;
    Ok((Entity::Seperator(primitive), steps))
}

pub fn parse_paragraph(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let (primitives, steps) = primitive::parse_text(input)?;

    match &primitives[..] {
        [value] => match value {
            primitive::Entity::HyperLink(_) => Ok((Entity::Link(value.clone()), steps)),
            primitive::Entity::SlashLink(_) => Ok((Entity::Link(value.clone()), steps)),
            _ => Ok((Entity::Paragraph(primitives), steps)),
        },
        _ => Ok((Entity::Paragraph(primitives), steps)),
    }
}

pub fn parse_link(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let (primitive, steps) = match input.chars().nth(0) {
        Some('/') => primitive::parse_slash_link(input)?,
        _ => primitive::parse_hyper_link(input)?,
    };

    Ok((Entity::Link(primitive), steps))
}

fn parse_with_sigil(
    input: StrTendril,
    sigil: char,
) -> Result<(Vec<primitive::Entity>, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut primitives = Vec::<primitive::Entity>::new();
    let mut end = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        let mut advance = 0usize;

        match primitives.len() {
            0 => match token {
                char if char == sigil => {
                    primitives.push(primitive::Entity::Sigil(input.try_subtendril(0, 1)?))
                }
                _ => break,
            },
            _ => match token {
                '\r' | '\n' => break,
                '\t' | ' ' => {
                    let slice = cut(&input, index)?;
                    let (primitive, steps) = primitive::parse_empty_space(slice)?;

                    advance = steps;
                    primitives.push(primitive);
                }
                _ => {
                    let slice = cut(&input, index)?;
                    let (text_primitives, steps) = primitive::parse_text(slice)?;

                    advance = steps;
                    primitives.extend(text_primitives.into_iter());
                }
            },
        };

        end = index + advance;
        iter.nth(advance);
    }

    Ok((primitives, end))
}

pub fn parse_header(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let (primitives, end) = parse_with_sigil(input, '#')?;
    Ok((Entity::Header(primitives), end))
}

pub fn parse_quote(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let (primitives, end) = parse_with_sigil(input, '>')?;
    Ok((Entity::Quote(primitives), end))
}

pub fn parse_list(input: StrTendril) -> Result<(Entity, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut items = Vec::<Vec<primitive::Entity>>::new();

    let mut end = 0usize;
    let mut line_breaks = 0usize;
    let mut empty_space_length = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        let mut advance = 0usize;
        match token {
            '-' => {
                let (item, steps) = parse_with_sigil(cut(&input, index)?, '-')?;
                items.push(item);
                advance = steps;

                line_breaks = 0;
                empty_space_length = 0;
            }
            '\r' | ' ' | '\t' => {
                empty_space_length += 1;
            }
            '\n' => {
                empty_space_length += 1;
                line_breaks += 1;
                if line_breaks > 1 {
                    break;
                }
            }
            _ => break,
        };

        end = index + advance;
        iter.nth(advance);
    }

    Ok((Entity::List(items), end - empty_space_length))
}
