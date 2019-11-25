use crate::cache::types::RString;
use crate::cache::db::RDict;

#[test]
fn store_string() {
    let mut dict = RDict::new();

    let key = RString::from("key");
    let val = RString::from("value");

    dict.set_string(key.clone(), val.clone());

    assert_eq!(Some(val.clone()), dict.get_string(&key));
}
