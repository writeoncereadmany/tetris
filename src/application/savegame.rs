use derive::Event;

#[derive(Event)]
pub struct NewHighScore(pub String, pub u32);

#[derive(Clone)]
pub struct Savegame {
    pub high_scores: [SavableHighScore; 5]
}

#[derive(Clone)]
pub struct SavableHighScore {
    pub name: [u8; 5],
    pub score: u32
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
            }
            self.high_scores[i] = SavableHighScore{ name: str_to_u8_5(&name), score: *score };
            return;
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
}