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
