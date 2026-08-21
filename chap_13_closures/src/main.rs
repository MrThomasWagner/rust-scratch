#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Green,
    Orange,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: &Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut greens = 0;
        let mut oranges = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Green => greens += 1,
                ShirtColor::Orange => oranges += 1,
            }
        }

        if greens >= oranges {
            ShirtColor::Green
        } else {
            ShirtColor::Orange
        }
    }
}

fn main() {
    let inventory = Inventory {
        shirts: vec![
            ShirtColor::Green,
            ShirtColor::Green,
            ShirtColor::Green,
            ShirtColor::Green,
            ShirtColor::Green,
            ShirtColor::Orange,
            ShirtColor::Orange,
            ShirtColor::Orange,
        ],
    };

    let user_preference = Some(ShirtColor::Orange);
    let result = inventory.giveaway(&user_preference);
    println!("User gets a {result:?} shirt");
    println!("User had preference of {user_preference:?} shirt");

    let user_preference2 = None;
    let result = inventory.giveaway(&user_preference2);
    println!("User2 gets a {result:?} shirt");
    println!("User had preference of {user_preference2:?} shirt");
}
