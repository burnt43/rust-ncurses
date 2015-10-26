extern crate libc;
use libc::{c_int};

#[test]
fn window_from_initscr_matches_my_machine() {
    unsafe {
        let ptr: *const i16 = initscr(); // this really gives a WINDOW*, but any raw pointer will do initially 
        endwin();
        assert_eq!( *ptr.offset(0), 0 );   // _cury
        assert_eq!( *ptr.offset(1), 0 );   // _curx
        assert_eq!( *ptr.offset(2), 32 );  // _maxy
        assert_eq!( *ptr.offset(3), 134 ); // _maxx
        assert_eq!( *ptr.offset(4), 0 );   // _begy
        assert_eq!( *ptr.offset(5), 0 );   // _begx
    }
}

#[link(name="ncurses")]
extern {
    fn initscr() -> *const i16;
    fn endwin() -> c_int;
}
