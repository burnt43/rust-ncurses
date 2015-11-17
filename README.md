# rust-ncurses
This is very basic and not fully implemented yet.

### To use in a project
Add dependency to Cargo.toml
```toml
[dependencies]
ncurses = { git = "https://github.com/burnt43/rust-ncurses" }
```

### Basic Examples
1. Initializing and Ending NCurses
```rust
extern crate ncurses;
use ncurses::Window;

let win: Window = ncurses::initscr();
ncurses::endwin();
```

2. Printing To Window and Waiting For a Keypress
```rust
extern crate ncurses;
use ncurses::Window;

let win: Window = ncurses::initscr();
win.printw("Hello, World");
win.getch();
ncurses::endwin();
```
### Notes
Right now what I'm trying to do is implement the 'w functions', and not their 'w-less' counterparts. This means I don't have a global ```printw```, but instead I implement ```wprintw```, except you call this method on the ```Window``` object as ```printw```. That means if you want to print ```"Hello, World"``` to ```stdscr```, you will have to get the ```Window``` object that represents ```stdscr``` from ```ncurses::initscr()``` and then call ```printw``` on it, which is effectively calling ```printw```. See example 2 above.
