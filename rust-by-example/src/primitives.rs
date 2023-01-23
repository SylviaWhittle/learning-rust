fn main() {
    // Variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32; // suffix annotation

    // if no type specified, a default will be used
    let default_float = 3.0; // f64
    let default_int = 7; // i32

    // types can also be inferred from context
    let mut inferred_type = 12; // type i64 is inferred from another line
    inferred_type = 3242342342342i64; // here we set the type

    // a mutable variable's value can be changed
    let mut mutable = 12; // mutable i32
    mutable = 21;

    // error - the type of the variable can't be changed
    mutable = true;

    // variables can be overwritten with shadowing
    let mutable = true;
}
