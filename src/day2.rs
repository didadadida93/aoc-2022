use std::fs;

enum Roshambo {
    Rock,
    Paper,
    Scissor,
}

impl Roshambo {
    fn get_value(&self) -> u32 {
        match self {
            Roshambo::Rock => 1,
            Roshambo::Paper => 2,
            Roshambo::Scissor => 3,
        }
    }

    fn against(&self, opponent: &Roshambo) -> u32 {
        match self {
            Roshambo::Rock => match opponent {
                Roshambo::Rock => 3,
                Roshambo::Paper => 0,
                Roshambo::Scissor => 6,
            },
            Roshambo::Paper => match opponent {
                Roshambo::Rock => 6,
                Roshambo::Paper => 3,
                Roshambo::Scissor => 0,
            },
            Roshambo::Scissor => match opponent {
                Roshambo::Rock => 0,
                Roshambo::Paper => 6,
                Roshambo::Scissor => 3,
            },
        }
    }
}

impl TryFrom<String> for Roshambo {
    type Error = String;

    fn try_from(s: String) -> Result<Roshambo, Self::Error> {
        match s.to_lowercase().as_str() {
            "a" | "x" => Ok(Roshambo::Rock),
            "b" | "y" => Ok(Roshambo::Paper),
            "c" | "z" => Ok(Roshambo::Scissor),
            other => Err(format!("Cannot parse {} to Roshambo", other)),
        }
    }
}

fn get_my_roshambo(opponent: &Roshambo, me: String) -> Result<Roshambo, String> {
    match me.to_lowercase().as_str() {
        "x" => match opponent {
            Roshambo::Rock => Ok(Roshambo::Scissor),
            Roshambo::Paper => Ok(Roshambo::Rock),
            Roshambo::Scissor => Ok(Roshambo::Paper),
        },
        "y" => match opponent {
            Roshambo::Rock => Ok(Roshambo::Rock),
            Roshambo::Paper => Ok(Roshambo::Paper),
            Roshambo::Scissor => Ok(Roshambo::Scissor),
        },
        "z" => match opponent {
            Roshambo::Rock => Ok(Roshambo::Paper),
            Roshambo::Paper => Ok(Roshambo::Scissor),
            Roshambo::Scissor => Ok(Roshambo::Rock),
        },
        other => Err(format!("Failed to get my Roshambo from {}", other)),
    }
}

pub fn solve() {
    // A -> Rock
    // B -> Paper
    // C -> Scissor
    // ---
    // X -> Rock
    // Y -> Paper
    // Z -> Scissor
    // ---
    // X -> Lose
    // Y -> Draw
    // Z -> Win
    // ---
    // Rock -> 1
    // Papper -> 2
    // Scissor -> 3
    // ---
    // Lost -> 0
    // Draw -> 3
    // Win -> 6
    let total = fs::read_to_string("./input/day-2.txt")
        .expect("Failed to read day-2.txt file")
        .trim()
        .lines()
        .map(|e| {
            let mut split = e.split(" ");
            let opponent: Roshambo = split
                .next()
                .expect("Failed to get opponent Roshambo")
                .to_string()
                .try_into()
                .unwrap();
            let me: Roshambo = split
                .next()
                .expect("Failed to get my Roshambo")
                .to_string()
                .try_into()
                .unwrap();
            me.against(&opponent) + me.get_value()
        })
        .sum::<u32>();

    let another_total = fs::read_to_string("./input/day-2.txt")
        .expect("Failed to read day-2.txt file")
        .trim()
        .lines()
        .map(|e| {
            let mut split = e.split(" ");
            let opponent: Roshambo = split
                .next()
                .expect("Failed to get opponent Roshambo")
                .to_string()
                .try_into()
                .unwrap();
            let me = split
                .next()
                .expect("Failed to get my Roshambo");
            let me = get_my_roshambo(&opponent, me.to_string()).unwrap();
            me.against(&opponent) + me.get_value()
        })
        .sum::<u32>();

    println!("---------------------------------");
    println!("day 2 puzzle-1 : {}", total);
    println!("day 2 puzzle-2 : {}", another_total);
}
