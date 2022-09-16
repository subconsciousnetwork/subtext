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
    Blank(E),
}

impl<E> Block<E>
where
    E: From<Entity> + AsRef<Entity>,
{
    /// Get the content entities for a block. For paragraphs, this is content
    /// that appears after leading whitespace; for headings, lists and block
    /// quotes, this is the content that appears after a sigil and subsequent
    /// leading whitespace; for blanks, this is empty.
    pub fn to_content_entities(&self) -> Vec<&E> {
        match self {
            Block::Header(entities) | Block::List(entities) | Block::Quote(entities) => entities
                .iter()
                .skip_while(|entity| match entity.as_ref() {
                    Entity::Sigil(_) | Entity::EmptySpace(_) => true,
                    _ => false,
                })
                .collect(),
            Block::Paragraph(entities) => entities
                .iter()
                .skip_while(|entity| match entity.as_ref() {
                    Entity::EmptySpace(_) => true,
                    _ => false,
                })
                .collect(),
            Block::Blank(_) => Vec::new(),
        }
    }

    /// Get the text content of a block, which is the concatenated string
    /// representation of all its content entities.
    pub fn to_text_content(&self) -> String {
        self.to_content_entities()
            .iter()
            .map(|entity| entity.as_ref().to_string())
            .collect()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Block::Header(entities)
            | Block::Paragraph(entities)
            | Block::Quote(entities)
            | Block::List(entities) => entities
                .iter()
                .map(|entity| entity.as_ref().as_bytes())
                .collect::<Vec<&[u8]>>()
                .concat(),
            Block::Blank(entity) => entity.as_ref().as_bytes().to_vec(),
        }
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

pub fn parse<E: From<Entity> + AsRef<Entity>>(
    input: SharedString,
) -> Result<(Block<E>, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut line_break_index = None;
    let mut last_index = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        last_index = index;
        match token {
            '\n' => {
                line_break_index = Some(index);
                break;
            }
            ' ' | '\t' => {
                iter.next();
                continue;
            }
            any_other => {
                let leading_whitespace_index = match index {
                    0 => 0usize,
                    _ => index,
                };

                let (mut entities, size, sigil) = match any_other {
                    '#' | '>' | '-' if index == 0 => {
                        let mut entities =
                            vec![Entity::Sigil(input.try_subtendril(index as u32, 1)?).into()];
                        let (mut content_entities, size) =
                            primitive::parse_text::<E>(cut(&input, index + 1)?)?;
                        entities.append(&mut content_entities);
                        (entities, size + 1, Some(any_other))
                    }
                    _ => {
                        let (entities, size) = primitive::parse_text::<E>(cut(&input, index)?)?;
                        (entities, size, None)
                    }
                };

                let entities = match leading_whitespace_index {
                    0 => entities,
                    _ => {
                        let mut new_entities = vec![Entity::EmptySpace(
                            input.subtendril(0, leading_whitespace_index as u32),
                        )
                        .into()];
                        new_entities.append(&mut entities);
                        new_entities
                    }
                };

                return Ok((
                    match sigil {
                        Some('#') => Block::Header(entities),
                        Some('>') => Block::Quote(entities),
                        Some('-') => Block::List(entities),
                        _ => Block::Paragraph(entities),
                    },
                    size + index + 1,
                ));
            }
        }
    }

    let length = if let Some(line_break_index) = line_break_index {
        line_break_index
    } else {
        last_index + 1
    };

    Ok((
        Block::Blank(Entity::EmptySpace(input.subtendril(0, length as u32)).into()),
        length + 1,
    ))
}
