#[derive(Clone, PartialEq, Eq)]
pub enum Foo {
    Bar,
    Baz,
}

fn if_continue_filter_neg(t: Vec<Foo>) -> () {
    for x in t {
        if !true {
            continue;
        }
        ()
    }
    ()
}

fn if_let_ok_map(t: Result<Foo, ()>) -> Result<Foo, ()> {
    if let Ok(v) = t {
        Ok(Foo::Bar)
    } else {
        Err(())
    }
}

fn map_unwrap(t: Vec<Option<Foo>>) -> Result<Vec<Foo>, ()> {
    let v: Vec<&Foo> = t.iter().map(|x| x.as_ref().unwrap()).collect();
    let u: Vec<Foo> = v.iter().map(|_| Foo::Bar).collect();
    Ok(u)
}

fn runwrap_or_else_backward<E>(t: Result<Foo, E>) -> Foo {
    match t {
        Ok(v) => v,
        Err(e) => Foo::Bar,
    }
}

fn runwrap_or_else_forward<E>(t: Result<Foo, E>) -> Foo {
    match t {
        Err(e) => Foo::Bar,
        Ok(v) => v,
    }
}

fn unwrap_or_else_backward(t: Option<Foo>) -> Foo {
    match t {
        Some(v) => v,
        None => Foo::Bar,
    }
}

fn unwrap_or_else_forward(t: Option<Foo>) -> Foo {
    match t {
        None => Foo::Bar,
        Some(v) => v,
    }
}

pub fn zero_range(t: &[Foo]) -> &[Foo] {
    let _v = &t[0..t.len()];
    &t[0..1]
}

pub fn iter_any_equals_right(t: Vec<Foo>) -> bool {
    t.iter().any(|x| x == &Foo::Bar)
}

pub fn iter_any_equals_left(t: Vec<Foo>) -> bool {
    t.iter().any(|x| &Foo::Bar == x)
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
    if let Some(f) = foo {
        f
    } else {
        return 0u64;
    }
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
    match opt {
        Some(x) => x,
        None => 0,
    }
}

pub fn unwrap_or_default_result(res: Result<i32, String>) -> i32 {
    match res {
        Ok(x) => x,
        Err(_) => 0,
    }
}

// Option::is_none_or patterns
pub fn option_is_none_or_forward(opt: Option<i32>) -> bool {
    match opt {
        None => true,
        Some(x) => x > 10,
    }
}

pub fn option_is_none_or_backward(opt: Option<i32>) -> bool {
    match opt {
        Some(x) => x > 10,
        None => true,
    }
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
