fn main() {
    let vec = vec![10, 20, 30];
    let result = find_largest(&vec);

    println!("{result}");
}

fn find_largest(vec: &[i32]) -> &i32 {
    let mut largest = &vec[0];
    for element in vec {
        if element > largest {
            largest = element;
        }
    }
    largest
}
