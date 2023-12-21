use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;

use piecetedrevival::bind;
use piecetedrevival::graphics::{KittyImage, KittyImageCmdValue};

fn main() {
    let rx = bind::get_sigwinch_channel();
    println!("{:?}", bind::get_window_sz());


    let mut img = Vec::new();
    let mut f = File::open("data/logo.png").unwrap();
    f.read_to_end(&mut img).unwrap();
    drop(f);

    let mut img = KittyImage::new(img);

    let mut c = Vec::new();
    let mut f = File::open("data/c.png").unwrap();
    f.read_to_end(&mut c).unwrap();
    drop(f);

    let mut c = KittyImage::new(c);

    c.place(HashMap::from([
        ('y', KittyImageCmdValue::U(413)),
        ('c', KittyImageCmdValue::U(32)),
        ('r', KittyImageCmdValue::U(16)),
        ('z', KittyImageCmdValue::I(-2)),
    ]));

    img.place(HashMap::from([
        ('c', KittyImageCmdValue::U(16)),
        ('r', KittyImageCmdValue::U(8)),
    ]));

    img.place(HashMap::from([
        ('c', KittyImageCmdValue::U(16)),
        ('r', KittyImageCmdValue::U(8)),
    ]));



    while let Err(TryRecvError::Empty) = rx.try_recv() {}
}
