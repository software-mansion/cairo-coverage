# cairo-coverage

## Example

Let's say you have a following program

```rust
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

```

and you cover it with tests

```rust
#[test]
fn calculator_add() {
    assert_eq!(calculator(2, 3, Operation::Add), 5);
    assert_eq!(calculator(-1, 1, Operation::Add), 0);
}
```

When running with `cairo-coverage` you will get a coverage report in `.lcov` format:

```lcov
TN:
SF:/path/to/your/project/src/lib.cairo
FN:7,8,add
FN:11,12,multiply
FN:15,20,calculator
FNDA:2,add
FNDA:0,multiply
FNDA:2,calculator
FNF:3
FNH:2
DA:7,2
DA:8,2
DA:11,0
DA:12,0
DA:15,2
DA:16,2
DA:17,2
DA:18,0
DA:19,2
LF:9
LH:6
end_of_record
```

Let's break it down

### Explanation

1. **General Information**
    - **TN**: Test Name (optional, left empty)
    - **SF**: Source File path `/path/to/your/project/src/lib.rs`

2. **Function Summary**
    - **FNF:3**: The number of functions found in the source file. There are 3 functions: `add`, `multiply`,
      and `calculator`.
    - **FNH:2**: The number of functions that were hit. 2 out of the 3 functions were
      executed: `add` and `calculator`.

3. **Function Details**
    - **FN:7,8,add**: The `add` function starts at line 7 and ends at line 8.
    - **FN:11,12,multiply**: The `multiply` function starts at line 11 and ends at line 12.
    - **FN:15,20,calculator**: The `calculator` function starts at line 15 and ends at line 20.

4. **Function Hit Details**
    - **FNDA:2,add**: The `add` function was executed 2 times.
    - **FNDA:0,multiply**: The `multiply` function was not executed in the tests.
    - **FNDA:2,calculator**: The `calculator` function was executed 2 times.

5. **Line Execution Details**
   - **DA:\<line number\>,\<hit count\>**: Indicates whether each line was executed and how many times.

   Here's the details of line coverage:

   | Line | Hits | Explanation                                      |
         |------|------|--------------------------------------------------|
   | 7    | 2    | Line 7 (definition of `add`) hit 2 times         |
   | 8    | 2    | Line 8 (body of `add`) hit 2 times               |
   | 11   | 0    | Line 11 (definition of `multiply`) not hit       |
   | 12   | 0    | Line 12 (body of `multiply`) not hit             |
   | 15   | 2    | Line 15 (definition of `calculator`) hit 2 times |
   | 16   | 2    | Line 16 (start of `match`) hit 2 times           |
   | 17   | 2    | Line 17 (call to `add` in `match`) hit 2 times   |
   | 18   | 0    | Line 18 (call to `multiply` in `match`) not hit  |
   | 19   | 2    | Line 19 (end of `match`) hit 2 times             |

6. **Line Coverage Summary**
    - **LF:9**: The total number of lines in the file is 9.
    - **LH:6**: The total number of lines that were hit (executed at least once) is 6.

7. **End of Record**
    - **end_of_record**: Indicates the end of this coverage record.

### Summary

1. **Functions Coverage**:
    - Three functions are defined.
    - Two functions are executed at least once: `add` and `calculator`.
    - One function (`multiply`) was not executed.

2. **Line Coverage**:
    - 9 lines of code in total.
    - 6 lines were executed during testing.
    - Lines 11 and 12 (`multiply` function) and line 19 (match arm for `Multiply` operation) were not executed.

