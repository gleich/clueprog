pub enum Character {
    MissScarlet,
    MrsWhite,
    MrsPeacock,
    ProfessorPlum,
    MrGreen,
    ColonelMustard,
}

impl Character {
    pub fn name(&self) -> &'static str {
        match &self {
            Character::MissScarlet => "Miss Scarlet",
            Character::MrsWhite => "Mrs. White",
            Character::MrsPeacock => "Mrs. Peacock",
            Character::ProfessorPlum => "Professor Plum",
            Character::MrGreen => "Mr. Green",
            Character::ColonelMustard => "Colonel Mustard",
        }
    }
}

pub enum Weapon {
    Rope,
    LeadPipe,
    Knife,
    Wrench,
    Candlestick,
    Revolver,
}

impl Weapon {
    pub fn name(&self) -> &'static str {
        match &self {
            Weapon::Rope => "Rope",
            Weapon::LeadPipe => "Lead Pipe",
            Weapon::Knife => "Knife",
            Weapon::Wrench => "Wrench",
            Weapon::Candlestick => "Candlestick",
            Weapon::Revolver => "Revolver",
        }
    }
}

pub enum Room {
    BilliardRoom,
    Study,
    Hall,
    Lounge,
    DiningRoom,
    Ballroom,
    Conservatory,
    Library,
    Kitchen,
}

impl Room {
    pub fn room(&self) -> &'static str {
        match &self {
            Room::BilliardRoom => "Billiard Room",
            Room::Study => "Study",
            Room::Hall => "Hall",
            Room::Lounge => "Lounge",
            Room::DiningRoom => "Dining Room",
            Room::Ballroom => "Ballroom",
            Room::Conservatory => "Conservatory",
            Room::Library => "Library",
            Room::Kitchen => "Kitchen",
        }
    }
}
