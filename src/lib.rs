extern crate libc;
pub mod ll;

use std::ffi::CString;
use std::mem;
use libc::{c_short};
use ll::NCursesWindow; 

pub struct Window {
    p_window: *const NCursesWindow,
}

impl Window {
    pub fn printw(&self, s: &str) {
        let c_string: CString = CString::new(s).unwrap();
        unsafe { ll::wprintw(self.p_window, c_string.as_ptr()); }
    }
    pub fn getch(&self) {
        unsafe { ll::wgetch(self.p_window); }
    }
    pub fn refresh(&self) {
        unsafe { ll::wrefresh(self.p_window); }
    }
}

pub fn initialize_screen() -> Window {
    let p_p_window: *const NCursesWindow = unsafe { ll::initscr() };
    Window { p_window: p_p_window }
}

pub fn end_window() {
    unsafe { ll::endwin(); }
}

#[test]
fn memory_layout_of_ncurses_window_is_good () {
    let window: Window = initialize_screen();
    end_window();

    assert_eq!( mem::size_of::<NCursesWindow>(), 96 );
    assert_eq!( window.p_window as usize + 0, ( unsafe {&((*(window.p_window))._cury)} as *const c_short) as usize );
    assert_eq!( window.p_window as usize + 2, ( unsafe {&((*(window.p_window))._curx)} as *const c_short) as usize );
    /*
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
    */
}

#[test]
fn basic_ncurses_functions_do_not_break() {
    let window: Window         = initialize_screen();
    let message_to_print: &str = "for the tests to continue, press any key...";
    window.printw(message_to_print);
    window.refresh();
    window.getch();
    end_window();

    /*
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
