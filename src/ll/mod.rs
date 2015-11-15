use libc::{c_int,c_short,c_ulong,c_char};

#[repr(C)]
struct Ldat;

#[repr(C)]
#[derive(Clone)]
pub struct NCursesWindow {
    pub _cury: c_short,
    pub _curx: c_short,
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

    _parent: *const NCursesWindow,

    _pad_y: c_short,
    _pad_x: c_short,
    _pad_top: c_short,
    _pad_left: c_short,
    _pad_bottom: c_short,
    _pad_right: c_short,

    _yoffset: c_short,
}

#[link(name="ncurses")]
extern {
    pub fn initscr() -> *const NCursesWindow;
    pub fn endwin()  -> c_int;
    pub fn wprintw(_: *const NCursesWindow, _: *const c_char) -> c_int;
    pub fn wgetch(_: *const NCursesWindow) -> c_int;
    pub fn wrefresh(_: *const NCursesWindow) -> c_int;
    pub fn mvwprintw(_: *const NCursesWindow, _: c_int, _: c_int, _: *const c_char) -> c_int;
    pub fn keypad(_: *const NCursesWindow, _: bool) -> c_int;
    pub fn raw() -> c_int;
    pub fn noraw() -> c_int;
    pub fn cbreak() -> c_int;
    pub fn nocbreak() -> c_int;
    pub fn echo() -> c_int;
    pub fn noecho() -> c_int;
}
