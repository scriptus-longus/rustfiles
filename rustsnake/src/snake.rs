use ncurses::*;
use super::*;

pub struct Snake {
  head: Point,
  score: u32,
  dir: Dir,
  body: Vec<Point>
}

impl Snake {
  pub fn new(head: Point, score: u32, dir: Dir, body: Vec<Point>) -> Self{
    Self {head: head, score: score, dir: dir, body: body}
  }

  pub fn get_head(&self) -> &Point {
    &self.head
  }

  pub fn set_dir(&mut self, dir: Dir) {
    self.dir = dir;
  }

  pub fn inc_score(&mut self) {
    self.score += 1;
  }

  pub fn inc(&mut self) {
    self.body.push(Point {x: 0, y: 0});
  }

  pub fn update(&mut self) {
    let mut prev_p: Point = self.head;

    match self.dir {
      Dir::DirLEFT => {self.head.x -= 1; } 
      Dir::DirRIGHT => {self.head.x += 1; } 
      Dir::DirUP => {self.head.y -= 1; } 
      Dir::DirDOWN => {self.head.y += 1; } 
    }

    for p in self.body.iter_mut() {
      let tmp = Point {x: p.x, y: p.y};
      *p = prev_p;
      prev_p = tmp;

    }
  }

  pub fn check_alive(&self, width: i32, height: i32) -> bool {
    if self.head.x < 0 || self.head.x > width || self.head.y < 0 || self.head.y > height {
      return false
    }
    return true
  } 

  pub fn draw(&self) {
    mvprintw(0, 0, &format!("score: {}", self.score)).unwrap();
    mvprintw(self.head.y, self.head.x, "#").unwrap();
    for p in self.body.iter() {
      mvprintw(p.y, p.x, "#").unwrap();
    }
  }
}
/*
fn update_food(food: &mut Food, snake: &mut Snake) {
  if food.x == snake.head.x && food.y == snake.head.y {
   snake.score += 1;  
   food.active = false;
   snake.body.push(Point {x: 5, y: 5} );

  }
}*/
