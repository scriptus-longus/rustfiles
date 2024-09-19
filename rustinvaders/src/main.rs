#![allow(dead_code, unused_variables)]
extern crate ncurses;

use ncurses::*;

fn main() {
  initscr();

  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  //nodelay(stdscr(), true);

  noecho();
  cbreak();

  addstr("Hello world").unwrap();

  refresh();


  getch();
  endwin();

}
