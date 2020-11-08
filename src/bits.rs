// @date: Nov 6, 2020
// @name: bits.rs
// @desc: Data structs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Food {
    name: String,
    price: f32,
    quanity: i32,
}

#[derive(Debug)]
pub enum Group {
    Fruit(Food),
    Vegetable(Food),
    Protein(Food),
    Dairy(Food),
    Oil(Food),
    Grain(Food),
}

pub fn bits() {
    let recipe = vec!["Fruit"];

    let fruit = String::from("fruit");

    let mut rec = Vec::<Group>::new();

    for i in recipe.iter() {
        if i == &"Fruit" {
            rec.push(Group::Fruit(Food {
                name: String::from("fruit"),
                price: 1.0,
                quanity: 20,
            }));
        }
    }

    rec.push(Group::Dairy(Food {
        name: fruit,
        price: 1.0,
        quanity: 20,
    }));

    let recvec: Vec<_> = rec.into_iter().filter(|r| match r {
            Group::Fruit(_) => true,
            _ => false,
        })
        .collect();

    // for i in rec {
    //   match i {
    //     Group::Fruit(x) => {
    //       println!("{:?}", x);
    //     }
    //     _ => (),
    //   }
    // }
}
