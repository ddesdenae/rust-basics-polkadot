fn find_maximum(numbers: &[i32]) -> i32 {
    let mut max = numbers[0];
    for &num in numbers {
        if num > max {
            max = num;
        }
    }
    max
}

fn find_minimum(numbers: &[i32]) -> i32 {
    let mut min = numbers[0];
    for &num in numbers {
        if num < min {
            min = num;
        }
    }
    min
}

fn main() {
    let numbers = [5, 2, 9, 1, 7, 6, 3, 15, -15];
    
    let max = find_maximum(&numbers);
    let min = find_minimum(&numbers);
    
    println!("Maximum value: {}", max);
    println!("Minimum value: {}", min);
}