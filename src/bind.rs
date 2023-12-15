use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::OnceLock;

use thiserror::Error;


mod c {
    pub use libc::c_int;

    extern "C" {
        pub fn cterm_get_sz(
            xc: *mut c_int, yc: *mut c_int, xpx: *mut c_int, ypx: &mut c_int,
        ) -> c_int;

        pub fn cterm_set_sigwinch_callback(_: Option<extern "C" fn(c_int)>)
        -> c_int;
    }
}



#[derive(Debug, Error)]
pub enum GetWindowSzError {
    /// The return of `TIOCGWINSZ` included invalid sizes (minus or equal to zero)
    #[error("Invalid size returned")]
    InvalidSize,
}


/// Gets the size of the terminal window as (columns of characters, rows of
/// characters, width in pixels, height in pixels) according to [^1] (using the
/// `TIOCGWINSZ` signal). Errors if the return of the signal does not represent
/// a valid size.
///
/// There is support for terminals that don't return both the size in
/// characters and in pixels, in this case it will most likely return
/// [InvalidSize][GetWindowSzError::InvalidSize]
///
/// # Panics
///
/// Panics if `ioctl` fails.
///
/// [^1]:
/// <https://sw.kovidgoyal.net/kitty/graphics-protocol/#getting-the-window-size>
#[doc(alias = "cterm_get_sz")]
pub fn get_window_sz()
    -> Result<(usize, usize, usize, usize), GetWindowSzError> {
    let mut rr: (i32, i32, i32, i32) = (0, 0, 0, 0);
    
    let err = unsafe {
        c::cterm_get_sz(&mut rr.0, &mut rr.1, &mut rr.2, &mut rr.3)
    };
    if err != 0 {
        panic!("cterm_get_sz ioctl call returned -1");
    }


    if rr.0 <= 0 || rr.1 <= 0 || rr.2 <= 0 || rr.3 <= 0 {
        Err(GetWindowSzError::InvalidSize)
    } else {
        Ok((rr.0 as usize, rr.1 as usize, rr.2 as usize, rr.3 as usize))
    }
}


static SENDER: OnceLock<Sender<()>> = OnceLock::new();

extern "C" fn sigwinch_callback(_: i32) {
    let _ = SENDER.get().unwrap().send(());
}

#[doc(alias = "cterm_set_sigwinch_callback")]
pub fn get_sigwinch_channel() -> Receiver<()> {
    let (s, r) = channel();
    // TODO: no unwrap()
    SENDER.set(s).unwrap();
    
    let err = unsafe {c::cterm_set_sigwinch_callback(Some(sigwinch_callback))};
    if err != 0 {
        panic!("sigaction returned -1");
    }

    r
}


