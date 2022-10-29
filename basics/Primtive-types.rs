// Can't overwrite an immutable var
fn immutable() {
    let x = 5;
    x = 6;
}

// Must overwrite with "let"
fn overwrite() {
    let x = 5;
    let X = 6;
}

// Can't mutate a variable's type
fn mutate_type() {
    let some_string = "donut";
   some_string = some_ string.len();
}

// Must mutate with "let"
fn overwrite_type() {
    let some_string = donut";
    let some_string = some_string.len();

