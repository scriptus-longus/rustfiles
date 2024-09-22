#![allow(dead_code, unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate ncurses;

use ncurses::*;

#[derive(Clone)]
enum Dir {
  dirLEFT,
  dirRIGHT
}

#[derive(Clone)]
pub struct Environment {
  W: i32,
  H: i32
}

pub struct Player {
  x: i32,
  y: i32
}

pub struct Bullet {
  x: i32,
  y: i32
}

#[derive(Clone)]
pub struct Enemy {
  x: i32,
  y: i32,

  dir: Dir 
}

impl Bullet {
  pub fn update(&mut self) {
    self.y -= 1;
  }
}

impl Enemy {
  pub fn update(&mut self) {
    match self.dir {
      Dir::dirLEFT => { self.x -= 1; }
      Dir::dirRIGHT => { self.x += 1; }
    }
  }
}

fn setup_invaders(env: &Environment) -> Vec<Enemy> {
  let mut ret = vec![Enemy {x: 0, y: 0, dir: Dir::dirRIGHT}; 50];
  for i in 0..50 {
    ret[i].y = 5 + 3*(((i as i32) )/10);
    ret[i].x = env.W/2 - 25 + 3*(((i as i32))%10);
    //ret[i] = Enemy {x: x_coord, y: y_coord, dir: Dir::dirRIGHT};
  }
  ret  
}

fn update_invaders(env: &Environment, invaders: &mut Vec<Enemy>) {
  let mut dir =  invaders[0].dir.clone();
  let mut changed = false;

  if invaders[0].x == env.W/2 {
    dir = Dir::dirLEFT; 
    changed = true;
  } else if invaders[0].x == env.W/2 - 25 {
    dir = Dir::dirRIGHT;
    changed = true;
  } 

  for invader in invaders.iter_mut() {
    if changed {
      invader.y += 1;
      invader.dir = dir.clone();
    }

    match dir {
      Dir::dirLEFT => {invader.x -= 1; }
      Dir::dirRIGHT => {invader.x += 1; }
    }
  }
}

fn main() {

  let mut bullets = Vec::<Bullet>::new();  
  let mut env = Environment {W: 0, H: 0};

  initscr();
  
  curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
  nodelay(stdscr(), true);
  noecho();
  cbreak();

  //let mut width = 0i32;
  //let mut height = 0i32;
  //let mut W = W;
  //let mut H = H;

  getmaxyx(stdscr(), &mut env.H, &mut env.W);

  let mut player = Player {x: env.W/2, y: env.H/2};
  let mut invaders = setup_invaders(&env); //vec![ Enemy {x: width/2, y: height/3, dir: Dir::dirRIGHT} ];

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
      _ => {mvprintw(0,0, "hi").unwrap(); }
    } 


    //mvprintw(0,0, &format!("{}", bullets.len())).unwrap();
    //mvprintw(enemy.x, enemy.y, "@");
    for invader in invaders.iter() {
      mvprintw(invader.y, invader.x, "@").unwrap();
    }

    mvprintw(player.y, player.x, "#").unwrap();

    //let keep = bool[invaders.size()];

    for bullet in bullets.iter_mut()  {
        bullet.update(); 
        mvprintw(bullet.y, bullet.x, "o").unwrap();
    }

    bullets.retain(|bullet| bullet.y > 0);

    // rudementary collision detection
    let mut retain_bullets = vec![true; bullets.len()]; 
    let mut retain_invaders = vec![true; invaders.len()];   
 
 
    for (i, bullet) in bullets.iter().enumerate() {
      for (j, invader) in invaders.iter().enumerate() {
        if invader.x == bullet.x && invader.y == bullet.y {
          retain_bullets[i] = false;
          retain_invaders[j] = false;
        }
      }
    }

   let mut iter = retain_bullets.iter();

    bullets.retain(|_| *iter.next().unwrap());
    iter = retain_invaders.iter();

    invaders.retain(|_| *iter.next().unwrap());

    update_invaders(&env, &mut invaders);

    refresh();
    napms(80);
  }

  endwin();

}
