use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

mod game_objects;

fn main() -> std::io::Result<()> {
    preload();
    let read = fs::read_to_string("WestOfHouse.json").expect("no read file");
    let current_room: game_objects::Room = serde_json::from_str(&read).unwrap();
    turn(&current_room);

    Ok(())
}

fn turn(room: &game_objects::Room) {
    // room readout
    // player enters command
    // command attempted
    // next turn
    println!("{}", room.name);
    let command = get_command();
    let parse = parse_command(&command, room);
    println!("{}", parse);
}

fn get_command() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    input = input.trim().to_lowercase();
    input
}

fn parse_command(command: &str, room: &game_objects::Room) -> String {
    match command {
        "n" | "north" => match &room.north {
            game_objects::Direction::Room(r) => return String::from(r),
            game_objects::Direction::NoGo(t) => return String::from(t),
        },
        _ => return String::new(),
    }
}

fn preload() {
    // Create all rooms/game objects and write their initial state to a file for later access
    // Each room/game object is scoped so memory is freed after it's written to file

    // West of House
    {
        let leaflet = game_objects::Interactive::Item(game_objects::Item {
            name: String::from("Leaflet"),
            desc: String::from("small leaflet"),
            size: 1,
            value: 1,
            take: true,
        });

        let mailbox = game_objects::Interactive::Container(game_objects::Container {
            name: String::from("Mailbox"),
            desc: String::from("small mailbox"),
            capacity: 5,
            open: false,
            inventory: vec![leaflet],
        });

        let west_of_house = game_objects::Room {
            name: String::from("West of House"),
            file: String::from("WestOfHouse.json"),
            description: String::from(
                "This is an open field west of a white house, with a boarded front door.",
            ),
            inventory: vec![mailbox],
            north: game_objects::Direction::Room(String::from("North of House")),
            northeast: game_objects::Direction::Room(String::from("North of House")),
            east: game_objects::Direction::NoGo(String::from(
                "The door is boarded and you can't remove the boards.",
            )),
            southeast: game_objects::Direction::Room(String::from("South of House")),
            south: game_objects::Direction::Room(String::from("South of House")),
            southwest: game_objects::Direction::NoGo(String::from("You can't go that way.")),
            west: game_objects::Direction::Room(String::from("Forest")),
            northwest: game_objects::Direction::NoGo(String::from("You can't go that way.")),
            up: game_objects::Direction::NoGo(String::from("You can't go that way.")),
            down: game_objects::Direction::NoGo(String::from("You can't go that way.")),
            i: game_objects::Direction::NoGo(String::from("You can't go that way.")),
            o: game_objects::Direction::NoGo(String::from("You can't go that way.")),
        };

        let serialized = serde_json::to_string(&west_of_house).unwrap();
        let mut file = File::create(&west_of_house.file).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}
