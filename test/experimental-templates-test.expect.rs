#[derive(Clone, PartialEq, Eq)]
pub enum Foo {
    Bar,
    Baz,
}

fn if_continue_filter_neg(t: Vec<Foo>) -> () {
    for x in t.iter().filter(|x| true) {()}
    ()
}

fn if_let_ok_map(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t.map_or_else(|| (), |v| { Foo::Bar })
}

fn map_unwrap(t: Vec<Option<Foo>>) -> Result<Vec<Foo>, ()> {
    let v: Vec<&Foo> = t.iter().map(|x| x.as_ref()).collect::<Result<_, _>>()?;
    let u: Vec<Foo> = v.iter().map(|_| Foo::Bar).collect();
    Ok(u)
}

fn runwrap_or_else_backward<E>(t: Result<Foo, E>) -> Foo {
    t.unwrap_or_else(|e| { Foo::Bar })
}

fn runwrap_or_else_forward<E>(t: Result<Foo, E>) -> Foo {
    t.unwrap_or_else(|e| { Foo::Bar })
}

fn unwrap_or_else_backward(t: Option<Foo>) -> Foo {
    t.unwrap_or_else(|| { Foo::Bar })
}

fn unwrap_or_else_forward(t: Option<Foo>) -> Foo {
    t.unwrap_or_else(|| { Foo::Bar })
}

pub fn zero_range(t: &[Foo]) -> &[Foo] {
    let _v = &t[0..t.len()];
    &t[..1]
}

pub fn iter_any_equals_right(t: Vec<Foo>) -> bool {
    t.iter().contains(&Foo::Bar)
}

pub fn iter_any_equals_left(t: Vec<Foo>) -> bool {
    t.iter().contains(&Foo::Bar)
}

pub fn if_then_some_forward() -> Option<Foo> {
    if true {
        Some(Foo::Bar)
    } else {
        None
    }
}

pub fn if_then_some_backward() -> Option<Foo> {
    if true {
        None
    } else {
        Some(Foo::Bar)
    }
}

pub fn let_if_let_else_return(foo: Option<u64>) -> u64 {
    let Some(f) = foo else { return 0u64; }; f
}

// Test cases demonstrating bool::then over-matching issues
pub fn simple_then_case(condition: bool) -> Option<i32> {
    if condition {
        Some(42)
    } else {
        None
    }
}

// BAD: Should NOT be transformed - if-else-if chain demonstrates over-matching
pub fn problematic_if_else_if_chain(x: i32) -> Option<i32> {
    if x > 10 {
        Some(100)
    } else if x > 5 {
        Some(50)
    } else {
        None
    }
}

pub fn complex_if_else_if_chain(condition_a: bool, condition_b: bool, condition_c: bool) -> Option<String> {
    if condition_a {
        Some("first".to_string())
    } else if condition_b {
        Some("second".to_string())
    } else if condition_c {
        Some("third".to_string())
    } else {
        None
    }
}

// Test cases for new patterns

// Additional pattern: unwrap_or_default (Rust 1.82+)
pub fn unwrap_or_default_option(opt: Option<i32>) -> i32 {
    opt.unwrap_or_default()
}

pub fn unwrap_or_default_result(res: Result<i32, String>) -> i32 {
    res.unwrap_or_else(|_| { 0 })
}

// Option::is_none_or patterns
pub fn option_is_none_or_forward(opt: Option<i32>) -> bool {
    opt.is_none_or(|x| x > 10)
}

pub fn option_is_none_or_backward(opt: Option<i32>) -> bool {
    opt.is_none_or(|x| x > 10)
}

// Vec::extract_if patterns
pub fn vec_extract_if_forward(vec: &mut Vec<i32>) -> Vec<i32> {
    // Remove all even numbers and collect them
    vec.retain(|x| *x % 2 != 0);
    let mut removed = Vec::new();
    for x in vec.iter() {
        if *x % 2 == 0 {
            removed.push(*x);
        }
    }
    removed
}

pub fn vec_filter_drain(vec: &mut Vec<i32>) -> Vec<i32> {
    let result: Vec<i32> = vec.iter().filter(|x| *x % 2 == 0).cloned().collect();
    vec.retain(|x| *x % 2 != 0);
    result
}
