use std::sync::mpsc::TryRecvError;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use piecetedrevival::bind;
use piecetedrevival::graphics::{kitty_image_write, KittyImageCmdValue};

fn main() {
    let rx = bind::get_sigwinch_channel();
    println!("{:?}", bind::get_window_sz());


    let mut img = Vec::new();
    let mut f = File::open("data/logo.png").unwrap();
    f.read_to_end(&mut img).unwrap();
    drop(f);

    kitty_image_write(
        &img,
        HashMap::from([
            ('f', KittyImageCmdValue::U(100)),
            ('a', KittyImageCmdValue::C('T')),
        ]),
    ).unwrap();




    while let Err(TryRecvError::Empty) = rx.try_recv() {

    }
}
