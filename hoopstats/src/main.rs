use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Player {
    name: String,
    jpg: i32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut players: Vec<Player> = Vec::new();
    let mut file_iterator = reader.lines();
    let mut curr_player = String::from(file_iterator.next().unwrap().unwrap());
    while curr_player != "DONE" {
        let jersey: i32 = i32::from_str(&file_iterator.next().unwrap().unwrap()).unwrap();
        let ppg: i32 = i32::from_str(&file_iterator.next().unwrap().unwrap()).unwrap();
        let jpg = jersey - ppg;
        let player = Player {
            name: curr_player,
            jpg
        };
        players.push(player);
        curr_player = file_iterator.next().unwrap().unwrap();
    }
    players.sort_by(|a, b| b.jpg.cmp(&a.jpg));
    for player in &players {
        println!("{} {}", player.name, player.jpg);
    }

}
