
fn gcd(mut n: u64, mut m: u64) -> u64 {
   // Assert that both arguments are greater than zero.
   assert!(n > 0);
   assert!(m > 0);

   while m!=0 {
   	 if m < n {
	    let t = m;
	    m = n;
	    n = t;
	 }
	 m = m % n;
   }
   return n;
}

fn main() {
    println!("Namaste, world!");
    println!("The GCD of 10 and 5 is {}",gcd(10, 5));
}
