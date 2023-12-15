//! Implementation of kitty's graphics protocol as specificed at [^1]. There's
//! no support for terminals that don't implement kitty's graphics protocol.
//!
//! [^1]: <https://sw.kovidgoyal.net/kitty/graphics-protocol>

use std::io::{self, Write};

use thiserror::Error;

static IMAGE_CHUNK_SIZE: usize = 4096;

/// Encodes kitty data into APC, correctly separating the payload into chunks.
/// The user only needs to create it with [KittyImageWriter::new], setting all
/// the key-value pairs you need and then use [write_all](Write::write_all) to
/// write the payload into the specified writer.
///
/// # Notes
/// - It uses [write_all](Write::write_all) on the inner writer.
pub struct KittyImageWriter<'w, W: Write> {
    w: &'w mut W,
    fields: Vec<String>,
    q: char,
    /// True if it isn't the first chunk
    linger: bool,
}

impl<'w, W: Write> Write for KittyImageWriter<'w, W> {
    fn flush(&mut self) -> io::Result<()> {
        self.w.flush()
    }
    
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.linger {
            false => {
                if buf.len() < IMAGE_CHUNK_SIZE {
                    write!(
                        self.w, "\x1b_Gq={},{};",
                        self.q, self.fields.join(","),
                    )?;
                    self.w.write_all(buf)?;
                    write!(self.w, "\x1b\\")?;
                    Ok(buf.len())
                } else {
                    self.linger = true;
                    write!(
                        self.w, "\x1b_Gq={},m=1,{};",
                        self.q, self.fields.join(","),
                    )?;
                    self.w.write_all(&buf[0..IMAGE_CHUNK_SIZE])?;
                    write!(self.w, "\x1b\\")?;
                    Ok(IMAGE_CHUNK_SIZE)
                }
            }
            true => {
                let last = buf.len() < IMAGE_CHUNK_SIZE;

                write!(
                    self.w, "\x1b_Gq={},m={};",
                    self.q, if last {'0'} else {'1'},
                )?;
                let r = if last {
                    self.w.write_all(buf)?;
                    Ok(buf.len())
                } else {
                    self.w.write_all(&buf[0..IMAGE_CHUNK_SIZE])?;
                    Ok(IMAGE_CHUNK_SIZE)
                };
                write!(self.w, "\x1b\\")?;
                r
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum KittyImageWriteBuildError {
    #[error("There are no fields for KittyImageWrite::new")]
    /// The implementation can't handle not having any fields since this
    /// wouldn't even make sense
    NoFields,
}

impl<'w, W: Write> KittyImageWriter<'w, W> {
    /// `w` is the place to write to, `fields` is a vector "key=value" strings
    ///
    /// # Caution
    /// Input isn't sanitized
    pub fn new(w: &'w mut W, mut fields: Vec<String>)
    -> Result<Self, KittyImageWriteBuildError> {
        let mut q = None;
        fields.retain(
            |x| if x.starts_with("q=") {
                q = x.chars().nth(2);
                println!("Q: {q:?}");
                false
            } else {
                true
            }
        );
        if fields.is_empty() {
            return Err(KittyImageWriteBuildError::NoFields);
        }
        
        Ok(Self {
            w: w,
            q: q.unwrap_or('0'),
            fields: fields,
            linger: false,
        })
    }
}



