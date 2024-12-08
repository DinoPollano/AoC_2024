mod data;

use crate::data::{load_actual_data, load_test_data};
#[derive(Debug)]
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
    pub const UP_LEFT: Position = Position { X: -1, Y: -1 };
    pub const UP_RIGHT: Position = Position { X: 1, Y: -1 };
    pub const DOWN_LEFT: Position = Position { X: -1, Y: 1 };
    pub const DOWN_RIGHT: Position = Position { X: 1, Y: 1 };
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
    println!("DAY 4");
    let matrix = load_actual_data();
    let count = count_crosses(matrix);
    println!("count: {}", count);
}

pub fn count_crosses(matrix: Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;

    for y in 1..matrix.len()-1 {
        let m_positions: Vec<usize> = matrix[y]
            .iter()
            .enumerate()
            .filter_map(|(index, &ch)| if ch == 'A' { Some(index) } else { None })
            .collect();
        for x in m_positions {
            let a_pos = Position {
                X: x as i32,
                Y: y as i32,
            };
            if is_cross(&matrix, &a_pos){count +=1};
        }
    }

    count
}

pub fn count_words(matrix: Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;

    for y in 0..matrix.len() {
        let x_positions: Vec<usize> = matrix[y]
            .iter()
            .enumerate()
            .filter_map(|(index, &ch)| if ch == 'X' { Some(index) } else { None })
            .collect();
        for x in x_positions {
            let x_pos = Position {
                X: x as i32,
                Y: y as i32,
            };
            for dir in vec![
                Directions::UP,
                Directions::UP_RIGHT,
                Directions::RIGHT,
                Directions::DOWN_RIGHT,
                Directions::DOWN,
                Directions::DOWN_LEFT,
                Directions::LEFT,
                Directions::UP_LEFT,
            ] {
                let (right_direction, m_pos) = check_position(&matrix, &x_pos, &dir, 'M');
                if right_direction {
                    let (right_direction, a_pos) =
                        check_position(&matrix, &m_pos.unwrap(), &dir, 'A');
                    if right_direction {
                        let (right_direction, _) =
                            check_position(&matrix, &a_pos.unwrap(), &dir, 'S');
                        if right_direction {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

pub fn check_position(
    matrix: &Vec<Vec<char>>,
    pos: &Position,
    dir: &Position,
    target: char,
) -> (bool, Option<Position>) {
    let height = matrix.len();
    let width = matrix[0].len();
    let new_pos = pos.get_neighbour(&dir);
    if new_pos.X < 0 || new_pos.Y < 0 || new_pos.X as usize >= width || new_pos.Y as usize >= height
    {
        return (false, None);
    }
    let is_target = matrix[new_pos.Y as usize][new_pos.X as usize] == target;
    (is_target, Some(new_pos))
}

pub fn is_diagonal(matrix: &Vec<Vec<char>>, a_pos: &Position, path: (Position, Position)) -> bool {
    if {
        let (correct, _) = check_position(&matrix, &a_pos, &path.0, 'M');
        correct
    } && {
        let (correct, _) = check_position(&matrix, &a_pos, &path.1, 'S');
        correct
    } {
        true
    } else if {
        let (correct, _) = check_position(&matrix, &a_pos, &path.0, 'S');
        correct
    } && {
        let (correct, _) = check_position(&matrix, &a_pos, &path.1, 'M');
        correct
    } {
        true
    } else {
        false
    }
}

pub fn is_cross (matrix : &Vec<Vec<char>>, a_pos: &Position) -> bool{
    is_diagonal(&matrix, &a_pos, (Directions::UP_LEFT, Directions::DOWN_RIGHT)) && is_diagonal(&matrix, &a_pos, (Directions::DOWN_LEFT, Directions::UP_RIGHT))
}

#[cfg(test)]
mod tests {
    use crate::{
        check_position, count_crosses, count_words, data::load_test_data, is_diagonal, Directions,
        Position, is_cross
    };

    #[test]
    fn test_check_position() {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        matrix.push(vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']);
        matrix.push(vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A']);

        let outline = [
            (Position { Y: 0, X: 4 }, Directions::DOWN_RIGHT, 'M', true),
            (Position { Y: 0, X: 6 }, Directions::RIGHT, 'A', true),
            (Position { Y: 0, X: 6 }, Directions::DOWN, 'S', true),
            (Position { Y: 1, X: 6 }, Directions::UP, 'M', true),
            (Position { Y: 1, X: 6 }, Directions::LEFT, 'M', true),
            (Position { Y: 1, X: 6 }, Directions::UP_LEFT, 'X', true),
            (Position { Y: 0, X: 0 }, Directions::LEFT, 'X', false),
            (Position { Y: 1, X: 0 }, Directions::DOWN, 'X', false),
            (Position { Y: 9, X: 0 }, Directions::RIGHT, 'X', false),
        ];

        for (pos, dir, char_to_look_for, expected_result) in outline {
            let (result, _) = check_position(&matrix, &pos, &dir, char_to_look_for);
            assert_eq!(
                result, expected_result,
                "failed with {:?} {:?} looking for {}",
                &pos, &dir, char_to_look_for
            )
        }
    }

    #[test]
    fn test_count_words() {
        let matrix = load_test_data();
        let expected_count = 18;
        let count = count_words(matrix);
        assert_eq!(count, expected_count);
    }
    #[test]
    fn test_is_diagonal() {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        matrix.push(vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']);
        matrix.push(vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A']);
        matrix.push(vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M']);

        let outline = [
            (
                Position { Y: 1, X: 2 },
                (Directions::UP_LEFT, Directions::DOWN_RIGHT),
                true,
            ),
            (
                Position { Y: 1, X: 2 },
                (Directions::DOWN_LEFT, Directions::UP_RIGHT),
                true,
            ),
        ];
        for (pos, path, expected_result) in outline {
            let result = is_diagonal(&matrix, &pos, path);
            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_is_cross(){
        let mut matrix: Vec<Vec<char>> = Vec::new();
        matrix.push(vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M']);
        matrix.push(vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A']);
        matrix.push(vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M']);

        let outline = [
            (
                Position { Y: 1, X: 2 },
                true,
            ),
            (
                Position { Y: 1, X: 1 },
                false,
            )
        ];
        for (pos,  expected_result) in outline {
            let result = is_cross(&matrix, &pos);
            assert_eq!(result, expected_result);
        }

    }
    #[test]
    fn test_count_crosses() {
        let matrix = load_test_data();
        let expected_count = 9;
        let count = count_crosses(matrix);
        assert_eq!(count, expected_count);
    }
}
