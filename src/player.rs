use std::io;
use rand::Rng;

use crate::fighter::Fighter

pub struct Player {
    fighter : Fighter,
    money : i64,
    bottles : i64,
    gym_is_found : bool,
    girl_is_found : bool,
    whores_is_found : bool,
    bar_is_found : bool,
    vet_is_found : bool,
    life_style : i8,
    name : String,
}
impl Player {
    fn add_exp (&mut self,extra_exp : i64){
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