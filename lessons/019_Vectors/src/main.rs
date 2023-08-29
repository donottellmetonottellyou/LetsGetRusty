#![allow(warnings)]

fn main() {
    let mut v = Vec::new();
    v.push("One".to_string());
    v.push("Two".into());
    v.push("Three".into());

    let v2 = vec![1, 2, 3];

    let s = &v[0]; // can panic

    // let s = v.remove(0);
    // /\ moves elements to the left to fill hole

    let s = v.get(0);
    if let Some(e) = s {
        println!("{e}");
    }

    for s in &mut v {
        s.push_str("!");
    }

    for s in &v {
        println!("{s}");
    }

    let mut v3 = vec![];

    for s in v {
        v3.push(s);
    }

    // let e = v.get(0);
    // borrow of moved value
}
