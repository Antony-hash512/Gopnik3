use std::io;
use rand::Rng;

mod fighter;
use crate::fighter::Fighter;

pub struct Player {
    pub fighter : Fighter,
    pub money : i64,
    pub bottles : i64,
    pub gym_is_found : bool,
    pub girl_is_found : bool,
    pub whores_is_found : bool,
    pub bar_is_found : bool,
    pub vet_is_found : bool,
    pub life_style : i8,
    pub name : String,
}
impl Player {
    pub fn add_exp (&mut self,extra_exp : i64){
        let mut exp_to_add_left : i64 = self.fighter.exp + extra_exp;
        while exp_to_add_left > self.fighter.level * self.fighter.level {
            exp_to_add_left -= self.fighter.level * self.fighter.level;
            self.level_up();
        }
        self.fighter.exp += extra_exp;
    }
    fn level_up(&mut self){
        self.fighter.level += 1;
        println!("Ты прокачал уровень, теперь твой уровень: {}", self.fighter.level)
    }
}