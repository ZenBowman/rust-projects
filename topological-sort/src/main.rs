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
    CountTop { count: 0, top: None }
}

fn add_to_end(top: &mut Option<Box<Successor>>, value: usize) {
    match top.as_mut() {
	Some(v) => add_to_end(&mut v.next, value),
	None => *top = Some(Box::new(Successor {successor_value: value, next: None})),
    }
}

fn print_chain(top: & Option<Box<Successor>>) {
    match top.as_ref() {
	Some(v) => {print!("--> {}", v.successor_value); print_chain(&v.next) },
	None => println!(" =")
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
	    count_array[*p1].top = Some(Box::new(Successor {successor_value: *p2, next: None}));
	} else {
	   add_to_end(&mut count_array[*p1].top, *p2);
	}
    }

    // Constructs a diagram like Figure 8, pg 264, TAOCP Vol 1.
    for i in 1..=numelements {
	print!("{} = {} ] ", i, count_array[i].count);
	print_chain(&count_array[i].top);
    }
    
    return Vec::new();
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
