extern crate libc;
use libc::{c_int,c_short,c_ulong,c_char};
use std::ffi::CString;
use std::mem;

#[repr(C)]
#[derive(Clone)]
pub struct Window {
    _cury: c_short,
    _curx: c_short,
    _maxy: c_short,
    _maxx: c_short,
    _begy: c_short,
    _begx: c_short,

    _flags: c_short,

    _attrs: c_ulong,
    _bkgd: c_ulong,

    _notimeout: bool,
    _clear: bool,
    _leaveok: bool,
    _scroll: bool,
    _idlok: bool,
    _idcok: bool,
    _immed: bool,
    _sync: bool,
    _use_keypad: bool,
    _delay: c_int,

    _line: c_ulong,

    _regtop: c_short,
    _regbottom: c_short,

    _parx: c_int,
    _pary: c_int,

    _parent: *const Window,

    _pad_y: c_short,
    _pad_x: c_short,
    _pad_top: c_short,
    _pad_left: c_short,
    _pad_bottom: c_short,
    _pad_right: c_short,

    _yoffset: c_short,
}

impl Window {
    pub fn printw(&self, s: &str) {
        let c_string: CString = CString::new(s).unwrap();
        unsafe { wprintw(self, c_string.as_ptr()); }
    }
    pub fn getch(&self) {
        unsafe { wgetch(self); }
    }
}

pub fn initialize_screen() -> Window {
    let result: &Window = unsafe { initscr() };
    result.clone()
}

pub fn end_window() {
    unsafe { endwin(); }
}

#[test]
fn window_from_initscr_matches_my_machine() {
    let window: Window = initialize_screen();
    window.printw("Hello");
    end_window();
    assert_eq!( mem::size_of::<Window>(), 96 );
    assert_eq!( window._cury, 0 );
    assert_eq!( window._curx, 5 );
    assert_eq!( window._maxy, 32 );
    assert_eq!( window._maxx, 134 );
    assert_eq!( window._begy, 0 );
    assert_eq!( window._begx, 0 );
    assert_eq!( window._flags, 14 );
    assert_eq!( window._attrs, 0 );
    assert_eq!( window._bkgd, 32 );
}

#[link(name="ncurses")]
extern {
    fn initscr() -> &Window;
    fn endwin()  -> c_int;
    fn wprintw(window: &Window, s: *const c_char) -> c_int;
    fn wgetch(window: &Window) -> c_int;
}
