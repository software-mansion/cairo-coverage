use readme_example::{calculator, Operation};

#[test]
fn calculator_add() {
    assert(calculator(2, 3, Operation::Add) == 5, '');
    assert(calculator(-1, 1, Operation::Add) == 0, '');
}
