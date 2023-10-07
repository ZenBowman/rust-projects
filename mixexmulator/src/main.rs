use rand;

struct MixByte {
    // MIX bytes are 6 bits long, but we put them into a u8 byte.
    bits: u8,
}

// Returns 1 if the bit at index i is set, 0 if unset.
// 0 is the highest order bit, 5 is the lowest order bit.
// The actual bits in the MixByte cannot be GT 63 (i.e. the two
// highest order bits in the underlying u8 value are ignored).
fn get_bit_at_index(byte: &MixByte, index: u8) -> u8 {
    assert!(index <= 5);
    assert!(byte.bits <= 63);
    match index {
	5 => 0b00000001 & byte.bits,
	4 => (0b00000010 & byte.bits) >> 1,
	3 => (0b00000100 & byte.bits) >> 2,
	2 => (0b00001000 & byte.bits) >> 3,
	1 => (0b00010000 & byte.bits) >> 4,
	0 => (0b00100000 & byte.bits) >> 5,
	_ => panic!("MIX bytes are 6 bits long"),
    }
}

fn print_bits(byte: &MixByte) {
    println!("{} = \t{} | {} | {} | {} | {} | {}",
	     byte.bits,
	     get_bit_at_index(&byte, 0),
	     get_bit_at_index(&byte, 1),
	     get_bit_at_index(&byte, 2),
	     get_bit_at_index(&byte, 3),
	     get_bit_at_index(&byte, 4),
	     get_bit_at_index(&byte, 5));
}

fn main() {
    for _i in 1..10 {
	let x = MixByte { bits: rand::random::<u8>() % 63 };
	print_bits(&x);
    }
}
