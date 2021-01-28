use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
pub enum Foo {
    Bar,
    Baz,
}

pub fn map_trivial_forward(t: Option<Foo>) -> Option<Foo> {
    match t {
        None => None,
        Some(f) => Some(f),
    }
}

pub fn map_trivial_backward(t: Option<Foo>) -> Option<Foo> {
    match t {
        Some(f) => Some(f),
        None => None,
    }
}

pub fn map_forward(t: Option<Foo>) -> Option<Foo> {
    match t {
        None => None,
        Some(f) => Some(Foo::Bar),
    }
}

pub fn map_backward(t: Optionn<Foo>) -> Option<Foo> {
    match t {
        Some(f) => Some(Foo::Bar),
        None => None,
    }
}

fn someify<T>(f: T) -> Option<T> {
    Some(f)
}

fn okify<T>(f: T) -> Result<T, ()> {
    Ok(f)
}

pub fn and_then_forward(t: Option<Foo>) -> Option<Foo> {
    match t {
        None => None,
        Some(f) => someify(f),
    }
}

pub fn and_then_backward(t: Option<Foo>) -> Option<Foo> {
    match t {
        None => None,
        Some(f) => someify(f),
    }
}

pub fn rmap_trivial_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    match t {
        Err(e) => Err(e),
        Ok(f) => Ok(f),
    }
}

pub fn rmap_trivial_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    match t {
        Ok(f) => Ok(f),
        Err(e) => Err(e),
    }
}

pub fn rmap_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    match t {
        Err(e) => Err(e),
        Ok(f) => Ok(Foo::Bar),
    }
}

pub fn rmap_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    match t {
        Ok(f) => Ok(Foo::Bar),
        Err(e) => Err(e),
    }
}

pub fn rand_then_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    match t {
        Err(e) => Err(e),
        Ok(f) => okify(f),
    }
}

pub fn rand_then_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    match t {
        Ok(f) => okify(f),
        Err(e) => Err(e),
    }
}

pub fn map_transpose_forward(t: Option<Foo>) -> Result<Option<Foo>, ()> {
    let opt = match t {
        Some(f) => Some(okify(f)?),
        None => None,
    };
    okify(opt)
}

pub fn map_transpose_backward(t: Option<Foo>) -> Result<Option<Foo>, ()> {
    let opt = match t {
        None => None,
        Some(f) => Some(okify(f)?),
    };
    okify(opt)
}

pub fn basic_interrogation_forward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    let f = match t {
        Err(e) => return Err(e),
        Ok(x) => x,
    };
    okify(f)
}

pub fn basic_interrogation_backward(t: Result<Foo, ()>) -> Result<Foo, ()> {
    let f = match t {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    okify(f)
}

pub fn contains_let_remove() {
    let mut hm: HashMap<String, String> = HashMap::new();

    if hm.contains_key(&"foo".to_string()) {
        let _hr = hm.remove(&"foo".to_string());
        hm.insert("foo".to_string(), "bar".to_string());
    }
}

pub fn contains_remove() {
    let mut hm: HashMap<String, String> = HashMap::new();

    if hm.contains_key(&"foo".to_string()) {
        hm.remove(&"foo".to_string());
        hm.insert("foo".to_string(), "bar".to_string());
    }
}

use std::collections::hash_map::Entry;

pub fn entry_match_backward() {
    let mut hm: HashMap<String, String> = HashMap::new();

    match hm.entry("foo".to_string()) {
        Entry::Vacant(e) => e.insert("bar".to_string()),
        Entry::Occupied(v) => v.into_mut(),
    };
    ()
}

pub fn entry_match_forward() {
    let mut hm: HashMap<String, String> = HashMap::new();

    match hm.entry("foo".to_string()) {
        Entry::Occupied(v) => v.into_mut(),
        Entry::Vacant(e) => e.insert("bar".to_string()),
    };
    ()
}

pub fn filter_filter_and(v: Vec<Foo>) -> Vec<Foo> {
    v.iter()
        .filter(|_x| true)
        .filter(|_x| false)
        .cloned()
        .collect()
}

pub fn filter_find(v: Vec<Foo>) -> Option<Foo> {
    v.iter()
        //
        .filter(|_x| true)
        .nth(0)
        .cloned()
}

pub fn filter_opt_some_any(t: Option<Foo>) -> bool {
    t
        //
        .filter(|_x| true)
        .is_some()
}

pub fn filter_opt_none_all(t: Option<Foo>) -> bool {
    t
        //
        .filter(|_x| false)
        .is_none()
}

pub fn if_let_ok_map_err<E>(t: Result<Foo, E>) -> Result<Foo, ()> {
    if let Ok(f) = t {
        Ok(f)
    } else {
        Err(())
    }
}

pub fn if_let_return_err(t: Option<Result<Foo, ()>>) -> Result<Foo, ()> {
    if let Some(f) = t {
        f
    } else {
        return Err(());
    }
}

pub fn if_let_some_map(t: Option<Foo>) -> Option<Foo> {
    if let Some(_v) = t {
        Some(Foo::Bar)
    } else {
        None
    }
}

pub fn is_err_backward<E>(t: Result<Foo, E>) -> bool {
    match t {
        Err(_) => true,
        Ok(_) => false,
    }
}

pub fn is_err_forward<E>(t: Result<Foo, E>) -> bool {
    match t {
        Ok(_) => false,
        Err(_) => true,
    }
}

pub fn is_ok_backward<E>(t: Result<Foo, E>) -> bool {
    match t {
        Err(_) => false,
        Ok(_) => true,
    }
}

pub fn is_ok_forward<E>(t: Result<Foo, E>) -> bool {
    match t {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_none_backward(t: Option<Foo>) -> bool {
    match t {
        None => true,
        Some(_) => false,
    }
}

pub fn is_none_forward(t: Option<Foo>) -> bool {
    match t {
        Some(_) => false,
        None => true,
    }
}

pub fn is_some_backward(t: Option<Foo>) -> bool {
    match t {
        None => false,
        Some(_) => true,
    }
}

pub fn is_some_forward(t: Option<Foo>) -> bool {
    match t {
        Some(_) => true,
        None => false,
    }
}

pub fn len_range(t: &[Foo]) -> &[Foo] {
    let _v = &t[0..t.len()];
    &t[..t.len()]
}

pub fn map_flatten_flatmap(t: Vec<Foo>) -> Vec<Foo> {
    t.iter()
        //
        .map(someify)
        .flatten()
        .cloned()
        .collect()
}

pub fn ok_or_else_backward(t: Option<Foo>) -> Result<Foo, ()> {
    match t {
        Some(f) => Ok(f),
        None => Err(()),
    }
}

pub fn ok_or_else_forward(t: Option<Foo>) -> Result<Foo, ()> {
    match t {
        Some(f) => Ok(f),
        None => Err(()),
    }
}

pub fn ok_return_none_ok_interrogation_forward<E>(t: Result<Foo, E>) -> Option<Foo> {
    let inner = match t {
        Ok(f) => f,
        Err(e) => return None,
    };
    someify(inner)
}

pub fn ok_return_none_ok_interrogation_backward<E>(t: Result<Foo, E>) -> Option<Foo> {
    let inner = match t {
        Err(e) => return None,
        Ok(f) => f,
    };
    someify(inner)
}

pub fn option_return_none_interrogation_forward(t: Option<Foo>) -> Option<Foo> {
    let inner = match t {
        Some(f) => f,
        None => return None,
    };
    someify(inner)
}

pub fn option_return_none_interrogation_backward(t: Option<Foo>) -> Option<Foo> {
    let inner = match t {
        Some(f) => f,
        None => return None,
    };
    someify(inner)
}

pub fn or_else_backward(t: Option<Foo>) -> Option<Foo> {
    match t {
        None => Some(Foo::Bar),
        Some(f) => Some(f),
    }
}

pub fn or_else_forward(t: Option<Foo>) -> Option<Foo> {
    match t {
        Some(f) => Some(f),
        None => Some(Foo::Bar),
    }
}

pub fn rerr_backward<E>(t: Result<Foo, E>) -> Option<E> {
    match t {
        Ok(f) => None,
        Err(e) => Some(e),
    }
}

pub fn rerr_forward<E>(t: Result<Foo, E>) -> Option<E> {
    match t {
        Err(e) => Some(e),
        Ok(f) => None,
    }
}

pub fn rmap_err_interrogation_backward<E>(t: Result<Foo, E>) -> Result<Foo, ()> {
    let inner = match t {
        Err(f) => return Err(()),
        Ok(x) => Ok(x),
    };
    inner
}

pub fn rmap_err_interrogation_forward<E>(t: Result<Foo, E>) -> Result<Foo, ()> {
    let inner = match t {
        Ok(x) => Ok(x),
        Err(f) => return Err(()),
    };
    inner
}

pub fn rok_backward<E>(t: Result<Foo, E>) -> Option<Foo> {
    match t {
        Ok(f) => Some(f),
        Err(e) => None,
    }
}

pub fn rok_forward<E>(t: Result<Foo, E>) -> Option<Foo> {
    match t {
        Err(e) => None,
        Ok(f) => Some(f),
    }
}

pub fn ror_else_backward<E>(t: Result<Foo, E>) -> Result<Foo, E> {
    match t {
        Ok(f) => Ok(f),
        Err(e) => Ok(Foo::Bar),
    }
}

pub fn ror_else_forward<E>(t: Result<Foo, E>) -> Result<Foo, E> {
    match t {
        Err(e) => Ok(Foo::Bar),
        Ok(f) => Ok(f),
    }
}

pub fn unwrap_err_ok_or_else_interrogation_backward(t: Option<Foo>) -> Result<Foo, ()> {
    let inner = match t {
        Some(f) => f,
        None => return Err(()),
    };
    okify(inner)
}

pub fn unwrap_err_ok_or_else_interrogation_forward(t: Option<Foo>) -> Result<Foo, ()> {
    let inner = match t {
        None => return Err(()),
        Some(f) => f,
    };
    okify(inner)
}
