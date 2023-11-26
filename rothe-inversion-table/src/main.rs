use std::vec::Vec;

struct Permutation {
    elements: Vec<u8>,
}

// Single cell in a Rothe inversion table.
// Can be a blank (indicating no inversion),
// a dot (indicating that the permutation has
// the element in this order), or an X (indicating an inversion).
#[derive(Copy, Clone, PartialEq)]
enum RotheInversionElem {
    Blank,
    Dot,
    X,
}

// Rothe's inversion Table from TAOCP #3, section 5.1
// Elements are stored in row-major order.
struct RotheInversionTable {
    dimension: usize,
    elements: Vec<RotheInversionElem>,
}

impl RotheInversionTable {
    fn get(&self, row: usize, col: usize) -> RotheInversionElem {
        let index = row * self.dimension + col;
        assert!(index < self.elements.len());
        self.elements[index]
    }

    fn put(&mut self, row: usize, col: usize, val: RotheInversionElem) {
        let index = row * self.dimension + col;
        assert!(index < self.elements.len());
        self.elements[index] = val
    }

    fn draw(&self) {
        for i in 0..self.dimension {
            print!("\t{0}", i + 1);
        }
        println!();

        for i in 0..self.dimension {
            print!("========");
        }
        println!();

        for i in 0..self.dimension {
            for j in 0..self.dimension {
                match self.get(i, j) {
                    RotheInversionElem::Blank => print!("|\t "),
                    RotheInversionElem::Dot => print!("|\t."),
                    RotheInversionElem::X => print!("|\tx"),
                }
            }
            println!("|")
        }

        for i in 0..self.dimension {
            print!("========");
        }
        println!();
    }

    fn has_inversion(&self, i: usize, j: usize) -> bool {
        if (self.get(i, j) == RotheInversionElem::Dot) {
            return false;
        }

        // As per the algorithm, we first check if there is a dot to the right in the same row.
        let mut dotToRight = false;
        for jprime in j..self.dimension {
            if (self.get(i, jprime) == RotheInversionElem::Dot) {
                // Once we find a dot to the right, we can exit this for loop, but we omit that step here.
                dotToRight = true;
            }
        }

        // If there is no dot to the right, there is no inversion.
        if (!dotToRight) {
            return false;
        }

        // Now we move downward from the cell, and look for a dot. At this point, if there is a dot in the same column,
	// we have an inversion.
        for iprime in i..self.dimension {
            if (self.get(iprime, j) == RotheInversionElem::Dot) {
                return true;
            }
        }

        // This final return covers the case where there is a dot to the right, but not to the bottom.
        return false;
    }
}

fn create_inversion_table(permutation: Permutation) -> RotheInversionTable {
    let dim = permutation.elements.len();
    let elems = vec![RotheInversionElem::Blank; dim * dim];
    let mut table = RotheInversionTable {
        dimension: dim,
        elements: elems,
    };

    // The examples in TAOCP are all 1-indexed, but rust vectors are zero indexed,
    // so we subtract each element by 1.

    // First we place the dots, as per Rothe's algorithm.
    let mut i = 0;
    for element in permutation.elements {
        let e: usize = element.into();
        table.put(i, e - 1, RotheInversionElem::Dot);
        i += 1;
    }

    // Now we detect the inversions. Note that we're not trying to be clever here,
    // we just calculate the value for all non-dot cells, even though anything to the
    // right of a dot could have been ignored.
    for i in 0..dim {
        for j in 0..dim {
            if table.has_inversion(i, j) {
                table.put(i, j, RotheInversionElem::X);
            }
        }
    }

    return table;
}

fn main() {
    // Case 1: Full inversion
    {
        let perm = Permutation {
            elements: vec![5, 4, 3, 2, 1],
        };
	
        let table = create_inversion_table(perm);
        table.draw();
    }

    // Case 2: In-order (no inversions)
    {
        let perm = Permutation {
            elements: vec![1, 2, 3, 4, 5],
        };
        let table = create_inversion_table(perm);
        table.draw();
    }

    // Case 3: Some elements out of order
    {
        let perm = Permutation {
            elements: vec![5, 2, 3, 1, 4],
        };
        let table = create_inversion_table(perm);
        table.draw();
    } 
}
