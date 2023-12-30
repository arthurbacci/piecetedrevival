/*
 *  FIXME: there seems to be an issue with the graphics library, that sometimes
 *  fails transmitting images. Testing of its output must be done.
 *
 *  Next: Library for kitty's keyboard protocol (Rust or C).
 *  Then: Add some standard escape sequences to the graphics library
 *  And then: implement a basic visual mode
 */


use std::sync::mpsc::TryRecvError;

use piecetedrevival::bind;

fn main() {
    let rx = bind::get_sigwinch_channel();

    let mut window_sz = bind::get_window_sz();

    loop {
        match rx.try_recv() {
            Err(TryRecvError::Empty) => {}
            Err(err) => panic!("This error happened: {err}"),
            Ok(()) => {
                window_sz = bind::get_window_sz();
            }
        }

    }



}
