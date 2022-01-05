use std::ops::Add;

use clap::{Arg, App};
use rand::Rng;

fn main() {
    let matches = App::new("My Super Program")
        .arg(Arg::new("capitalizahh").multiple_occurrences(true))
        .get_matches();
    let mut rand = rand::thread_rng();

    let iterator = matches.values_of("capitalizahh");
    for el in iterator.unwrap() {
        let mut res = String::new();
        for x in el.chars() {
            if rand.gen_bool(0.5) == true {
                let y = x.to_ascii_uppercase();
                res.push_str(format!("{}", y).as_str());
            } else {
                res.push_str(format!("{}", x).as_str());
            }

        }
        println!("{}", res);
    }
}
