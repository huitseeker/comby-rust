use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
pub enum Foo {
    Bar,
    Baz,
}

pub fn map_trivial_forward(t: Option<Foo>) -> Option<Foo> {
    t
}

pub fn map_trivial_backward(t: Option<Foo>) -> Option<Foo> {
    t
}

pub fn map_forward(t: Option<Foo>) -> Option<Foo> {
    t.map(|f| { Foo::Bar })
}

pub fn map_backward(t: Optionn<Foo>) -> Option<Foo> {
    t.map(|f| { Foo::Bar })
}

fn someify<T>(f: T) -> Option<T> {
    Some(f)
}

fn okify<T>(f: T) -> Result<T, ()> {
    Ok(f)
}

pub fn and_then_forward(t: Option<Foo>) -> Option<Foo> {
    t.and_then(|f| { someify(f) })
}

pub fn and_then_backward(t: Option<Foo>) -> Option<Foo> {
    t.and_then(|f| { someify(f) })
}

pub fn rmap_trivial_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t
}

pub fn rmap_trivial_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t
}

pub fn rmap_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t.map(|f| { Foo::Bar })
}

pub fn rmap_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t.map(|f| { Foo::Bar })
}

pub fn rand_then_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t.and_then(|f| { okify(f) })
}

pub fn rand_then_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    t.and_then(|f| { okify(f) })
}

pub fn map_transpose_forward(t: Option<Foo>) -> Result<Option<Foo>, ()> {
    let opt = t.map(|f| { okify(f) }).transpose()?;
    okify(opt)
}

pub fn map_transpose_backward(t: Option<Foo>) -> Result<Option<Foo>, ()> {
    let opt = t.map(|f| { okify(f) }).transpose()?;
    okify(opt)
}

pub fn basic_interrogation_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    let f = t?;
    okify(f)
}

pub fn basic_interrogation_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    let f = t?;
    okify(f)
}

pub fn contains_let_remove() {
    let mut hm: HashMap<String, String> = HashMap::new();

    let _hr = hm.remove(&"foo".to_string()); if _hr.is_some() { hm.insert("foo".to_string(), "bar".to_string()); }
}

pub fn contains_remove() {
    let mut hm: HashMap<String, String> = HashMap::new();

    if hm.remove(&"foo".to_string()).is_some() { hm.insert("foo".to_string(), "bar".to_string()); }
}

use std::collections::hash_map::Entry;

pub fn entry_match_backward() {
    let mut hm: HashMap<String, String> = HashMap::new();

    hm.entry("foo".to_string()).or_insert_with(|| "bar".to_string());
()
}

pub fn entry_match_forward() {
    let mut hm: HashMap<String, String> = HashMap::new();

    hm.entry("foo".to_string()).or_insert_with(|| "bar".to_string());
()
}

pub fn filter_filter_and(v: Vec<Foo>) -> Vec<Foo> {
    v.iter()
        .filter(|_x| true && false)
        .cloned()
        .collect()
}

pub fn filter_find(v: Vec<Foo>) -> Option<Foo> {
    v.iter()
        //
        .find(|_x| true)
        .cloned()
}

pub fn filter_opt_some_any(t: Option<Foo>) -> bool {
    t
        //
        .any(|_x| true)
}

pub fn filter_opt_none_all(t: Option<Foo>) -> bool {
    t
        //
        .all(|_x| !false)
}

pub fn if_let_ok_map_err<E>(t: Result<Foo, E>) -> Result<Foo, ()> {
    t.map_err(|_| ())
}

pub fn if_let_return_err(t: Option<Result<Foo, ()>>) -> Result<Foo, ()> {
    t.ok_or_else(|| ())?
}

pub fn if_let_some_map(t: Option<Foo>) -> Option<Foo> {
    t.map(|_v| { Foo::Bar })
}

pub fn is_err_backward<E>(t: Result<Foo, E>) -> bool {
    t.is_err()
}

pub fn is_err_forward<E>(t: Result<Foo, E>) -> bool {
    t.is_err()
}

pub fn is_ok_backward<E>(t: Result<Foo, E>) -> bool {
    t.is_ok()
}

pub fn is_ok_forward<E>(t: Result<Foo, E>) -> bool {
    t.is_ok()
}

pub fn is_none_backward(t: Option<Foo>) -> bool {
    t.is_none()
}

pub fn is_none_forward(t: Option<Foo>) -> bool {
    t.is_none()
}

pub fn is_some_backward(t: Option<Foo>) -> bool {
    t.is_some()
}

pub fn is_some_forward(t: Option<Foo>) -> bool {
    t.is_some()
}

pub fn len_range(t: &[Foo]) -> &[Foo] {
    let _v = &t[0..];
    &t[..]
}

pub fn map_flatten_flatmap(t: Vec<Foo>) -> Vec<Foo> {
    t.iter()
        //
        .flat_map(someify)
        .cloned()
        .collect()
}

pub fn ok_or_else_backward(t: Option<Foo>) -> Result<Foo, ()> {
    t.ok_or_else(|| { () })
}

pub fn ok_or_else_forward(t: Option<Foo>) -> Result<Foo, ()> {
    t.ok_or_else(|| { () })
}

pub fn ok_return_none_ok_interrogation_forward<E>(t: Result<Foo, E>) -> Option<Foo> {
    let inner = t.ok()?;
    someify(inner)
}

pub fn ok_return_none_ok_interrogation_backward<E>(t: Result<Foo, E>) -> Option<Foo> {
    let inner = t.ok()?;
    someify(inner)
}

pub fn option_return_none_interrogation_forward(t: Option<Foo>) -> Option<Foo> {
    let inner = t?;
    someify(inner)
}

pub fn option_return_none_interrogation_backward(t: Option<Foo>) -> Option<Foo> {
    let inner = t?;
    someify(inner)
}

pub fn or_else_backward(t: Option<Foo>) -> Option<Foo> {
    t.or_else(|| { Some(Foo::Bar) })
}

pub fn or_else_forward(t: Option<Foo>) -> Option<Foo> {
    t.or_else(|| { Some(Foo::Bar) })
}

pub fn rerr_backward<E>(t: Result<Foo, E>) -> Option<E> {
    t.err()
}

pub fn rerr_forward<E>(t: Result<Foo, E>) -> Option<E> {
    t.err()
}

pub fn rmap_err_interrogation_backward<E>(t: Result<Foo, E>) -> Result<Foo, ()> {
    let inner = t.map_err(|f| { () })?;
    inner
}

pub fn rmap_err_interrogation_forward<E>(t: Result<Foo, E>) -> Result<Foo, ()> {
    let inner = t.map_err(|f| { () })?;
    inner
}

pub fn rok_backward<E>(t: Result<Foo, E>) -> Option<Foo> {
    t.ok()
}

pub fn rok_forward<E>(t: Result<Foo, E>) -> Option<Foo> {
    t.ok()
}

pub fn ror_else_backward<E>(t: Result<Foo, E>) -> Result<Foo, E> {
    t.or_else(|e| { Ok(Foo::Bar) })
}

pub fn ror_else_forward<E>(t: Result<Foo, E>) -> Result<Foo, E> {
    t.or_else(|e| { Ok(Foo::Bar) })
}

pub fn unwrap_err_ok_or_else_interrogation_backward(t: Option<Foo>) -> Result<Foo, ()> {
    let inner = t.ok_or_else(|| ())?;
    okify(inner)
}

pub fn unwrap_err_ok_or_else_interrogation_forward(t: Option<Foo>) -> Result<Foo, ()> {
    let inner = t.ok_or_else(|| ())?;
    okify(inner)
}

pub fn captured_identifiers() {
    let world = "World";
    println!("Hello {}", world);
}

pub fn if_then_forward() -> Option<Foo> {
    if true {
        Some(Foo::Bar)
    } else {
        None
    }
}

pub fn if_then_backward() -> Option<Foo> {
    if true {
        None
    } else {
        Some(Foo::Bar)
    }
}

pub fn let_if_let_else_return(foo: Option<u64>) -> u64 {
    let Some(bar) = foo else { return 0u64; };
    bar
}
