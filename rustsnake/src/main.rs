extern crate ncurses;
mod snake;

use ncurses::*;
use rand::Rng;

use snake::*;

enum Dir {
  DirLEFT,
  DirRIGHT,
  DirUP,
  DirDOWN,
}

#[derive(Copy, Clone)]
pub struct Point {
  x: i32,
  y: i32
}

pub struct Food {
  x: i32,
  y: i32,
  active: bool
}

fn update_food(food: &mut Food, snake: &mut Snake) {
  if food.x == snake.get_head().x && food.y == snake.get_head().y {
    snake.inc_score();  
    snake.inc();
    food.active = false;
  }
}


impl Food {
  pub fn update(&mut self, width: i32, height: i32) {
    if self.active == false {
      self.y = rand::thread_rng().gen_range(0..height);
      self.x = rand::thread_rng().gen_range(0..width);
      self.active = true;
    }

    mvprintw(self.y, self.x, "@").unwrap();
  }
}

fn main() {
  initscr();

  keypad(stdscr(), true);
  noecho();
  nodelay(stdscr(), true);

  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

  let mut max_x = 0;
  let mut max_y = 0;

  getmaxyx(stdscr(), &mut max_y, &mut max_x);

  let mut player = Snake::new(Point {x: max_x/2, y: max_y/2}, 0, Dir::DirLEFT, Vec::new());

  let mut food = Food {x: 12, y: 12, active: true};
  let mut ch = 0;


  while ch != 27 {  // press esc to exit
    if !player.check_alive(max_x, max_y) {
      break;
    }

    clear();
    ch = getch();

  
    match ch {
      KEY_LEFT =>
      {
        player.set_dir(Dir::DirLEFT);
      },
      KEY_RIGHT =>
      {
        player.set_dir(Dir::DirRIGHT);
      },
      KEY_UP =>
      {
        player.set_dir(Dir::DirUP);
      },
      KEY_DOWN =>
      {
        player.set_dir(Dir::DirDOWN);
      },
      
      _ => { }
    }
  
    player.update(); 
    food.update(max_x, max_y);
    update_food(&mut food, &mut player);

    player.draw();

    refresh();
    napms(150);
  }
  
  endwin();
}
