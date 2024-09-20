#![allow(dead_code, unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate ncurses;

use ncurses::*;

pub struct Player {
  x: i32,
  y: i32
}

pub struct Bullet {
  x: i32,
  y: i32,
}

impl Bullet {
  pub fn update(&mut self) {
    self.y -= 1;
  }
}

fn main() {

  let mut bullets = Vec::<Bullet>::new();  

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
        bullets.push(Bullet {x: player.x, y: player.y-1});
      }
      _ => {}
    } 


    //mvprintw(0,0, &format!("{}", bullets.len())).unwrap();
    mvprintw(player.y, player.x, "#").unwrap();

    for bullet in bullets.iter_mut()  {
        bullet.update();
        mvprintw(bullet.y, bullet.x, "o").unwrap();
    }

    bullets.retain(|bullet| bullet.y > 0);


    refresh();
    napms(80);
  }

  endwin();

}
