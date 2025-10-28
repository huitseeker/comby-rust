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
fn direct_return_unwrap(opt: Option<Foo>) -> Result<Foo, NoneError> {
    opt
}

fn direct_return_expect(res: Result<Foo, String>) -> Result<Foo, String> {
    res
}

// 2. Function argument patterns
fn function_arg_unwrap(opt: Option<i32>) -> Result<String, NoneError> {
    Ok(format!("Value: {}", opt.unwrap()))
}

fn function_arg_expect(opt: Option<i32>) -> Result<String, NoneError> {
    Ok(format!("Value: {}", opt.expect("Expected value")))
}

// 3. Complex expression patterns
fn complex_expression_unwrap(a: Option<i32>, b: Option<i32>) -> Result<i32, NoneError> {
    let sum = a? + b?;
    Ok(sum)
}

fn complex_expression_expect(a: Option<i32>, b: Option<i32>) -> Result<i32, NoneError> {
    let sum = a? + b?;
    Ok(sum)
}

// 4. Match expression patterns
fn match_unwrap(opt: Option<i32>) -> Result<String, NoneError> {
    let result = match opt {
        Some(val) => format!("Number: {}", val.unwrap()),
        None => String::new(),
    };
    Ok(result)
}

fn match_expect(opt: Option<i32>) -> Result<String, NoneError> {
    let result = match opt {
        Some(val) => format!("Number: {}", val.expect("Should have value")),
        None => String::new(),
    };
    Ok(result)
}

// 5. Conditional expression patterns
fn conditional_unwrap(condition: bool, opt: Option<i32>) -> Result<i32, NoneError> {
    let value = if let Some(temp) = opt { temp } else { 42 };
    Ok(value)
}
fn conditional_expect(condition: bool, opt: Option<i32>) -> Result<i32, NoneError> {
    let value = if let Some(temp) = opt { temp } else { 42 };
    Ok(value)
}
fn closure_unwrap(data: Vec<Option<i32>>) -> Result<Vec<i32>, NoneError> {
    let results: Vec<i32> = data.into_iter().map(|opt| opt.unwrap()).collect();
    Ok(results)
}

fn closure_expect(data: Vec<Option<i32>>) -> Result<Vec<i32>, NoneError> {
    let results: Vec<i32> = data.into_iter().map(|opt| opt.expect("Expected value")).collect();
    Ok(results)
}

// 7. Mixed patterns
fn mixed_patterns(opt1: Option<Foo>, opt2: Option<i32>) -> Result<String, NoneError> {
    match opt1 {
        Some(foo) => Ok(format!("{:?}", foo)),
        None => Ok(format!("Default: {}", opt2.unwrap())),
    }
}
