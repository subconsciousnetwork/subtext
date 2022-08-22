use anyhow::{anyhow, Result};
use async_compat::CompatExt;
use async_stream::try_stream;
use async_utf8_decoder::Utf8Decoder;
use futures::{Stream, StreamExt};
use tokio::io::AsyncRead;

use crate::{block::Block, parse, primitive::Entity};

pub async fn parse_one<B, E>(input: &[u8]) -> Result<B>
where
    B: From<Block<E>>,
    E: From<Entity> + AsRef<Entity>,
{
    match parse(input)?.next() {
        Some(block) => Ok(block),
        None => Err(anyhow!("No block found in input")),
    }
}

pub async fn stream<B, E, R>(input: R) -> impl Stream<Item = Result<B>>
where
    E: From<Entity> + AsRef<Entity>,
    B: From<Block<E>>,
    R: AsyncRead + Unpin,
{
    try_stream! {
      let mut decoder = Utf8Decoder::new(input.compat());
      let mut content_buffer = String::new();
      let mut empty_space_buffer = String::new();

      while let Some(result) = decoder.next().await {
          match result {
              Ok(chunk) => {
                  for character in chunk.chars() {
                      match character {
                          '\r' | '\n' => {
                              if content_buffer.len() > 0 {
                                  yield parse_one(content_buffer.as_ref()).await?;
                                  content_buffer.clear();
                              }

                              empty_space_buffer.push(character);
                          }
                          ' ' | '\t' => {
                              if empty_space_buffer.len() > 0 {
                                  empty_space_buffer.push(character)
                              } else {
                                  content_buffer.push(character)
                              }
                          }
                          _ => {
                              if empty_space_buffer.len() > 0 {
                                  yield parse_one(empty_space_buffer.as_ref()).await?;
                                  empty_space_buffer.clear();
                              }

                              content_buffer.push(character);
                          }
                      }
                  }
              }
              Err(_) => {}
          }
      }

      if content_buffer.len() > 0 {
        yield parse_one(content_buffer.as_ref()).await?;
      }

      if empty_space_buffer.len() > 0 {
        yield parse_one(empty_space_buffer.as_ref()).await?;
      }
    }
}

#[cfg(test)]
mod tests {
    use async_compat::CompatExt;
    use futures::{channel::mpsc, io, SinkExt};
    use futures::{pin_mut, StreamExt, TryStream, TryStreamExt};

    use crate::block::Block;
    use crate::primitive::Entity;
    use crate::stream;

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    async fn it_parses_an_async_stream_of_bytes() {
        let (mut tx, rx) = mpsc::unbounded();
        let block_stream = stream::<Block<Entity>, _, _>(rx.into_async_read().compat()).await;

        tx.send(Ok(b"# the title".to_vec())).await.unwrap();
        tx.send(Ok(b"\n".to_vec())).await.unwrap();
        tx.send(Ok(b"The first part... /foo-".to_vec()))
            .await
            .unwrap();
        tx.send(Ok(b"bar-baz ... the second part".to_vec()))
            .await
            .unwrap();

        tx.close().await.unwrap();

        pin_mut!(block_stream);

        let block = block_stream.next().await;

        match block {
            Some(Ok(Block::Header(entities))) => {
                assert_eq!(entities.get(0).unwrap().to_string(), "#");
                assert_eq!(entities.get(1).unwrap().to_string(), " ");
                assert_eq!(entities.get(2).unwrap().to_string(), "the title");
            }
            _ => panic!("Incorrect block or primitive type!"),
        }

        let block = block_stream.next().await;

        match block {
            Some(Ok(Block::Seperator(_))) => {}
            _ => panic!("Incorrect block or primitive type!"),
        }

        let block = block_stream.next().await;

        match block {
            Some(Ok(Block::Paragraph(entities))) => {
                assert_eq!(entities.get(0).unwrap().to_string(), "The first part... ");
                assert_eq!(entities.get(1).unwrap().to_string(), "/foo-bar-baz");
                assert_eq!(entities.get(2).unwrap().to_string(), " ... the second part");
            }
            _ => panic!("Incorrect block or primitive type!"),
        }
    }
}
