use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        let v = match self.values.entry(arg) {
            Entry::Occupied(o) => *o.into_mut(),
            Entry::Vacant(_) => (self.calculation)(arg),
        };
        v
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
