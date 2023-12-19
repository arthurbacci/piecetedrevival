//! Implementation of kitty's graphics protocol as specificed at [^1]. There's
//! no support for terminals that don't implement kitty's graphics protocol.
//!
//! [^1]: <https://sw.kovidgoyal.net/kitty/graphics-protocol>

use std::io::{self, Write};
use std::collections::HashMap;
use std::fmt;

use thiserror::Error;
use base64::write::EncoderWriter;
use base64::engine::general_purpose;

static IMAGE_CHUNK_SIZE: usize = 4096;


#[derive(Debug, Error)]
pub enum KittyImageWriteError {
    #[error("There are no fields for KittyImageWrite::new")]
    /// The implementation can't handle not having any fields since this
    /// wouldn't even make sense
    NoFields,
}

#[derive(Debug, Clone, Copy)]
pub enum KittyImageCmdValue {
    C(char),
    I(i32),
    U(u32),
}

impl fmt::Display for KittyImageCmdValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KittyImageCmdValue::C(c) => write!(f, "{c}"),
            KittyImageCmdValue::I(i) => write!(f, "{i}"),
            KittyImageCmdValue::U(u) => write!(f, "{u}"),
        }
    }
}

/// Encodes kitty data into APC, correctly separating the payload into chunks.
///
/// # Caution
/// 
/// Input isn't sanitized
// FIXME: poor (inexistent) error handling for IO errors
pub fn kitty_image_write(
    buf: &[u8], mut fields: HashMap<char, KittyImageCmdValue>,
) -> Result<(), KittyImageWriteError> {
    //      This may be removed
    if fields.is_empty() {
        return Err(KittyImageWriteError::NoFields);
    }

    let mut enc = EncoderWriter::new(Vec::new(), &general_purpose::STANDARD);
    //      TODO: Error handling
    //      Maybe use anyhow
    enc.write_all(buf).unwrap();
    let b = enc.finish().unwrap();
    let mut buf = &b[..];

    let mut out = io::stdout().lock();
    
    let q = fields.remove(&'q').unwrap_or(KittyImageCmdValue::U(0));
    let mut fields_s = String::new();
    for (k, v) in fields {
        fields_s.push(',');
        fields_s.push(k);
        fields_s.push('=');
        fields_s.push_str(&format!("{v}"));
    }
    let fields = fields_s;

    //      TODO: test with buf.len() == IMAGE_CHUNK_SIZE and other stuff
    //      tests can be done by reading the response packet from kitty

    if buf.len() <= IMAGE_CHUNK_SIZE {
        write!(out, "\x1b_Gq={q}{};", fields).unwrap();
        out.write_all(&buf).unwrap();
        write!(out, "\x1b\\").unwrap();
    } else {
        write!(out, "\x1b_Gq={q},m=1{};", fields).unwrap();
        out.write_all(&buf[0..IMAGE_CHUNK_SIZE]).unwrap();
        write!(out, "\x1b\\").unwrap();
        buf = &buf[IMAGE_CHUNK_SIZE..];

        while buf.len() > IMAGE_CHUNK_SIZE {
            write!(out, "\x1b_Gq={q},m=1;").unwrap();
            out.write_all(&buf[0..IMAGE_CHUNK_SIZE]).unwrap();
            write!(out, "\x1b\\").unwrap();
            buf = &buf[IMAGE_CHUNK_SIZE..];
        }

        write!(out, "\x1b_Gq={q},m=0;").unwrap();
        out.write_all(&buf).unwrap();
        write!(out, "\x1b\\").unwrap();
    }

    out.flush().unwrap();

    Ok(())
}





