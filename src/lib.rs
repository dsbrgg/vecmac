// declarative macros as basically substitution of code
// inside of the macro declaration, what is valid is anything that is
// grammaticaly (in rust) correct

// The delimeter for macros (eg. [], (), etc) can't be chosen in its implementation
// this is completely up to the user of the macro as all of them as valid
// and have the same meaning in this context

// Why use it?
// Declarative macros are good to express repetition
// eg. implement a trait for a type that the logic is the same everytime
// 44:16 at the reference video
// #[derive] logic is only for PROC MACROS, no declarative macros

// macro_rules also does not have TRAIT BOUNDS

#[macro_export]
macro_rules! avec {
    // empty_vec ------------
    () => {
        Vec::new()
    };

    // single ---------------
    // reason for double curly brackets:
    // if using only one curly brackets, the expression will tell rust to
    // expand the macro into these three statements below and by using double
    // we make sure that the rule will execute it as a block, which will ultimaly 
    // become an expression 
    //
    // `expr` is a token for macros that maps to any rust expression
    ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};


    // double --------------
    // $() syntax will expand to a pattern
    // `,+` means: one or more inputs, separated by comma
    // `$(,)?` means that it will accept a trailling comma, where `?` means zero or one
    // declarative macros accept only `?`, `+` and `*` for patterns, so its not
    // an actual regex engine
    ($($element:expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        // `*` token will execute the expression as many times as
        // the variable $element appear in the pattern created
        $(vs.push($element);)*
        vs
    }};

    // clone ---------------------
    // its always good to evaluate an expression just once
    // eg. let count = $count;
    ($element:expr; $count:expr) => {{
        let count = $count;
        let mut vs = Vec::with_capacity(count);
        // this is a good way to allocate to a vec when the size is known
        // .extend will make rust know that the vector will have elements added to it
        // and preallocate the correct size for it instead of reallocating the pointer on each
        // iteration
        // std::iter::repeat will yield clones of the element given as long as you .take them
        vs.extend(std::iter::repeat($element).take(count));
        vs
    }};
}

#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trail() {
    let x: Vec<&'static str> = avec!["ola", "ola", "ola",];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], "ola");
    assert_eq!(x[1], "ola");
    assert_eq!(x[2], "ola");
}

#[test]
fn clone() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
