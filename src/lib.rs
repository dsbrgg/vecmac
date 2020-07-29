// declarative macros as basically substitution of code
// inside of the macro declaration, what is valid is anything that is
// grammaticaly (in rust) correct

// The delimeter for macros (eg. [], (), etc) can't be chosen in its implementation
// this is completely up to the user of the macro as all of them as valid
// and have the same meaning in this context

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
    ($($element:expr),+) => {{
        let mut vs = Vec::new();
        // `*` token will execute the expression as many times as
        // the variable $element appear in the pattern created
        $(vs.push($element);)*
        vs
    }}
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
