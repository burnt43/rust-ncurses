extern crate libc;
use libc::{c_int,c_short};

// obviously not complete
#[repr(C)]
struct Window {
    _cury: c_short,
    _curx: c_short,
    _maxy: c_short,
    _maxx: c_short,
    _begy: c_short,
    _begx: c_short,
}

#[test]
fn window_from_initscr_matches_my_machine() {
    unsafe {
        let window: &Window = initscr();
        endwin();
        assert_eq!( window._cury, 0 );
        assert_eq!( window._curx, 0 );
        assert_eq!( window._maxy, 32 );
        assert_eq!( window._maxx, 134 );
        assert_eq!( window._begy, 0 );
        assert_eq!( window._begx, 0 );
    }
}

#[link(name="ncurses")]
extern {
    fn initscr() -> &Window;
    fn endwin() -> c_int;
}
