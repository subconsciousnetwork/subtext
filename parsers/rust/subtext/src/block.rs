use crate::str::SharedString;
use crate::{primitive, primitive::Entity, util::cut};
use tendril::SubtendrilError;

#[derive(Debug, Clone)]
pub enum Block<E>
where
    E: From<Entity> + AsRef<Entity>,
{
    Header(Vec<E>),
    Paragraph(Vec<E>),
    Quote(Vec<E>),
    List(Vec<Vec<E>>),
    Link(E),
    Seperator(E),
}

impl<E> Block<E>
where
    E: From<Entity> + AsRef<Entity>,
{
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Block::Header(entities) | Block::Paragraph(entities) | Block::Quote(entities) => {
                Block::entities_to_bytes(entities)
            }
            Block::List(list) => list
                .iter()
                .map(Block::entities_to_bytes)
                .reduce(|mut bytes, mut next| {
                    bytes.extend_from_slice("\n".as_bytes());
                    bytes.append(&mut next);
                    bytes
                })
                .or_else(|| Some(Vec::new()))
                .unwrap(),
            Block::Link(entity) | Block::Seperator(entity) => Vec::from(entity.as_ref().as_bytes()),
        }
    }

    fn entities_to_bytes(entities: &Vec<E>) -> Vec<u8> {
        entities
            .iter()
            .map(|entity| entity.as_ref().as_bytes())
            .collect::<Vec<&[u8]>>()
            .concat()
    }
}

impl<E> ToString for Block<E>
where
    E: From<Entity> + AsRef<Entity>,
{
    fn to_string(&self) -> String {
        let bytes = self.to_bytes();

        match std::str::from_utf8(&bytes) {
            Ok(str) => str,
            Err(_) => "",
        }
        .into()
    }
}

pub fn parse_empty_space<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let (primitive, steps) = primitive::parse_empty_space(input)?;
    Ok((Block::Seperator(primitive.into()), steps))
}

pub fn parse_paragraph<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let (primitives, steps) = primitive::parse_text::<E>(input)?;

    match &primitives[..] {
        [value] => match value.as_ref() {
            primitive::Entity::HyperLink(_) => {
                Ok((Block::Link(value.as_ref().clone().into()), steps))
            }
            primitive::Entity::SlashLink(_) => {
                Ok((Block::Link(value.as_ref().clone().into()), steps))
            }
            _ => Ok((Block::Paragraph(primitives), steps)),
        },
        _ => Ok((Block::Paragraph(primitives), steps)),
    }
}

pub fn parse_link<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let (primitive, steps) = match input.chars().nth(0) {
        Some('/') => primitive::parse_slash_link(input)?,
        _ => primitive::parse_hyper_link(input)?,
    };

    Ok((Block::Link(primitive.into()), steps))
}

fn parse_with_sigil<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
    sigil: char,
) -> Result<(Vec<E>, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut primitives = Vec::<E>::new();
    let mut end = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        let mut advance = 0usize;

        match primitives.len() {
            0 => match token {
                char if char == sigil => {
                    primitives.push(primitive::Entity::Sigil(input.try_subtendril(0, 1)?).into());
                }
                _ => break,
            },
            _ => match token {
                '\r' | '\n' => break,
                '\t' | ' ' => {
                    let slice = cut(&input, index)?;
                    let (primitive, steps) = primitive::parse_empty_space(slice)?;

                    advance = steps - 1;
                    primitives.push(primitive.into());
                }
                _ => {
                    let slice = cut(&input, index)?;
                    let (text_primitives, steps) = primitive::parse_text(slice)?;

                    advance = steps - 1;
                    primitives.extend(text_primitives.into_iter());
                }
            },
        };

        end = index + advance + 1;
        iter.nth(advance);
    }

    Ok((primitives, end))
}

pub fn parse_header<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let (primitives, end) = parse_with_sigil(input, '#')?;
    Ok((Block::Header(primitives), end))
}

pub fn parse_quote<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let (primitives, end) = parse_with_sigil(input, '>')?;
    Ok((Block::Quote(primitives), end))
}

pub fn parse_list<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut items = Vec::<Vec<E>>::new();

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

    Ok((Block::List(items), end - empty_space_length))
}
