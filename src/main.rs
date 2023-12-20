use std::fs::File;
use std::io::Read;

use piecetedrevival::bind;
use piecetedrevival::graphics::{KittyImage};

fn main() {
    //let rx = bind::get_sigwinch_channel();
    println!("{:?}", bind::get_window_sz());


    let mut img = Vec::new();
    let mut f = File::open("data/logo.png").unwrap();
    f.read_to_end(&mut img).unwrap();
    drop(f);

    let img = KittyImage::new(img);



    //while let Err(TryRecvError::Empty) = rx.try_recv() {}
}
