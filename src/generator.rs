//! Implementation of the magic number generation algorithm.
//!
//! See [Chess Programming Wiki article](https://www.chessprogramming.org/Looking_for_Magics#Feeding_in_Randoms)
//! for more information.

use crate::{attacks, entry::MagicEntry, random::Random};

/// Generates a magic number for the square in the specified directions.
///
/// # Panics
///
/// Panics if the magic number was not found due to unexpected reasons.
pub fn generate_magic_number(directions: &[i8], square: i8, random: &mut Random) -> MagicEntry {
    let mask = attacks::get_relevant_occupancies(directions, square);
    let size = 1 << mask.count_ones();

    let expected = generate_attacks(size, directions, square, mask);

    // Use some kind of safety counter
    for _ in 0..1_000_000 {
        // The magic number candidate should have a low number of non-zero bits
        let magic = random.next_u64() & random.next_u64() & random.next_u64();

        // Skip inappropriate magic numbers
        if (mask.wrapping_mul(magic) & 0xFF00_0000_0000_0000).count_ones() < 6 {
            continue;
        }

        if try_magic_number(mask, magic, &expected).is_ok() {
            let shift = 64 - mask.count_ones();
            let size = expected.len();

            return MagicEntry::new(mask, magic, shift, size);
        }
    }

    // We should never get here due to the use of PRNG!
    panic!("Magic number not found")
}

/// Generate an attack map for the square in the specified directions.
fn generate_attacks(size: usize, directions: &[i8], square: i8, mask: u64) -> Vec<u64> {
    let mut map = vec![0; size];

    // Populate the expected attacks for all occupancy sets by iterating over all subsets of the attack mask
    // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set#All_Subsets_of_any_Set
    let mut occupancies = 0u64;
    for attacks in map.iter_mut() {
        *attacks = attacks::get_attacks(directions, square, occupancies);
        occupancies = occupancies.wrapping_sub(mask) & mask;
    }
    map
}

/// The error type returned when magic number fails and causes a collision.
#[derive(Debug, Copy, Clone)]
struct MagicNumberCollision;

/// Determines if the magic number works for all squares and possible occupancies.
///
/// # Errors
///
/// This function will return an error if the magic number caused a collision.
fn try_magic_number(mask: u64, magic: u64, expected: &[u64]) -> Result<(), MagicNumberCollision> {
    let shift = 64 - mask.count_ones();

    let mut actual = vec![0; expected.len()];
    let mut occupancies = 0u64;

    for &attacks in expected.iter() {
        let hash = (occupancies.wrapping_mul(magic) >> shift) as usize;

        if is_collision_detected(&actual, hash, attacks) {
            return Err(MagicNumberCollision);
        }

        actual[hash] = attacks;
        occupancies = occupancies.wrapping_sub(mask) & mask;
    }

    Ok(())
}

/// Determines if the given hash causes a collision.
#[inline(always)]
fn is_collision_detected(actual: &[u64], hash: usize, attacks: u64) -> bool {
    actual[hash] != 0 && actual[hash] != attacks
}
