use thiserror::Error;


mod c {
    pub use libc::c_int;

    #[repr(C)]
    pub struct cterm_sz {
        pub row: c_int,
        pub col: c_int,
        pub pxwidth: c_int,
        pub pxheight: c_int,
    }

    extern "C" {
        pub fn cterm_get_sz() -> cterm_sz;
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
/// [^1]:
/// <https://sw.kovidgoyal.net/kitty/graphics-protocol/#getting-the-window-size>
#[doc(alias = "cterm_get_sz")]
pub fn get_window_sz()
    -> Result<(usize, usize, usize, usize), GetWindowSzError> {
    let sz = unsafe {c::cterm_get_sz()};

    if sz.col <= 0 || sz.row <= 0 || sz.pxwidth <= 0 || sz.pxheight <= 0 {
        Err(GetWindowSzError::InvalidSize)
    } else {
        Ok((
            sz.col as usize, sz.row as usize,
            sz.pxwidth as usize, sz.pxheight as usize,
        ))
    }
}


