extern crate libc;
mod ll;

#[macro_use]
mod attribute;

use std::ffi::CString;
use std::mem;
use libc::{c_short};
use ll::NCursesWindow;
use attribute::{Attribute, ScalarAttribute};

pub struct Window {
    p_window: *const NCursesWindow,
}

impl Window {
    pub fn printw(&self, s: &str) {
        unsafe { ll::wprintw( self.p_window, CString::new(s).unwrap().as_ptr() ); }
    }
    pub fn mvprintw(&self, pos: (i32,i32), s: &str) {
        unsafe { ll::mvwprintw( self.p_window, pos.0, pos.1, CString::new(s).unwrap().as_ptr() ); }
    }
    pub fn getyx(&self) -> (i16,i16) {
        unsafe { ( (*(self.p_window))._cury, (*(self.p_window))._curx ) }
    }
    pub fn getch(&self) {
        unsafe { ll::wgetch(self.p_window); }
    }
    pub fn refresh(&self) {
        unsafe { ll::wrefresh(self.p_window); }
    }
    pub fn keypad(&self, enabled: bool) {
        unsafe { ll::keypad(self.p_window,enabled); }
    }
    pub fn attr_on<T: ScalarAttribute>(&self, attr: T) {
        unsafe { ll::wattr_on(self.p_window, attr.to_attr_t(), 0); }
    }
    pub fn attr_off<T: ScalarAttribute>(&self, attr: T) {
        unsafe { ll::wattr_off(self.p_window, attr.to_attr_t(), 0); }
    }
}

pub fn initscr() -> Window {
    let p_p_window: *const NCursesWindow = unsafe { ll::initscr() };
    Window { p_window: p_p_window }
}

pub fn endwin() {
    unsafe { ll::endwin(); }
}

pub fn raw() {
    unsafe { ll::raw(); }
}

pub fn noraw() {
    unsafe { ll::noraw(); }
}

pub fn cbreak() {
    unsafe { ll::cbreak(); }
}

pub fn nocbreak() {
    unsafe { ll::nocbreak(); }
}

pub fn echo() {
    unsafe { ll::echo(); }
}

pub fn noecho() {
    unsafe { ll::noecho(); }
}

#[test]
fn hello_world() {
    let window: Window = initscr();
    window.keypad(true);
    window.attr_on(attributes![Attribute::Bold, Attribute::Underline]);
    window.printw("THIS SHOULD BE BOLD AND UNDERLINED");
    window.attr_off(attributes![Attribute::Bold, Attribute::Underline]);
    window.mvprintw((1,0),"THIS SHOULD NOT BE BOLD OR UNDERLINED");
    window.refresh();
    window.getch();
    endwin();
}

/*
#[test]
fn memory_layout_of_ncurses_window_is_good () {
    let window: Window = initscr();
    endwin();

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
    let window: Window         = initscr();
    let message_to_print: &str = "for the tests to continue, press any key...";
    window.printw(message_to_print);
    window.mvprintw((5,0),message_to_print);
    window.mvprintw((10,10),message_to_print);
    assert_eq!( window.getyx(), (10,10 + message_to_print.len() as i16) );
    window.refresh();
    window.getch();
    endwin();

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
*/
