fn sum_multiples_under(mfirst: &i32, msecond: &i32, max: &i32) -> i32 {
    let mut sum: i32 = 0;

    for i in 0..*max {
        if i%*mfirst == 0 || i%*msecond == 0 {
            sum = sum + i
        }
    }
    // return
    sum
}

fn main() {
    let max: i32 = 1000;
    let mfirst: i32 = 3;
    let msecond: i32 = 5;

    println!("Problem 1:");
    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.");
    println!("Find the sum of all the multiples of 3 or 5 below {}.", (max - 1));
    println!("The sum is: {}", sum_multiples_under(&mfirst, &msecond, &max));
}
