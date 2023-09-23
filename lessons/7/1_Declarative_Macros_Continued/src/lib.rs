/// Declarative Macro:
/// macro_rules! name {
///     rule0;
///     rule1;
///     ...
///     ruleN;
/// }
///
/// Macro Rule:
/// (pattern) => {
///     rust syntax...
/// }
#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello, world!");
    };
}

#[macro_export]
macro_rules! hash_map {
    ($key:ty, $val:ty) => {{
        use std::collections::HashMap;
        let map: HashMap<$key, $val> = HashMap::new();
        map
    }};
    ($($key:expr => $val:expr),*) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(map.insert($key, $val);)*
        map
    }};
}
