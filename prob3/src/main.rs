fn check_if_prime(num: &i64) -> bool {
    if *num <= 1 {
        return false;
    }
    else if *num <= 3 {
        return true;
    }

    for i in 1..*num { 
        if *num % i == 0 && i != 1 && i != *num { // exclude 1 and the value of num, since we know those are factors
            return false;
        }
    }

    // number is prime
    true
}

fn find_largest_factor(num: &i64) -> i64 {
    for i in (1..(*num+1)).rev() {
        // factor
        if *num % i == 0 && check_if_prime(&i) {
            // number is a prime factor, this is what we're looking for
            return i;
        }
    }
    // return
    1
}

fn main() {
    let num: i64 = 99999999;

    println!("Problem 3:");
    println!("The prime factors of 13195 are 5, 7, 13 and 29.");
    println!("What is the largest prime factor of the number {} ?", num);
    println!("Answer: {}", find_largest_factor(&num));
}
