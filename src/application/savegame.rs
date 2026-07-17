use std::fmt::{Debug, Formatter};
use derive::Event;

#[derive(Event)]
pub struct NewHighScore(pub String, pub u32);

#[derive(Clone, PartialEq, Debug)]
pub struct Savegame {
    pub high_scores: [SavableHighScore; 5]
}

#[derive(Clone, PartialEq)]
pub struct SavableHighScore {
    pub name: [u8; 5],
    pub score: u32
}

impl Debug for SavableHighScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.view().fmt(f)
    }
}

impl Savegame {
    pub fn new() -> Self {
        Savegame {
            high_scores: [
                SavableHighScore::new("BETTY", 100_000),
                SavableHighScore::new("TOMMY", 50_000),
                SavableHighScore::new("CILLA", 20_000),
                SavableHighScore::new("LANA", 10_000),
                SavableHighScore::new("MAX", 1_000),
            ]
        }
    }

    pub fn add_score(&mut self, NewHighScore(name, score): &NewHighScore) {
        for i in 0..5 {
            if self.high_scores[i].score < *score {
                for j in (i..4).rev() {
                    self.high_scores[j + 1] = self.high_scores[j].clone();
                }
                self.high_scores[i] = SavableHighScore{ name: str_to_u8_5(&name), score: *score };
                return;
            }
        }
    }
}

fn str_to_u8_5(name_str: &str) -> [u8; 5] {
    let mut name = [' ' as u8; 5];
    for (i, ch) in name_str.as_bytes().iter().enumerate() {
        if i < 5 {
            name[i] = *ch;
        }
    }
    name
}

impl SavableHighScore {
    pub fn new(name_str: &str, score: u32) -> Self {
        let name = str_to_u8_5(name_str);

        SavableHighScore {
            name,
            score
        }
    }

    pub fn view(&self) -> (String, u32) {
        (String::from_utf8(self.name.to_vec()).unwrap(), self.score)
    }
}

#[cfg(test)]
mod tests {
    use crate::application::savegame::{NewHighScore, SavableHighScore, Savegame};

    #[test]
    fn should_not_update_high_scores_if_score_too_low() {
        let mut scores = Savegame { high_scores: [
            SavableHighScore::new("1st", 500),
            SavableHighScore::new("2nd", 400),
            SavableHighScore::new("3rd", 300),
            SavableHighScore::new("4th", 200),
            SavableHighScore::new("5th", 100),
        ]};

        let original_scores = scores.clone();

        scores.add_score(&NewHighScore("6th".to_string(), 50));

        assert_eq!(scores, original_scores)
    }

    #[test]
    fn should_update_high_scores_in_correct_place_and_push_lower_scores_down() {
        let mut scores = Savegame { high_scores: [
            SavableHighScore::new("1st", 500),
            SavableHighScore::new("2nd", 400),
            SavableHighScore::new("3rd", 300),
            SavableHighScore::new("4th", 200),
            SavableHighScore::new("5th", 100),
        ]};

        let expected_scores = Savegame { high_scores: [
            SavableHighScore::new("1st", 500),
            SavableHighScore::new("2nd", 400),
            SavableHighScore::new("2.5th", 350),
            SavableHighScore::new("3rd", 300),
            SavableHighScore::new("4th", 200),
        ]};

        scores.add_score(&NewHighScore("2.5th".to_string(), 350));

        assert_eq!(scores, expected_scores)
    }
}