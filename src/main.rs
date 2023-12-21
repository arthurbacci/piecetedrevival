use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::sync::mpsc::TryRecvError;

use piecetedrevival::bind;
use piecetedrevival::graphics::{KittyImage, KittyImageCmdValue};

fn main() {
    let rx = bind::get_sigwinch_channel();
    println!("{:?}", bind::get_window_sz());


    let mut logo = Vec::new();
    File::open("data/logo.png").unwrap().read_to_end(&mut logo).unwrap();

    let logo = KittyImage::new(logo, HashMap::from([
        ('f', KittyImageCmdValue::U(100)),
    ]));


    let mut c = Vec::new();
    File::open("data/c.png").unwrap().read_to_end(&mut c).unwrap();

    let c = KittyImage::new(c, HashMap::from([
        ('f', KittyImageCmdValue::U(100)),
    ]));

    let c1 = c.place(HashMap::from([
        ('y', KittyImageCmdValue::U(413)),
        ('c', KittyImageCmdValue::U(32)),
        ('r', KittyImageCmdValue::U(16)),
        ('z', KittyImageCmdValue::I(-2)),
        ('C', KittyImageCmdValue::U(1)),
    ]));

    let l1 = logo.place(HashMap::from([
        ('c', KittyImageCmdValue::U(16)),
        ('r', KittyImageCmdValue::U(8)),
    ]));

    let l2 = logo.place(HashMap::from([
        ('c', KittyImageCmdValue::U(16)),
        ('r', KittyImageCmdValue::U(8)),
    ]));



    while let Err(TryRecvError::Empty) = rx.try_recv() {}

    drop(c1);
    drop(l1);
    drop(l2);
}
