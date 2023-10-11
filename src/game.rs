use anyhow::{Error, Ok};

#[derive(Debug, Default)]
pub enum BeeType {
    #[default]
    Honey,
    Rock,
}

#[derive(Debug, Default)]
pub struct QueenBee {
    harvest_rate: i8,
    r#type: BeeType,
}

impl QueenBee {
    pub fn new(harvest_rate: i8, r#type: BeeType) -> Self {
        Self {
            harvest_rate,
            r#type,
        }
    }
}

#[derive(Debug)]
pub struct BeeHive {
    queen_bee: Option<QueenBee>,
    total_bees: i32,
}

impl BeeHive {
    pub fn set_queen_bee(&mut self, queen_bee: QueenBee) {
        self.queen_bee = Some(queen_bee);
    }
}

#[derive(Default)]
pub struct Game {
    total_honey: i32,
    beehives: Vec<BeeHive>,
    queen_bees: Vec<QueenBee>,
}

impl Game {
    pub fn add_honey(&mut self, honey_to_add: i32) {
        self.total_honey += honey_to_add
        // could this potentially fail if this overflows???
    }

    pub fn remove_honey(&mut self, honey_to_remove: i32) -> Result<(), Error> {
        if self.total_honey < honey_to_remove {
            return Err(Error::msg("Not enough honey!"));
        }
        self.total_honey -= honey_to_remove;
        Ok(())
    }

    pub fn get_total_honey(&self) -> i32 {
        self.total_honey
    }

    pub fn get_all_beehives(&self) -> &Vec<BeeHive> {
        &self.beehives
    }

    pub fn buy_beehive(&mut self) -> String {
        let res = self.remove_honey(10);

        if let Err(e) = res {
            e.to_string()
        } else {
            self.add_beehive();
            "Bought a beehive".to_string()
        }
    }

    pub fn set_queen_bee(&mut self, queen_bee: QueenBee, hive_index: usize) -> Result<(), Error> {
        let num_hives = self.get_all_beehives().len();
        if hive_index >= num_hives {
            return Err(Error::msg("Hive does not exist"));
        }
        let selected_hive = &mut self.beehives[hive_index];
        selected_hive.set_queen_bee(queen_bee);
        Ok(())
    }

    fn add_beehive(&mut self) {
        self.beehives.push(BeeHive {
            queen_bee: None,
            total_bees: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_honey() {
        let mut game = Game {
            total_honey: 100,
            ..Default::default()
        };

        assert!(game.remove_honey(99).is_ok());
        assert!(game.remove_honey(99).is_err());
        assert!(game
            .remove_honey(99)
            .err()
            .is_some_and(|m| m.to_string() == "Not enough honey!"));
    }

    #[test]
    fn test_set_queen_bee() {
        let mut game = Game {
            total_honey: 100,
            ..Default::default()
        };

        game.buy_beehive();
        let result = game.set_queen_bee(QueenBee::new(8, super::BeeType::Honey), 0);
        assert!(result.is_ok())
    }
}
