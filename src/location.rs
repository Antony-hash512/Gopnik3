pub enum Location {
    Street,
    Battle,
    Market,
    Vet,
    Bar,
    Gym
}
impl Location {
    pub fn get_name(&self) -> &str {
        match self {
            Location::Street => "улица",
            Location::Battle => "бой",
            Location::Market => "рынок",
            Location::Vet => "ветеринар",
            Location::Bar => "бар",
            Location::Gym => "тренажерный зал",
        }
    }
    pub fn get_prefix(&self) -> &str {
        match self {
            Location::Street => "Улица> ",
            Location::Battle => "Битва> ",
            Location::Market => "Рынок> ",
            Location::Vet => "Ветеринар> ",
            Location::Bar => "Бар> ",
            Location::Gym => "Тренажерный зал> ",
        }
    }
}