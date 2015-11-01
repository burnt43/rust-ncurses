extern crate libc;
use libc::{c_int,c_short,c_ulong,c_char};
use std::ffi::CString;
use std::mem;

#[repr(C)]
struct Ldat;

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

    _line: *const Ldat,

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
    pub fn refresh(&self) {
        unsafe { wrefresh(self); }
    }
}

pub fn initialize_screen() -> Window {
    let result: &Window = unsafe { initscr() };
    result.clone()
}

pub fn end_window() {
    unsafe { endwin(); }
}

pub fn get_ch() {
    unsafe { getch(); }
}

pub fn will_this_work () {
    let win: &Window = unsafe { initscr() };
    win.printw("Hello");
    win.refresh();
    win.getch();
    unsafe { endwin(); }
}

#[test]
fn window_from_initscr_matches_my_machine() {
    will_this_work();
    /*
    let window: Window         = initialize_screen();
    let message_to_print: &str = "Hello";
    window.printw(message_to_print);
    window.refresh();
    //get_ch();
    //window.getch();
    end_window();

    assert_eq!( mem::size_of::<Window>(), 96 );
    assert_eq!( (&window as *const Window) as usize,      (&(window._cury) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 2,  (&(window._curx) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 4,  (&(window._maxy) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 6,  (&(window._maxx) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 8,  (&(window._begy) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 10, (&(window._begx) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 12, (&(window._flags) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 16, (&(window._attrs) as *const c_ulong) as usize );
    assert_eq!( (&window as *const Window) as usize + 24, (&(window._bkgd) as *const c_ulong) as usize );
    assert_eq!( (&window as *const Window) as usize + 32, (&(window._notimeout) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 33, (&(window._clear) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 34, (&(window._leaveok) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 35, (&(window._scroll) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 36, (&(window._idlok) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 37, (&(window._idcok) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 38, (&(window._immed) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 39, (&(window._sync) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 40, (&(window._use_keypad) as *const bool) as usize );
    assert_eq!( (&window as *const Window) as usize + 44, (&(window._delay) as *const c_int) as usize );
    assert_eq!( (&window as *const Window) as usize + 48, (&(window._line) as *const *const Ldat) as usize );
    assert_eq!( (&window as *const Window) as usize + 56, (&(window._regtop) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 58, (&(window._regbottom) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 60, (&(window._parx) as *const c_int) as usize );
    assert_eq!( (&window as *const Window) as usize + 64, (&(window._pary) as *const c_int) as usize );
    assert_eq!( (&window as *const Window) as usize + 72, (&(window._parent) as *const *const Window) as usize );
    assert_eq!( (&window as *const Window) as usize + 80, (&(window._pad_y) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 82, (&(window._pad_x) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 84, (&(window._pad_top) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 86, (&(window._pad_left) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 88, (&(window._pad_bottom) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 90, (&(window._pad_right) as *const c_short) as usize );
    assert_eq!( (&window as *const Window) as usize + 92, (&(window._yoffset) as *const c_short) as usize );

    assert_eq!( window._cury, 0 );
    assert_eq!( window._curx, message_to_print.len() as i16 );
    assert!( window._maxy > 0 );
    assert!( window._maxx > 0 );
    assert_eq!( window._begy, 0 );
    assert_eq!( window._begx, 0 );
    assert_eq!( window._flags, 14 );
    assert_eq!( window._attrs, 0 );
    assert_eq!( window._bkgd, 32 );
    assert!( !window._notimeout );
    assert!( !window._clear );
    assert!( !window._leaveok );
    assert!( !window._scroll );
    assert!( !window._idlok );
    assert!( window._idcok );
    assert!( !window._immed );
    assert!( !window._sync );
    assert!( !window._use_keypad );
    assert_eq!( window._delay, -1 );
    assert_eq!( window._regtop, 0 );
    assert_eq!( window._regbottom, 32 );
    assert_eq!( window._parx, -1 );
    assert_eq!( window._pary, -1 );
    assert_eq!( window._pad_top, -1 );
    assert_eq!( window._pad_left, -1 );
    assert_eq!( window._pad_bottom, -1 );
    assert_eq!( window._pad_right, -1 );
    assert_eq!( window._yoffset, 0 );
    assert!( window._parent.is_null() );
    assert!( !window._line.is_null() );
    */
}

#[link(name="ncurses")]
extern {
    fn initscr() -> &Window;
    fn endwin()  -> c_int;
    fn wprintw(_: &Window, _: *const c_char) -> c_int;
    fn wgetch(_: &Window) -> c_int;
    fn wrefresh(_: &Window) -> c_int;
    fn getch() -> c_int;
}
