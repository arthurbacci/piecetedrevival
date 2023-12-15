use std::sync::mpsc::TryRecvError;

mod bind;

fn main() {
    let rx = bind::get_sigwinch_channel();
    println!("{:?}", bind::get_window_sz());
    while let Err(TryRecvError::Empty) = rx.try_recv() {

    }
}
