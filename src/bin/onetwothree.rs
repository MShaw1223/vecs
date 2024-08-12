fn main() {
    task_one();
    task_two();
    task_three();
}

fn task_one() {
    let mut v = Vec::new();
    let values = [1, 2, 3];
    for i in values {
        v.push(i);
    }
    print!("Task 1 - print vec:\n");
    println!("{:?}", v);
}

fn task_two() {
    let v = Vec::from([10, 20, 30, 40, 50]);
    let third = v[2];
    print!("Task 2 - Access 3rd element in vec\n");
    println!("{:?}", third);
}

fn task_three() {
    let v = Vec::from([5, 10, 15, 20, 25]);
    print!("Task 3 - Print each element in vec\n");
    for el in v {
        println!("{:?}", el);
    }
}
