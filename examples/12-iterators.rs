fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let results: Vec<_> = numbers.iter()
                                 .map(|x| x * 2)
                                 .filter(|&x| x > 5)
                                 .take(2)
                                 .collect();

    println!("{:?}", results);
}
