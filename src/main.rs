mod attacks;
mod entry;
mod generator;
mod random;

use crate::random::Random;

const ROOK_DIRECTIONS: &[i8; 4] = &[1, 8, -8, -1];
const BISHOP_DIRECTIONS: &[i8; 4] = &[7, 9, -9, -7];

fn main() {
    print_struct();
    print_magics(ROOK_DIRECTIONS, "ROOK");
    print_magics(BISHOP_DIRECTIONS, "BISHOP");
}

fn print_struct() {
    println!("#[rustfmt::skip]");
    println!("pub struct MagicEntry {{ pub mask: u64, pub magic: u64, pub shift: u32, pub offset: usize }}");
    println!();
}

fn print_magics(directions: &[i8], name: &str) {
    println!("#[rustfmt::skip]");
    println!("pub const {}_MAGICS: [MagicEntry; 64] = [", name);

    let mut random = Random::new();
    let mut offset = 0;

    for square in 0..64 {
        let entry = generator::generate_magic_number(directions, square, &mut random);
        println!(
            "    MagicEntry {{ mask: 0x{:0>16X}, magic: 0x{:0>16X}, shift: {}, offset: {} }},",
            entry.mask, entry.magic, entry.shift, offset
        );

        offset += entry.size;
    }

    println!("];");
    println!();

    println!("pub const {}_MAP_SIZE: usize = {};", name, offset);
    println!();
}
