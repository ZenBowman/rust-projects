use rand;

// MIX bytes are 6 bits long, but we put them into a u8 byte.
struct MixByte {
    bits: u8,
}

enum MixSign {
    PLUS,
    MINUS,
}

// A MIX word is 5 bytes plus a sign. Note that the Rust representation of
// each byte is a u8 value, but the MIX bytes are 6 bits; so interpreting
// the real value of a 2 byte packed value will involve some care.
struct MixWord {
    bytes: [MixByte; 5],
    sign: MixSign,
}

// Each MIX byte is 6 bits, so in order to derive the value of a DOUBLEBYTE,
// we shift the higher order byte by 6 and then do a bitwise OR of the
// lower order byte.
// Big endian order (as in TAOCP).
fn doublebyte_value(byte0: &MixByte, byte1: &MixByte) -> u16 {
    let mut result:u16 = byte0.bits as u16;
    result = (result << 6) | (byte1.bits as u16);
    result
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
    println!(
        "{} = \t{} | {} | {} | {} | {} | {}",
        byte.bits,
        get_bit_at_index(&byte, 0),
        get_bit_at_index(&byte, 1),
        get_bit_at_index(&byte, 2),
        get_bit_at_index(&byte, 3),
        get_bit_at_index(&byte, 4),
        get_bit_at_index(&byte, 5)
    );
}

fn main() {
    println!("First we print a few random bytes");
    for _i in 1..10 {
        let x = MixByte {
            bits: rand::random::<u8>() % 63,
        };
        print_bits(&x);
    }

    println!("----------------------------");
    println!("Then we print some doublebytes");
    let x = MixByte { bits: 0 };
    let y = MixByte { bits: 1 };
    println!("{}", doublebyte_value(&x, &y));
    println!("{}", doublebyte_value(&y, &x));

    let max_x = MixByte { bits: 63 };
    println!("{}", doublebyte_value(&max_x, &max_x));
}
