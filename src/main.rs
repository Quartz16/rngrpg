extern crate pcg_rand;
extern crate rand;

use rand::{Rng, SeedableRng};
use pcg_rand::Pcg64;
use std::collections::HashSet;
use std::collections::HashMap;



enum EdgeType {
    Solid, //no exiting or entering
    Exit, //can leave but not come back
    Enter, //can enter but only from the other side
    TwoWay, //can both enter and exit
}

struct Edge {
    coord: (i64, i64), //coordinates of edge (e.g. North wall of room (0,0) is (0, 1)
    et: EdgeType, //what type of edge is it currently
    cond: bool, //could this edge change to a different type (conditional)
    cond_et: EdgeType, //if conditional, holds what the edge could change to
}

struct Room {
    coord: (i64, i64), //coordinates of the room (in increments of 2), start is (0,0)
    num_edges_created: u8,
    //move_directions: &Edge Vec, //directions with edgetype TwoWay or Exit, that the player can move in
}

enum Place {
    Room(Room),
    Edge(Edge),
}

struct Map {
    map: HashMap<(i64, i64), Place>,
    name: &'static str,
    //probably add stuff for probabilities here
}

fn init_room(map: &mut HashMap<(i64, i64), Place>) -> &Place {
    let start = Room {coord:(0,0), num_edges_created:0};
    map.insert((0, 0), Place::Room(start));
    return &map[&(0, 0)];
}

fn roll(min: i32, max: i32) -> i32 { 
    let mut pcg = Pcg64::from_entropy();
    let x : f64 = pcg.gen();
    let min_flt = min as f64;
    let max_flt = max as f64;
    let roll_value = ((x * max_flt) + min_flt) as i32;
    println!("Roll: {}", roll_value);
    return roll_value;
    
}

fn attack(ac: i32, roll_mod: i32) -> bool {
    let attack_value: i32 = roll(1, 20) + roll_mod;
    return attack_value > ac;
}

fn get_coord_from_drxn(start_coord:(i64, i64), drxn:u8) -> (i64, i64) {
    match drxn {
            0 => (start_coord.0, start_coord.1-1),
            1 => (start_coord.0 +1, start_coord.1),
            2 => (start_coord.0, start_coord.1 + 1),
            3 => (start_coord.0-1, start_coord.1),
            _ => (1, 1) //if we have more than 4 directions, assign to the top right corner then
    }
}

fn create_first_edge(map: &mut HashMap<(i64, i64), Place>, start: &mut Room) -> u8 {
    let drxn: u8 = roll(0, 3) as u8;
    start.num_edges_created += 1;
    let edge_coord = get_coord_from_drxn(start.coord, drxn);
    let mut new_edge = Edge {
        coord:edge_coord,
        et:EdgeType::TwoWay,
        cond:false,
        cond_et:EdgeType::TwoWay,
    };
    map.insert(edge_coord, Place::Edge(new_edge));
    return drxn;
}

fn create_rem_edges(room: &mut Room, rem_drxns: HashSet<u8>) -> u8 {
    let drxn_size = rem_drxns.len() as i32;
    let drxn: u8 = roll(0, drxn_size-1) as u8;
    return drxn;
}

fn create_map() -> Map {
    let mut map = HashMap::<(i64, i64), Place>::new();
    let mut start = init_room(&mut map);
    let mut start_drxns : HashSet<u8> = [0, 1, 2, 3].iter().cloned().collect();
    let init_drxn = create_first_edge(&mut map, &mut start);
    start_drxns.remove(&init_drxn);
    return Map {
        map,
        name: "The most Based Map You've ever Seen",
    }
}


fn main() {
    println!("{}", attack(14, 5));
    let map = create_map();
    println!("{}\n", map.name);

}
