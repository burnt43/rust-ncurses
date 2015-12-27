extern crate libc;
mod ll;

#[macro_use]
mod attribute;

use std::ffi::CString;
use std::mem;
use libc::{c_short,c_char};
use ll::{NCursesWindow,attr_t,chtype};
use attribute::{Attribute, ScalarAttribute};

pub struct Window {
    p_window: *const NCursesWindow,
}

impl Window {

    pub fn addch<T: ScalarAttribute>(&self, attr: T) {
        unsafe { ll::waddch(self.p_window, attr.to_attr_t() ); }
    }

    pub fn addchnstr(&self, chars: Vec<chtype>, num_chars: i32) {
        let mut m_chars : Vec<chtype> = chars;
        m_chars.push(0);
        unsafe { ll::waddchnstr(self.p_window, m_chars.as_ptr(), num_chars); }
    }

    pub fn addchstr(&self, chars: Vec<chtype>) {
        let mut m_chars: Vec<chtype> = chars;
        m_chars.push(0);
        unsafe { ll::waddchnstr(self.p_window, m_chars.as_ptr(), -1); }
    }

    pub fn addnstr(&self, string: &str, num_chars: i32) {
        unsafe { ll::waddnstr(self.p_window, CString::new(string).unwrap().as_ptr(), num_chars); }
    }

    pub fn addstr(&self, string: &str) {
        unsafe { ll::waddnstr(self.p_window, CString::new(string).unwrap().as_ptr(), -1); }
    }

    pub fn attr_off<T: ScalarAttribute>(&self, attr: T) {
        unsafe { ll::wattr_off(self.p_window, attr.to_attr_t(), 0); }
    }

    pub fn attr_on<T: ScalarAttribute>(&self, attr: T) {
        unsafe { ll::wattr_on(self.p_window, attr.to_attr_t(), 0); }
    }
    
    pub fn border<T: ScalarAttribute>(&self, left_side: T, right_side: T, top_side: T, bottom_side: T, top_left: T, top_right: T, bottom_left: T, bottom_right: T) {
        unsafe { ll::wborder(self.p_window, left_side.to_attr_t(), right_side.to_attr_t(), top_side.to_attr_t(), bottom_side.to_attr_t(), top_left.to_attr_t(), top_right.to_attr_t(), bottom_left.to_attr_t(), bottom_right.to_attr_t() ); }
    }

    pub fn boxify<T: ScalarAttribute>(&self, vertical: T, horizontal: T) {
        unsafe { ll::wborder(self.p_window, vertical.to_attr_t(), vertical.to_attr_t(), horizontal.to_attr_t(), horizontal.to_attr_t(),0,0,0,0); }
    }
    
    pub fn getch(&self) -> Option<char> {
        std::char::from_u32( unsafe { ll::wgetch(self.p_window) } as u32 )
    }
    
    pub fn getmaxyx(&self) -> (i16,i16) {
        unsafe { ( (*(self.p_window))._maxy + 1, (*(self.p_window))._maxx + 1) }
    }
    
    pub fn getnstr(&self, num_chars: i32) -> Result<String,std::str::Utf8Error> {
        let buffer: Vec<u8> = vec![1;128];
        let p_str: *mut c_char = CString::new(buffer).unwrap().into_raw();
        unsafe { ll::wgetnstr(self.p_window, p_str, num_chars); }
        let read_string: CString = unsafe { CString::from_raw(p_str) };
        let result: &str = try!( read_string.to_str() );
        Ok(result.to_string())
    }
    
    pub fn getyx(&self) -> (i16,i16) {
        unsafe { ( (*(self.p_window))._cury, (*(self.p_window))._curx ) }
    }
    
    pub fn keypad(&self, enabled: bool) {
        unsafe { ll::keypad(self.p_window,enabled); }
    }
    
    pub fn mv(&self, pos: (i32,i32)) {
        unsafe { ll::wmove(self.p_window,pos.0,pos.1); }
    }
    
    pub fn mvprintw(&self, pos: (i32,i32), s: &str) {
        unsafe { ll::mvwprintw( self.p_window, pos.0, pos.1, CString::new(s).unwrap().as_ptr() ); }
    }

    pub fn new( dimensions: (i32,i32), starting_point: (i32,i32) ) -> Window {
        unsafe { 
            Window { p_window: ll::newwin( dimensions.0, dimensions.1, starting_point.0, starting_point.1 ) } 
        }
    }

    pub fn printw(&self, s: &str) {
        unsafe { ll::wprintw( self.p_window, CString::new(s).unwrap().as_ptr() ); }
    }

    pub fn refresh(&self) {
        unsafe { ll::wrefresh(self.p_window); }
    }

    pub fn scrl(&self, n: i32) {
        unsafe { ll::wscrl(self.p_window,n); }
    }

    pub fn scrollok(&self, ok: bool) {
        unsafe { ll::scrollok(self.p_window, ok); }
    }

}

// Global Functions
pub fn baudrate() -> i32 {
    unsafe { ll::baudrate() }
}

pub fn beep() {
    unsafe { ll::beep(); }
}

pub fn cbreak() {
    unsafe { ll::cbreak(); }
}

pub fn echo() {
    unsafe { ll::echo(); }
}

pub fn endwin() {
    unsafe { ll::endwin(); }
}

pub fn initscr() -> Window {
    let p_p_window: *const NCursesWindow = unsafe { ll::initscr() };
    Window { p_window: p_p_window }
}

pub fn nocbreak() {
    unsafe { ll::nocbreak(); }
}

pub fn noecho() {
    unsafe { ll::noecho(); }
}

pub fn noraw() {
    unsafe { ll::noraw(); }
}

pub fn raw() {
    unsafe { ll::raw(); }
}

#[test]
fn hello_world() {
    let window: Window = initscr();
    window.keypad(true);
    window.scrollok(true);
    window.attr_on(Attribute::Bold | Attribute::Underline);
    window.printw("THIS SHOULD BE BOLD AND UNDERLINED");
    window.attr_off(Attribute::Bold | Attribute::Underline);
    window.mvprintw((1,0),"THIS SHOULD NOT BE BOLD OR UNDERLINED");
    window.mvprintw((3,0),"Press 'x' to exit ");
    window.addch('a' | Attribute::Bold | Attribute::Underline);
    window.addch('b');
    window.addnstr("This is a really long string and it has a lot of characters and I am curious to see what will happen when it reaches the edge of the screen how will ncurses to if I give it -1 as the second parameter?",-1);
    window.mvprintw((12,0),&format!("Max Y: {}",window.getmaxyx().0));
    window.mvprintw((13,0),&format!("Max X: {}",window.getmaxyx().1));
    window.mvprintw((25,0),&format!("baudrate: {}",baudrate()));
    window.mv( (24,0) );
    window.addchnstr(chtype_vec!['a', 'b', 'c', 'd', 'e'],4);
    window.mv( (23,0) );
    window.addchnstr(string_as_chtype!("HELLO!"),-1);
    window.mv((22,0));
    window.addchstr(string_as_chtype!("Line 22"));
    window.mv((21,0));
    window.addstr("Line 21");
    window.mv( (15,100) );
    window.addch('J');
    window.mv( (16,101) );
    window.addch('A');
    window.mvprintw((14,0),"Please type a string: ");
    echo();
    window.refresh();

    let other_win: Window = Window::new( (5,50), (15,75) );
    other_win.boxify(0,0);
    other_win.mvprintw((1,1),"AAA");
    other_win.refresh();

    match window.getnstr(-1) {
        Ok(s) => window.mvprintw((15,0),&s),
        Err(_) => {},
    }
    noecho();
    loop {
        match window.getch() {
            Some('x') => break,
            Some('k') => {
                beep();
                window.scrl(1);
                window.refresh();
            },
            Some('j') => {
                beep();
                window.scrl(-1);
                window.refresh();
            }
            Some(_)   => {},
            None      => {},
        }
    }
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
