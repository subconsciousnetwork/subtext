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
    List(Vec<E>),
    Link(E),
    Blank(E),
}

impl<E> Block<E>
where
    E: From<Entity> + AsRef<Entity>,
{
    pub fn to_text_content(&self) -> String {
        todo!()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Block::Header(entities)
            | Block::Paragraph(entities)
            | Block::Quote(entities)
            | Block::List(entities) => Block::entities_to_bytes(entities),
            Block::Link(entity) | Block::Blank(entity) => Vec::from(entity.as_ref().as_bytes()),
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

pub fn parse_blank<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let (primitive, steps) = primitive::parse_empty_space(input)?;
    Ok((Block::Blank(primitive.into()), steps))
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
    let (primitives, end) = parse_with_sigil(input, '-')?;
    Ok((Block::List(primitives), end))
}
