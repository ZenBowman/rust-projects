use std::collections::HashSet;
use std::option;

struct Successor {
    successor_value: usize,
    next: Option<Box<Successor>>,
}

struct CountTop {
    count: usize,
    top: Option<Box<Successor>>,
}

// Constructs an empty CountTop object for initialization.
fn empty_counttop(_unused: usize) -> CountTop {
    CountTop {
        count: 0,
        top: None,
    }
}

// Follow the link of successors until you find an empty slot.
// Operation T3 in Algorithm T.
fn add_to_end(top: &mut Option<Box<Successor>>, value: usize) {
    match top.as_mut() {
        Some(v) => add_to_end(&mut v.next, value),
        None => {
            *top = Some(Box::new(Successor {
                successor_value: value,
                next: None,
            }))
        }
    }
}

fn print_chain(top: &Option<Box<Successor>>) {
    match top.as_ref() {
        Some(v) => {
            print!("--> {}", v.successor_value);
            print_chain(&v.next)
        }
        None => println!(" ="),
    }
}

fn decrement_successors(next: &Option<Box<Successor>>, items_to_dec: &mut Vec<usize>) {
    match next {
	Some(v) => {
	    items_to_dec.push(v.successor_value);
	    decrement_successors(&v.next, items_to_dec);
	},
	None => return,
    }
}

// Implementation of topological sort from Knuth's TAOCP Book 1
// Chapter 2.2.3
fn toposort(ordered_pairs: &[(usize, usize)]) -> Vec<usize> {
    let mut distinct_elements: HashSet<usize> = HashSet::new();
    for (p1, p2) in ordered_pairs {
        distinct_elements.insert(*p1);
        distinct_elements.insert(*p2);
    }
    let numelements = distinct_elements.len();
    println!("Found {} distinct elements", numelements);

    // Being lazy here, we only allocate space for 200 elements.
    // We could use a vector, but staying true to the Knuth example.
    assert!(numelements < 200);

    let mut count_array: [CountTop; 200] = std::array::from_fn(empty_counttop);

    for (p1, p2) in ordered_pairs {
        count_array[*p2].count += 1;
        if count_array[*p1].top.is_none() {
            count_array[*p1].top = Some(Box::new(Successor {
                successor_value: *p2,
                next: None,
            }));
        } else {
            add_to_end(&mut count_array[*p1].top, *p2);
        }
    }

    // Constructs a diagram like Figure 8, pg 264, TAOCP Vol 1.
    for i in 1..=numelements {
        print!("{} = {} ] ", i, count_array[i].count);
        print_chain(&count_array[i].top);
    }

    let mut result: Vec<usize> = Vec::with_capacity(numelements);
    
    while distinct_elements.len() > 0 {
	// Find element with preceding count = 0;
	// Determine the successors, and reduce their count.
	for element in &distinct_elements {
	    if count_array[*element].count == 0 {
		result.push(*element);
		let mut items_to_dec: Vec<usize> = Vec::new();
		decrement_successors(&count_array[*element].top, &mut items_to_dec);
		for successor in items_to_dec {
		    if (count_array[successor].count != 0) {
			count_array[successor].count -= 1;
		    }
		}
		break;
	    }
	}
	// Remove that element from the list.
	match result.last() {
	    Some(v) => distinct_elements.remove(v),
	    None => false,
	};
    }

    println!("\n\nResult\n-----------------\n");
    for item in &result {
	println!("{}", item);
    }
    return result;
}

fn main() {
    // pg 264, equation (18).
    let ordered_pairs = [
        (9, 2),
        (3, 7),
        (7, 5),
        (5, 8),
        (8, 6),
        (4, 6),
        (1, 3),
        (7, 4),
        (9, 5),
        (2, 8),
    ];
    toposort(&ordered_pairs);
}
