fn main() {
    let_variable_bindings();
    let_variable_bindings_2();
    type_annotation();
    type_annotation_2();
    type_inference();
    variable_shadowing();
    tuples();
    tuples_annotated();
    tuple_destructuring();
    semi_colon_statements();
    block_scoping();
    blocks_are_expressions();
    multiple_statement_blocks();
    expressions();
    structures();
    let_patterns_if_expressions();
    match_arms_are_patterns();
    mutable_variable_bindings();
    traits_are_sharable_interfaces();
    trait_markers();
    generics();
    turbofish();
    structs_can_be_generic();
    macros();
    panics();
    enums();
    references_have_lifetimes();
}

fn let_variable_bindings() {
    let x; // declare "x"
    x = 42; // assign 42 to "x"
    Some(x);
}

fn let_variable_bindings_2() {
    let x = 42;
    Some(x);
}

fn type_annotation() {
    let x: i32;
    x = 42;
    Some(x);
}

fn type_annotation_2() {
    // in a single line
    let x: i32 = 42;
    Some(x);
}

fn type_inference() {
    let x;
    x = 42;
    Some(x);
}

fn variable_shadowing() {
    // separate bindings with the same name can be introduced
    let x = 13;
    let x = x + 3;
    Some(x);
}

fn tuples() {
    // think of them as "fixed-length" collection of values of differing types
    let pair = ('a', 17);
    pair.0; // this is 'a'
    pair.1; // this is 17
}

fn tuples_annotated() {
    let pair: (char, i32) = ('a', 17);
    Some(pair);
}

fn tuple_destructuring() {
    let pair: (char, i32) = ('a', 17);
    let (some_char, some_int) = pair;
    Some((some_char, some_int));

    // USEFUL WHEN FUNCTION RETURNS A TUPLE
    // let (left, right) = slice.split_at(middle);
}

fn semi_colon_statements() {
    let _foo = "bar";
    let _multi_line = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
}

fn block_scoping() {
    let x = "out";
    {
        let x = "inside";
        println!("from the {}", x);
    }
    println!("from the {}", x);
}

fn blocks_are_expressions() {
    // this
    let _x = 42;
    // is the same as this
    let _x2 = { 42 };
}

fn multiple_statement_blocks() {
    let _x = {
        let y = 1; // first statement
        let z = 2; // second statement
        y + z
    };
}

fn expressions() {
    // explicit return statement
    #[allow(dead_code)]
    fn fair_dice_roll() -> i32 {
        return 5;
    }

    // expression (sans semi-colon) is an implicit return
    #[allow(dead_code)]
    fn fair_dice_roll_imp() -> i32 {
        5
    }

    // conditionals
    #[allow(dead_code)]
    fn fair_dice_roll_conditional() -> i32 {
        let feeling_lucky = true;

        if feeling_lucky {
            7
        } else {
            4
        }
    }

    // match is also an exprssion
    let feeling_lucky = false;
    let _is_lucky = match feeling_lucky {
        true => 8,
        false => 4,
    };
}

fn structures() {
    struct Person {
        name: String,
        location: String,
    }

    let p1 = Person {
        name: String::from("Bob"),
        location: String::from("SF"),
    };
    let _p2 = Person {
        name: String::from("Jane"),
        location: String::from("Portland"),
    };
    let p3 = Person { ..p1 }; // dot-dot for merging non-overlapping fields

    // destructuring
    let Person { name, .. } = p3;
    println!("name: {}", name);
    let Person { location, .. } = p3;
    println!("location: {}", location);
}

fn let_patterns_if_expressions() {
    struct Number {
        odd: bool,
        value: i32,
    }

    fn main() {
        let one = Number {
            odd: true,
            value: 1,
        };
        let two = Number {
            odd: false,
            value: 2,
        };
        print_number(one);
        print_number(two);
    }

    fn print_number(n: Number) {
        if let Number { odd: true, value } = n {
            println!("Odd number: {}", value);
        } else if let Number { odd: false, value } = n {
            println!("Even number: {}", value);
        }
    }

    main();
}

fn match_arms_are_patterns() {
    struct Number {
        odd: bool,
        value: i32,
    }

    fn print_number(n: Number) {
        match n {
            Number { odd: true, value } => println!("Odd number: {}", value),
            Number { odd: false, value } => println!("Even number: {}", value),
        }
    }

    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        odd: false,
        value: 2,
    };
    print_number(one);
    print_number(two);
}

fn mutable_variable_bindings() {
    struct Number {
        odd: bool,
        value: i32,
    }

    let mut n = Number {
        odd: true,
        value: 17,
    };
    n.value = 18;
    n.odd = false;
}

fn traits_are_sharable_interfaces() {
    // 1. we can implement our trat on anyone's type
    // 2. we can implement someone else's trait on our types
    // 3. we cannot implement a foreign trait on a foreign type
    trait Signed {
        fn is_strictly_negative(self) -> bool;
    }

    #[allow(dead_code)]
    struct Number {
        odd: bool,
        value: i32,
    }

    // implemented on our own type
    impl Signed for Number {
        fn is_strictly_negative(self) -> bool {
            self.value < 0
        }
    }

    let num = Number {
        odd: true,
        value: -55,
    };
    print!("the number {} | ", num.value);
    println!("strictly negative? {}", num.is_strictly_negative());

    // implemented on a foreign type
    impl Signed for i32 {
        fn is_strictly_negative(self) -> bool {
            self < 0
        }
    }

    let num2 = -88;
    print!("the number {} | ", num2);
    println!("strictly negative? {}", num2.is_strictly_negative());

    // foreign trait on our type
    // the `Neg` trait is used to overload `-`, the unary minus operator
    impl std::ops::Neg for Number {
        type Output = Self; // `Self` means the type `Number` here

        fn neg(self) -> Self {
            Number {
                value: -self.value,
                odd: self.odd,
            }
        }
    }

    let m = Number {
        odd: true,
        value: 987,
    };
    let n = -m; // this is only possible because we implemented the `Neg` trait
    println!("{}", n.value); // -987
}

fn trait_markers() {
    // some traits are *markers* -- they don't say that a type implements some methods, they say
    // that certain operations can be performed with that type
    // i32 implements the Copy trait and the following works
    let a: i32 = 15;
    let _b = a; // `a` has been copied
    let _c = a; // `a` has been copied again

    // the following also works
    fn print_i32(x: i32) {
        println!("x = {}", x);
    }

    let d: i32 = 15;
    print_i32(d); // `a` is copied
    print_i32(d); // `a` is copied again

    struct Number {
        odd: bool,
        value: i32,
    }

    fn print_number(x: Number) {
        println!("{} number {}", if x.odd { "odd" } else { "even" }, x.value);
    }

    let x = Number {
        odd: true,
        value: 51,
    };
    print_number(x); // `n` is moved (owned now by print_number)
                     // print_number(x); // this doesn't work because the value has been moved

    fn print_number_ref(x: &Number) {
        println!("{} number {}", if x.odd { "odd" } else { "even" }, x.value);
    }
    let y = Number {
        odd: false,
        value: 52,
    };
    print_number_ref(&y); // `y` is borrowed for the time of the call
    print_number_ref(&y); // `y` is borrowed again

    // works if our function takes a mutable reference and our variable binding is also `mut`
    fn invert(n: &mut Number) {
        n.value = -n.value
    }

    fn print_num(n: &Number) {
        println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
    }

    let mut z = Number {
        odd: true,
        value: 51,
    };
    print_num(&z);
    invert(&mut z); // `n` is borrowed mutably - everything is explicit
    print_num(&z);

    // trait methods can also take `self` by reference or mutable reference
    impl std::clone::Clone for Number {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }

    // when invoking trait methods, the receiver is borrowed implicitly
    let n = Number {
        odd: true,
        value: 51,
    };
    let mut m = n.clone();
    m.value += 100;
    print_num(&n);
    print_num(&m);

    // the following are equivalent
    let _e = n.clone();
    let _f = std::clone::Clone::clone(&n);

    // `Copy` requires that `Clone` is implemented, too
    impl std::marker::Copy for Number {}
    // `Clone` can still be used and `Number` values will no longer be moved (but copied)
    let g = n.clone();
    let _h = g; // `h` is a copy of g

    // NOTE: some traits are so commons that they can be implemented automatically using the
    // `derive` attribute
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    struct Number2 {
        odd: bool,
        value: i32,
    }
}

fn generics() {
    use std::fmt::{Debug, Display};

    #[allow(dead_code)]
    fn foobar<T>(_arg: T) {
        // do something with `arg`
    }

    // they can have multiple type parameters
    #[allow(dead_code)]
    fn foobar2<T, U>(_left: T, _right: U) {
        // do something with `left` and `right`
    }

    // type parameters usually have "constraints" aka trait bounds (boundaries/limits)
    #[allow(dead_code)]
    fn print<T: Display + Debug>(value: T) {
        println!("value = {}", value);
    }

    // if there are many trait bounds there is a better syntax
    #[allow(dead_code)]
    fn print2<T>(value: T)
    where
        T: Display + Debug,
    {
        println!("value = {}", value);
    }

    // trait bound (constraints) can be complicated as they can require a type parameter to
    // implement multiple traits
    fn compare<T>(left: T, right: T)
    where
        T: Debug + PartialEq,
    {
        println!(
            "{:?} {} {:?}",
            left,
            if left == right { "==" } else { "!=" },
            right
        );
    }

    compare("tea", "coffee");
}

fn turbofish() {
    // generic functions can be thought of as "namespaces" containing an infinity of functions with
    // different concrete types
    // crates, modules, types, generic functions can all be "explored" using `::` (turbofish)
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
}

fn structs_can_be_generic() {
    #[allow(dead_code)]
    struct Pair<T> {
        a: T,
        b: T,
    }

    fn print_type_name<T>(_val: &T) {
        println!("{}", std::any::type_name::<T>());
    }

    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // prints "Pair<i32>"
    print_type_name(&p2); // prints "Pair<bool>"

    // heap-allocated array
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"

    // macro (for "vec literals")
    let v3 = vec![1, 2, 3];
    let v4 = vec![true, false, true];
    print_type_name(&v3); // prints "Vec<i32>"
    print_type_name(&v4); // prints "Vec<bool>"
}

fn macros() {
    // this is...
    println!("{}", "Hello there!");
    // roughly the same as...
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello there!\n").unwrap();

    // panic is a "violent stop execution"
    // stops execution spitting out the line number in the source code
    // panic!("This panics!");
}

fn panics() {
    // some methods also panic
    // option type can contain someting or nothing
    // if .unwrap() is invoked on it and it doesn't contain something
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is good

    let _o2: Option<i32> = None;
    //o2.unwrap(); // this panics
}

fn enums() {
    println!("foobar");
    enum OptionX<T> {
        None,
        Some(T),
    }

    impl<T> OptionX<T> {
        fn unwrap(self) -> T {
            // enum variants may be used in patterns
            match self {
                Self::Some(t) => t,
                Self::None => panic!(".unwrap() called on None option"),
            }
        }
    }

    use OptionX::{None, Some};

    let o1: OptionX<i32> = Some(128);
    o1.unwrap();

    let _o2: OptionX<i32> = None;
    // o2.unwrap(); // this panics

    // `Result` is also an enum that contains someting or error
    #[allow(dead_code)]
    enum ResultX<T, E> {
        Ok(T),
        Err(E),
    }
}

fn references_have_lifetimes() {
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        let x_ref = &x; // `x_ref` starts existing - it borrows `x`
        println!("x_ref = {}", x_ref);
        // `x_ref` stops existing
        // `x` stops existing
    } // dropped
      // `x` no longer exists

    // NOTE: the lifetime of a reference cannot exceed the lifetime of the variable binding that it
    // borrows
    //let x_ref = {
    //let x = 42;
    //&x // "borrowed value does not live long enough"
    //};
    //println!("x_ref = {}", x_ref);
    // error: `x` does not live long enough

    // NOTE: a variable binding can be immutably borrowed multiple times
    let x = 42;
    let x_ref1 = &x;
    let x_ref2 = &x;
    let x_ref3 = &x;
    println!("ref1: {} | ref2: {} | ref3: {}", x_ref1, x_ref2, x_ref3);

    // NOTE: while borrowed, a variable binding cannot be mutated
    #[allow(unused_mut)]
    let mut x = 42;
    let x_ref = &x;
    // x = 13; // cannot do this
    println!("x_ref = {}", x_ref);
    // error: cannot assign to `x` because it is borrowed

    // NOTE: while immutably borrowed, a variable cannot be mutably borrowed
    #[allow(unused_mut)]
    let mut y = 42;
    let y_ref1 = &y;
    // let y_ref2 = &mut y; // cannot do this
    // error: cannot borrow `y` as mutable because it is also borrowed as immutable
    println!("y_ref1 = {}", y_ref1);

    // NOTE: references in function arguments also have lifetimes
    #[allow(dead_code)]
    fn print(_x: &i32) {
        // `x` is borrowed (from the outside) for the entire time this function is called
    }

    // NOTE: functions with reference arguments can be called with borrows of different lifetimes
    // - all functions that take references are generic
    // - lifetimes are generic parameters

    // elided (non-named) lifetimes
    #[allow(dead_code)]
    fn print1(_x: &i32) {}

    // named lifetimes
    #[allow(dead_code)]
    fn print2<'a>(_x: &'a i32) {}

    // NOTE: this permits the return of references whose lifetime depends on the lifetime of the
    // arguments
    struct Number {
        value: i32,
    }

    fn number_value<'a>(num: &'a Number) -> &'a i32 {
        &num.value
    }

    let n = Number { value: 47 };
    let _v = number_value(&n);
    // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`
    // // while `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.

    // NOTE: structs can also be generic over lifetimes allowing them to hold references
    #[allow(dead_code)]
    struct NumRef<'a> {
        x: &'a i32,
    }

    let z: i32 = 99;
    let _z_ref = NumRef { x: &z };
    // `z_ref` cannot outlive `z`, etc.
}
