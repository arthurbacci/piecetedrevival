use std::sync::mpsc::TryRecvError;
use std::fs::File;
use std::io::{self, Write};

use base64::write::EncoderWriter;
use base64::engine::general_purpose;

use piecetedrevival::bind;
use piecetedrevival::graphics::KittyImageWriter;

fn main() {
    let rx = bind::get_sigwinch_channel();
    println!("{:?}", bind::get_window_sz());


    let mut out = io::stdout();
    let mut imgw = KittyImageWriter::new(
        &mut out,
        vec!["f=100".to_string(), "a=T".to_string()],
    ).unwrap();

    // TODO: make KittyImageWriter do the base64 stuff
    let mut img = Vec::new();
    let mut enc = EncoderWriter::new(&mut img, &general_purpose::STANDARD);
    let mut f = File::open("data/logo.png").unwrap();
    io::copy(&mut f, &mut enc).unwrap();
    drop(enc);
    drop(f);
    imgw.write_all(&img).unwrap();
    drop(imgw);
    out.flush().unwrap();
    drop(out);



    while let Err(TryRecvError::Empty) = rx.try_recv() {

    }
}
