use crate::str::SharedString;
use tendril::SubtendrilError;

use crate::{
    predicate::{
        link_predicate, white_space_predicate, wiki_link_delimiter_predicate, ParseLinkAs,
    },
    util::cut,
};

#[derive(Debug, Clone)]
pub enum Entity {
    Sigil(SharedString),
    TextSpan(SharedString),
    EmptySpace(SharedString),
    SlashLink(SharedString),
    HyperLink(SharedString),
    WikiLink(SharedString),
}

impl AsRef<Entity> for Entity {
    fn as_ref(&self) -> &Entity {
        self
    }
}

impl Entity {
    pub fn to_string(&self) -> String {
        match self {
            Entity::TextSpan(tendril) => tendril.into(),
            Entity::Sigil(tendril) => tendril.into(),
            Entity::EmptySpace(tendril) => tendril.into(),
            Entity::SlashLink(tendril) => tendril.into(),
            Entity::HyperLink(tendril) => tendril.into(),
            Entity::WikiLink(tendril) => tendril.into(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Entity::TextSpan(tendril)
            | Entity::Sigil(tendril)
            | Entity::EmptySpace(tendril)
            | Entity::SlashLink(tendril)
            | Entity::HyperLink(tendril)
            | Entity::WikiLink(tendril) => tendril.as_bytes(),
        }
    }
}

pub fn parse_empty_space(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    let mut iter = input.char_indices().peekable();
    let mut end = 0usize;

    while let Some(&(index, token)) = iter.peek() {
        match token {
            '\r' | '\n' | '\t' | ' ' => iter.next(),
            _ => break,
        };
        end = index;
    }

    end = end + 1;

    Ok((
        Entity::EmptySpace(input.try_subtendril(0, end as u32)?),
        end,
    ))
}

pub fn parse_until<P>(
    input: SharedString,
    mut predicate: P,
) -> Result<(SharedString, usize), SubtendrilError>
where
    P: FnMut(&char) -> Option<usize>,
{
    let mut iter = input.char_indices().peekable();
    let mut end = 0usize;
    while let Some(&(index, token)) = iter.peek() {
        end = index;

        let predicate_match = predicate(&token);

        if let Some(match_length) = predicate_match {
            end = end + match_length - 1;
            return Ok((input.try_subtendril(0, end as u32)?, end));
        }

        iter.next();
    }

    return Ok((input, end + 1));
}

pub fn parse_slash_link(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until(input, white_space_predicate()) {
        Ok((value, steps)) => Ok((Entity::SlashLink(value), steps)),
        Err(error) => Err(error),
    }
}

pub fn parse_hyper_link(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until(input, white_space_predicate()) {
        Ok((value, steps)) => Ok((Entity::HyperLink(value), steps)),
        Err(error) => Err(error),
    }
}

pub fn parse_wiki_link(input: SharedString) -> Result<(Entity, usize), SubtendrilError> {
    match parse_until(input, wiki_link_delimiter_predicate()) {
        Ok((value, steps)) => Ok((Entity::WikiLink(value), steps)),
        Err(error) => Err(error),
    }
}

pub fn parse_text<E>(input: SharedString) -> Result<(Vec<E>, usize), SubtendrilError>
where
    E: From<Entity> + AsRef<Entity>,
{
    let mut iter = input.char_indices().peekable();
    let mut start = 0usize;
    let mut end = 0usize;
    let mut entities = Vec::<E>::new();

    let mut is_link = link_predicate();

    'parse: while let Some(&(index, token)) = iter.peek() {
        if let Some((match_length, parse_as)) = is_link(&token) {
            end = index - (match_length - 1);

            if end > start {
                let text_span_entity = Entity::TextSpan(
                    input.try_subtendril(start as u32, end as u32 - start as u32)?,
                );
                entities.push(text_span_entity.into());
            }

            let link_input = cut(&input, end)?;
            let (link_entity, steps) = match parse_as {
                ParseLinkAs::HyperLink => parse_hyper_link(link_input)?,
                ParseLinkAs::SlashLink => parse_slash_link(link_input)?,
                ParseLinkAs::WikiLink => parse_wiki_link(link_input)?,
            };

            start = end + steps;
            end = start;

            entities.push(link_entity.into());

            match iter.nth(start - index) {
                Some((_, '\r')) | Some((_, '\n')) => {
                    break 'parse;
                }
                _ => {
                    continue 'parse;
                }
            };
        }

        match token {
            '\r' | '\n' => {
                end = index;
                break 'parse;
            }
            _ => (),
        };

        iter.next();

        end = index;

        if let None = iter.peek() {
            end = end + 1;
        }
    }

    if end > start {
        let value = input.try_subtendril(start as u32, end as u32 - start as u32)?;
        end = end;
        entities.push(Entity::TextSpan(value).into());
    }

    Ok((entities, end))
}
