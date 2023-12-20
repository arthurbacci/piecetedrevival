//! Implementation of general functionality for terminal graphics, including
//! kitty's graphics protocol as specificed at [^1]. It's not a major goal of
//! this project to support terminals that don't implement the most common
//! protocols, focusing mainly on kitty.
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
pub enum KittyImageWriteError {}

///
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
//      FIXME: poor (inexistent) error handling
//      TODO: read stdin searching for kitty's response. This may require
//          implementing some kind of async "messages" so that responses from
//          the terminal are redirected correctly. `mpsc` may not be perfect
//          for this since more than one threads would need to intercept the
//          packages. `crossbeam_channel` would be good if it had a way to seek
//          data before making it disappear from the channel. Another solution
//          would be to have a mpsc for each kind of event (keyboard press,
//          etc), this is most likely the best option since 
pub fn kitty_image_write(
    buf: &[u8], mut fields: HashMap<char, KittyImageCmdValue>,
) -> Result<(), KittyImageWriteError> {
    //      TODO: Test with empty fields

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


//      TODO: Make if first try using a temporary file (using a library for it,
//      since it probably has a better handling and may work on non-linux sys-
//      tems) and sending the data if it fails. As described in https://
//      sw.kovidgoyal.net/kitty/graphics-protocol/
//      #querying-support-and-available-transmission-mediums
//
//      Notes
//
//      Delete an image with "a=d;d=I" and "i=id"
//      Delete a placement with "a=d;d=i" and "i=id,d=pid"
//      Kitty may delete older images if too much storage is used in total, it
//          may be useful to try to send the image data again if this happens,
//          but somehow avoid an infinite loop of sending-image, querying-image
//          and not-finding-image
/*
pub struct KittyImage {
    placements: Vec<KittyImagePlacement>,
}
*/

/*
impl Drop for KittyImage {
    fn drop(&mut self) {
        //      TODO: Here I should tell kitty to delete the image from it
    }
}
*/







