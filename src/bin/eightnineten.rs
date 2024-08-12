fn main() {
    task_eight();
    task_nine();
    task_ten();
}

fn task_eight() {
    let mut v = Vec::<i32>::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    print!("Task 8 - Filtering a vec\n");
    print!("Original: {:?}\n", v);
    v.retain(|&x| x % 2 == 0);
    println!("Filtered: {:?}", v);
}

fn task_nine() {
    let v = Vec::<i32>::from([12, 45, 7, 34, 89, 24, 67]);
    print!("Task 9 - Max value in vec\n");
    print!("Vec: {:?}\n", v);
    let mut m: i32 = v[0];
    for i in v {
        if i > m {
            m = i
        }
    }
    //  or 'if let Some(m) = v.iter().max();'
    println!("Max in vec: {}", m);
}

fn task_ten() {
    print!("Task 10 - Points in vec\n");
    let mut points = Vec::<Point>::new();
    struct Point {
        x: i32,
        y: i32,
    }
    points.push(Point { x: 3, y: 8 });
    points.push(Point { x: 6, y: 4 });
    points.push(Point { x: 9, y: 2 });
    for p in &points {
        println!("Points: X:{} Y:{}", p.x, p.y);
    }
}
