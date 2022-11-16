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
