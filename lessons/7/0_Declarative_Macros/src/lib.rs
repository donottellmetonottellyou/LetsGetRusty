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
