#![allow(dead_code, unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate ncurses;

use ncurses::*;

pub struct Player {
  x: i32,
  y: i32
}

fn main() {

  initscr();
  
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  nodelay(stdscr(), true);
  noecho();
  cbreak();

  let mut width = 0i32;
  let mut height = 0i32;

  getmaxyx(stdscr(), &mut height, &mut width);

  let mut player = Player {x: width/2, y: height/2};


  let mut ch = 0u8;

  while ch != b'q' {
    clear();

    ch = getch() as u8;

    
    match ch {
      b'j' =>
      {
        player.x -= 1;
      }
      b'k' =>
      {
        player.x += 1;
      }
      b' ' =>
      {
        mvprintw(0,0,"pew").unwrap();
      }
      _ => {}
    } 

    mvprintw(player.y, player.x, "#").unwrap();

    refresh();
    napms(80);
  }

  endwin();

}
