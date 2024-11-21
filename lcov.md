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
    assert(calculator(2, 3, Operation::Add) == 5, '');
    assert(calculator(-1, 1, Operation::Add) == 0, '');
}
```

When running with `cairo-coverage` you will get a coverage report in `.lcov` format:

```lcov
TN:
SF:/path/to/your/project/src/lib.rs
FN:8,readme_example::add
FNDA:2,readme_example::add
FN:16,readme_example::calculator
FNDA:4,readme_example::calculator
FN:12,readme_example::multiply
FNDA:0,readme_example::multiply
FNF:3
FNH:2
DA:8,2
DA:12,0
DA:16,4
DA:17,2
DA:18,0
LF:5
LH:3
end_of_record
```

Let's break it down

### Explanation

1. **General Information**
    - **TN**: Test Name (optional, left empty)
    - **SF**: Source File path `/path/to/your/project/src/lib.rs`

2. **Function Details**
    - **FN:8,readme_example::add**: The `add` function starts at line 8.
    - **FN:12,readme_example::multiply**: The `multiply` function starts at line 12.
    - **FN:16,readme_example::calculator**: The `calculator` function starts at line 16.

3. **Function Hit Details**
    - **FNDA:4,readme_example::add**: The `add` function was executed 4 times (Currently not accurate as expected is to
      be 2).
    - **FNDA:0,readme_example::multiply**: The `multiply` function was not executed in the tests.
    - **FNDA:4,readme_example::calculator**: The `calculator` function was executed 4 times (Currently not accurate as
      expected is to be 2).

4. **Function Summary**
    - **FNF:3**: The number of functions found in the source file. There are 3 functions: `add`, `multiply`,
      and `calculator`.
    - **FNH:2**: The number of functions that were hit. 2 out of the 3 functions were
      executed: `add` and `calculator`.

5. **Line Execution Details**
    - **DA:\<line number\>,\<hit count\>**: Indicates whether each line was executed and how many times.

   Here's the details of line coverage:

   | Line | Hits | Explanation                                     |
   |------|------|-------------------------------------------------|
   | 8    | 4    | Line 8 (body of `add`) hit 4 times              |
   | 12   | 0    | Line 12 (body of `multiply`) not hit            |
   | 16   | 2    | Line 16 (start of `match`) hit 2 times          |
   | 17   | 4    | Line 17 (call to `add` in `match`) hit 4 times  |
   | 18   | 0    | Line 18 (call to `multiply` in `match`) not hit |

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
    - 5 lines of code in total.
    - 3 lines were executed during testing.
    - Lines 11 and 12 (`multiply` function) and line 19 (match arm for `Multiply` operation) were not executed.

> 📝 **Note**
>
> This format is for a single file. If there are multiple files, each file's report will be concatenated together
> with `end_of_record` separating them, like this:
> ```lcov
> TN:
> SF:/path/to/your/project/src/operations.cairo
> FN:8,readme_example::add
> FNDA:4,readme_example::add
> FN:12,readme_example::multiply
> FNDA:0,readme_example::multiply
> ... other metrics ...
> end_of_record
> TN:
> SF:/path/to/your/project/src/lib.cairo
> FN:16,readme_example::calculator
> FNDA:4,readme_example::calculator
> ... other metrics ...
> LH:10
> end_of_record
> ```
