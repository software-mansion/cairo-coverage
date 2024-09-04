pub enum Operation {
    Add,
    Multiply,
}


fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn calculator(a: i32, b: i32, operation: Operation) -> i32 {
    match operation {
        Operation::Add => add(a, b),
        Operation::Multiply => multiply(a, b),
    }
}
