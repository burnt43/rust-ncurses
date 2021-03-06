use libc::{c_int,c_short,c_ulong,c_char};
use super::attribute::{ScalarAttribute,Attribute};

pub type size_t = c_short;
pub type chtype = c_ulong;
pub type attr_t = chtype;
pub type pairs_t = c_short;
pub type color_t = c_short;

#[repr(C)]
struct Ldat;

#[repr(C)]
#[derive(Clone)]
pub struct NCursesWindow {
    pub _cury: size_t,
    pub _curx: size_t,
    pub _maxy: size_t,
    pub _maxx: size_t,
    _begy: size_t,
    _begx: size_t,

    _flags: c_short,

    pub _attrs: attr_t,
    _bkgd: chtype,

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

    _regtop: size_t,
    _regbottom: size_t,

    _parx: c_int,
    _pary: c_int,

    _parent: *const NCursesWindow,

    _pad_y: size_t,
    _pad_x: size_t,
    _pad_top: size_t,
    _pad_left: size_t,
    _pad_bottom: size_t,
    _pad_right: size_t,

    _yoffset: size_t,
}

#[link(name="ncurses")]
extern {
    pub fn baudrate() -> c_int;
    pub fn beep() -> c_int;
    pub fn cbreak() -> c_int;
    pub fn echo() -> c_int;
    pub fn endwin()  -> c_int;
    pub fn init_pair(_: pairs_t, _: color_t, _: color_t) -> c_int;
    pub fn initscr() -> *const NCursesWindow;
    pub fn keypad(_: *const NCursesWindow, _: bool) -> c_int;
    pub fn mvwprintw(_: *const NCursesWindow, _: c_int, _: c_int, _: *const c_char) -> c_int;
    pub fn newwin(_: c_int, _: c_int, _: c_int, _: c_int) -> *const NCursesWindow;
    pub fn nocbreak() -> c_int;
    pub fn noecho() -> c_int;
    pub fn noraw() -> c_int;
    pub fn raw() -> c_int;
    pub fn start_color() -> c_int;
    pub fn scrollok(_: *const NCursesWindow, _: bool) -> c_int;
    pub fn waddch(_: *const NCursesWindow, _: chtype);
    pub fn waddchnstr(_: *const NCursesWindow, _: *const chtype, _: c_int) -> c_int;
    pub fn waddnstr(_: *const NCursesWindow, _: *const c_char, _: c_int) -> c_int;
    pub fn wattr_off(_: *const NCursesWindow, _: attr_t, _: c_ulong) -> c_int;
    pub fn wattr_on(_: *const NCursesWindow, _: attr_t, _: c_ulong) -> c_int;
    pub fn wbkgd(_: *const NCursesWindow, _: chtype) -> c_int;
    pub fn wbkgdset(_: *const NCursesWindow, _: chtype);
    pub fn wborder(_: *const NCursesWindow, _: chtype, _: chtype, _: chtype, _: chtype, _: chtype, _: chtype, _: chtype, _: chtype) -> c_int;
    pub fn wgetch(_: *const NCursesWindow) -> c_int;
    pub fn wgetnstr(_: *const NCursesWindow, _: *const c_char, _: c_int) -> c_int;
    pub fn wmove(_: *const NCursesWindow, _: c_int, _: c_int) -> c_int;
    pub fn wprintw(_: *const NCursesWindow, _: *const c_char) -> c_int;
    pub fn wrefresh(_: *const NCursesWindow) -> c_int;
    pub fn wscrl(_: *const NCursesWindow, _: c_int) -> c_int;
}
