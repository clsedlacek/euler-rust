fn sum_even_fibonacci_under(max: &i64) -> i64 {
    // initial sum, since we 
    let mut total_sum: i64 = 2;
    let mut prev_terms = (1i64, 2i64);

    loop {
        let next_num: i64 = prev_terms.0 + prev_terms.1;
        println!("Adding: {} + {} = {}", prev_terms.0, prev_terms.1, next_num);
        prev_terms = (prev_terms.1, next_num);
        if next_num >= *max {
            break;
        }
        else if next_num % 2 == 0 {
            total_sum += next_num;
        }
    }

    // return
    total_sum
}

fn main() {
    // exclusive max
    let max: i64 = 4000000;

    println!("Problem 2:");
    println!("Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:");
    println!("1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...");
    println!("By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.");
    println!("Solution: {}", sum_even_fibonacci_under(&max));
}