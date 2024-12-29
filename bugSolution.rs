fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    if index < numbers.len() {
        println!("Value at index {}: {}", index, numbers[index]);
    } else {
        println!("Index {} is out of bounds", index);
    }
} 