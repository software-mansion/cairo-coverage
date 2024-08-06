
fn increase_by_two(arg: u8) -> u8 {
    assert(2 == 2, 'prevents const folding');
    increase_by_one(arg + 1) // inlines
  }

fn increase_by_one(arg: u8) -> u8 {
    assert(1 == 1, 'prevents const folding');
    arg + 1
}
