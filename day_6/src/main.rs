use core::hash;
use std::collections::HashSet;

use data::load_actual_data;

mod data;
#[derive(Debug,PartialEq,Hash,Eq,Clone, Copy )]
pub struct Position {
    X: i32,
    Y: i32,
}
struct Directions;

impl Directions {
    pub const UP: Position = Position { X: 0, Y: -1 };
    pub const DOWN: Position = Position { X: 0, Y: 1 };
    pub const LEFT: Position = Position { X: -1, Y: 0 };
    pub const RIGHT: Position = Position { X: 1, Y: 0 };
}

impl Position {
    pub fn get_neighbour(&self, dir: &Position) -> Position {
        Position {
            X: self.X + dir.X,
            Y: self.Y + dir.Y,
        }
    }
}
fn main() {
        let map = load_actual_data();
        println!("count: {}", count_positions(&map));
}

fn get_starting_position(map: &Vec<Vec<char>>) -> Position{
    for y in 0..map.len()
    {
        match map[y].iter().position(|&candidate|{candidate == '^'}){
            Some(x) => return Position {X: x as i32, Y: y as i32},
            _ => (),
        }
    }
    Position {X: -1, Y: -1}
}

fn count_positions(map: &Vec<Vec<char>>) -> u32 {
   let mut current_position = get_starting_position(&map);
   let clock_wise_direction = [Directions::UP, Directions::RIGHT, Directions::DOWN, Directions::LEFT];
   let mut positions : HashSet<Position> = HashSet::new();
   positions.insert(current_position);
   let mut next_direction = clock_wise_direction.iter().cycle().peekable();
   'traverse_map: loop {
      'cycle_directions:  loop {
            let candidate_pos = current_position.get_neighbour(&next_direction.peek().unwrap());
            match candidate_pos {
               pos if pos.X < 0 || pos.Y < 0  => break 'traverse_map positions.len() as u32,
               pos if pos.Y >= map.len() as i32 => break 'traverse_map positions.len() as u32,            
               pos if pos.X >= map[0].len() as i32 => break 'traverse_map positions.len() as u32,            
               _ => {},
            };

            match map[candidate_pos.Y as usize][candidate_pos.X as usize]{
                '#'=>{
                    next_direction.next();
                },
                '.'=>{
                    current_position = candidate_pos;
                    positions.insert(current_position);
                    break 'cycle_directions;
                    
                },
                '^'=>{
                    current_position = candidate_pos;
                    positions.insert(current_position);
                    break 'cycle_directions;
                    
                },
                _ =>{},
            }
        } 
   }
}

#[cfg(test)]
mod tests {
    use crate::{data::load_test_data, get_starting_position, Position, count_positions, Directions};
    #[test]
    fn test_count_positions() {
        let map = load_test_data();
        let expected_count: u32 = 41;
        let actual_count = count_positions(&map);
        assert_eq!(expected_count, actual_count);
    }
    
    #[test]
    fn test_get_starting_position(){
        let map = load_test_data();
        let expected_starting_position = Position {X: 4, Y: 6};
        let actual_position= get_starting_position(&map);        
        assert_eq!(actual_position, expected_starting_position);
    }
    #[test]
    fn test_directions(){
        let clock_wise_direction = [Directions::UP, Directions::RIGHT, Directions::DOWN, Directions::LEFT];
        let mut next_direction = clock_wise_direction.iter().cycle().peekable();
        for _i in 0..9{
            println!("{:?}",next_direction.peek().unwrap());
            next_direction.next();
        }
    }
    

}
