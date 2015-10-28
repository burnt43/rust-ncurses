extern crate libc;
use libc::{c_int,c_short,c_ulong};
use std::mem;

#[repr(C)]
struct Window {
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
    fn initscr() -> Window {
        let result: &Window = unsafe { initscr() };
        Window {
            _cury: result._cury,
            _curx: result._curx,
            _maxy: result._maxy,
            _maxx: result._maxx,
            _begy: result._begy,
            _begx: result._begx,

            _flags: result._flags,

            _attrs: result._attrs,
            _bkgd: result._bkgd,

            _notimeout: result._notimeout,
            _clear: result._clear,
            _leaveok: result._leaveok,
            _scroll: result._scroll,
            _idlok: result._idlok,
            _idcok: result._idcok,
            _immed: result._immed,
            _sync: result._sync,
            _use_keypad: result._use_keypad,
            _delay: result._delay,

            _line: result._line,

            _regtop: result._regtop,
            _regbottom: result._regbottom,

            _parx: result._parx,
            _pary: result._pary,

            _parent: result._parent,

            _pad_y: result._pad_y,
            _pad_x: result._pad_x,
            _pad_top: result._pad_top,
            _pad_left: result._pad_left,
            _pad_bottom: result._pad_bottom,
            _pad_right: result._pad_right,

            _yoffset: result._yoffset,
        }
    }
    fn endwin() {
        unsafe { endwin() };
    }
}

#[test]
fn window_from_initscr_matches_my_machine() {
    let window: Window = Window::initscr();
    Window::endwin();
    assert_eq!( mem::size_of::<Window>(), 96 );
    assert_eq!( window._cury, 0 );
    assert_eq!( window._curx, 0 );
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
    fn endwin() -> c_int;
}
