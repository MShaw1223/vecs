fn main() {
    task_four();
    task_five();
}

fn task_four() {
    let mut v = Vec::<i32>::from([1, 2, 3, 4, 5]);
    print!("Task 4 - Double each element in vec\n");
    print!("Original: {:?}\n", v);
    for i in &mut v {
        // as i is &v (ref to v) have to deref to access and use the value : *
        *i *= 2;
    }
    println!("Modified: {:?}", v);
}

fn task_five() {
    let mut v = Vec::<i32>::from([100, 200, 300, 400, 500]);
    print!("Task 5 - Remove specific elements\n");
    print!("Original: {:?}\n", v);
    // closures are where params are specified between the pipes
    v.retain(|&x| x != 200 && x != 400);
    // or could do |x| *x !=200 etc, as &x allows work on the value not the reference
    println!("Modified: {:?}", v);
}
