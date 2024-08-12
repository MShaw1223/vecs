fn main() {
    task_six();
    task_seven();
}

fn task_six() {
    let mut v = Vec::<i32>::from([1, 2, 3]);
    let mut second = Vec::<i32>::from([4, 5, 6]);
    print!("Task 6 - Combine vecs\n");
    print!("Original\nv: {:?}\nsec:{:?}\n", v, second);
    v.append(&mut second);
    println!("Combined\nv: {:?}\nsec:{:?}", v, second);
}

fn task_seven() {
    let mut v = Vec::<i32>::from([3, 1, 4, 1, 5, 9, 2, 6, 5]);
    print!("Task 7 - Order Vec elements\n");
    print!("Original: {:?}\n", v);
    v.sort();
    println!("Modified: {:?}", v);
}
