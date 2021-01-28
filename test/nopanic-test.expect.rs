#[derive(Clone, PartialEq, Eq)]
pub enum Foo {
    Bar,
    Baz,
}

fn foo(t: Option<Foo>) -> Result<Foo, NoneError> {
    let out = t?;
    Ok(out)
}
fn foo_exp(t: Option<Foo>) -> Result<Foo, NoneError> {
    let out = t?;
    Ok(out)
}
fn bar(t: Result<Foo, NoneError>) -> Result<Foo, NoneError> {
    let out = t?;
    Ok(out)
}
fn bar_exp(t: Result<Foo, NoneError>) -> Result<Foo, NoneError> {
    let out = t?;
    Ok(out)
}
