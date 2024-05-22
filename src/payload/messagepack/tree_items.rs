use rmpv::Value;
use tui_tree_widget::{Selector, TreeItem};

use super::map_key;

pub fn tree_items(root: &Value) -> Vec<TreeItem<'_, Selector>> {
    match root {
        Value::Map(object) => from_map(object),
        Value::Array(array) => from_array(array),
        _ => vec![TreeItem::new_leaf(Selector::None, root.to_string())],
    }
}

fn recurse(key: Selector, value: &Value) -> TreeItem<Selector> {
    match value {
        Value::Map(object) => {
            let text = key.to_string();
            TreeItem::new(key, text, from_map(object)).unwrap()
        }
        Value::Array(array) => {
            let text = key.to_string();
            TreeItem::new(key, text, from_array(array)).unwrap()
        }
        _ => {
            let text = format!("{key}: {value}");
            TreeItem::new_leaf(key, text)
        }
    }
}

fn from_map(object: &[(Value, Value)]) -> Vec<TreeItem<'_, Selector>> {
    object
        .iter()
        .map(|(key, value)| recurse(Selector::ObjectKey(map_key(key)), value))
        .collect()
}

fn from_array(array: &[Value]) -> Vec<TreeItem<'_, Selector>> {
    array
        .iter()
        .enumerate()
        .map(|(index, value)| recurse(Selector::ArrayIndex(index), value))
        .collect()
}

#[test]
fn value_to_string_is_same_as_variant_to_string() {
    let int: rmpv::Integer = 42.into();
    assert_eq!(int.to_string(), Value::Integer(int).to_string());

    let float = 13.37;
    assert_eq!(float.to_string(), Value::F64(float).to_string());
}
