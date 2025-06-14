use std::io;
use rand::Rng;


use crate::location::Location;
use crate::fighter::Fighter;

pub struct Player {
    pub fighter : Fighter,
    pub location : Location,
    pub current_level_exp: i64,
    pub money : i64,
    pub bottles : i64,
    pub gym_is_found : bool,
    pub girl_is_found : bool,
    pub whores_is_found : bool,
    pub bar_is_found : bool,
    pub vet_is_found : bool,
    pub life_style : String,
    pub name : String,
}
impl Player {
    pub fn add_exp (&mut self,extra_exp : i64){
        let mut exp_to_add_left : i64 = extra_exp;
        let mut exp_to_add_next_level : i64 = self.fighter.level * 100;
        while exp_to_add_left > exp_to_add_next_level {
            exp_to_add_left -= exp_to_add_next_level;
            self.level_up();
            exp_to_add_next_level = self.fighter.level * 100;
        }
        self.current_level_exp += exp_to_add_left;
    }
    fn level_up(&mut self){
        self.fighter.level += 1;
        println!("Ты прокачал уровень, теперь твой уровень: {}", self.fighter.level)
    }
}