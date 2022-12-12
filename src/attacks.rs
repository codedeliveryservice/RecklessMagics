use core::cmp::min;

/// Returns an attack mask from the square in the specified directions, considering the occupancies.
pub fn get_attacks(directions: &[i8], square: i8, occupancies: u64) -> u64 {
    directions.iter().fold(0, |acc, &direction| {
        acc | sliding_attack(square, occupancies, direction)
    })
}

/// Returns a mask of relevant occupations from the square in the specified directions.
pub fn get_relevant_occupancies(directions: &[i8], square: i8) -> u64 {
    directions.iter().fold(0, |acc, &direction| {
        acc | sliding_relevant_occupancies(square, direction)
    })
}

fn sliding_attack(mut square: i8, occupancies: u64, direction: i8) -> u64 {
    let mut output = 0;

    for _ in 0..distance_to_edge(square, direction) {
        let bitboard = 1 << square;
        if bitboard & occupancies != 0 {
            break;
        }

        square += direction;
        output |= bitboard;
    }

    output
}

fn sliding_relevant_occupancies(mut square: i8, direction: i8) -> u64 {
    let mut output = 0;

    for _ in 1..distance_to_edge(square, direction) {
        square += direction;
        output |= 1 << square;
    }

    output
}

/// Returns the ray distance from the square to the edge.
///
/// See [Chess Programming Wiki](https://www.chessprogramming.org/Direction#Ray_Directions)
/// for more information.
///
/// # Panics
///
/// Panics if the specified direction is invalid ray direction.
fn distance_to_edge(square: i8, direction: i8) -> i8 {
    match direction {
        1 => 7 - file(square),
        8 => 7 - rank(square),

        -1 => file(square),
        -8 => rank(square),

        7 => min(7 - rank(square), file(square)),
        9 => min(7 - rank(square), 7 - file(square)),

        -7 => min(rank(square), 7 - file(square)),
        -9 => min(rank(square), file(square)),

        _ => panic!("Unexpected direction '{}'", direction),
    }
}

fn rank(square: i8) -> i8 {
    square / 8
}

fn file(square: i8) -> i8 {
    square % 8
}
