use labview_interop::types::LStrHandle;

use std::{
    ffi::{c_void, CString},
    ptr,
    str::FromStr,
};
use toml_edit::{table, Document, InlineTable, Item, Table, Value};

// return any TOML parse error as a string using toml_edit::TomlError
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_get_error(toml_str: LStrHandle, mut error_str: LStrHandle) {
    let toml_str = toml_str.to_rust_string();

    // try to parse the TOML string
    return match Document::from_str(&toml_str) {
        Ok(_) => {
            // return an empty string if no error
            error_str.set_str("").unwrap();
        }
        Err(error) => {
            // return the error string (if any)
            error_str.set_str(&error.to_string()).unwrap();
        }
    };
}

// return a pointer to a Document, which can be used in other .dll functions
// takes a TOML string as an input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_from_string(toml_str: LStrHandle) -> *mut c_void {
    let toml_str = unsafe { toml_str.as_ref().unwrap().to_rust_string() };

    let doc = match Document::from_str(&toml_str) {
        Ok(doc) => doc,
        Err(_) => {
            println!("Unable to parse TOML string: {}", toml_str);
            return ptr::null_mut();
        }
    };

    let doc: Box<Document> = Box::new(doc);

    Box::into_raw(doc) as *mut c_void
}

// return a toml string from a Document
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_to_string(doc: *mut c_void, mut toml_string: LStrHandle) {
    // todo: add better error return
    if doc.is_null() {
        println!("Document pointer is null");
        toml_string.set_str("").expect("String should be valid.");
    }
    let doc = unsafe { &mut *(doc as *mut Document) };

    let toml_str = Document::to_string(doc);

    toml_string
        .set_str(&toml_str)
        .expect("String should be valid.");
}

// return a pointer to the root Table of a Document
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_get_root_table(doc: *mut c_void) -> *mut c_void {
    if doc.is_null() {
        println!("Document pointer is null");
        return ptr::null_mut();
    }
    let doc = unsafe { &mut *(doc as *mut Document) };

    let table = match doc.as_table() {
        table => table,
    };

    let table = Box::new(table.clone());

    Box::into_raw(table) as *mut c_void
}

// Close a Document and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_close(doc: *mut c_void) {
    if doc.is_null() {
        println!("Document pointer is null");
        return;
    }
    let doc = unsafe { Box::from_raw(doc as *mut Document) };
    drop(doc);
}

// convert from a Table to a toml string
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_to_string(table: *mut c_void, mut toml_string: LStrHandle) {
    if table.is_null() {
        println!("Table pointer is null");
        toml_string.set_str("").expect("String should be valid.");
    }
    let table = unsafe { &mut *(table as *mut Table) };

    let toml_str = Table::to_string(table);
    toml_string
        .set_str(&toml_str)
        .expect("String should be valid.");
}

// convert a Table to an Item
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_to_item(table: *mut c_void) -> *mut c_void {
    if table.is_null() {
        println!("Table pointer is null");
        return ptr::null_mut();
    }
    let table = unsafe { &mut *(table as *mut Table) };

    let item = match Item::Table(table.clone()) {
        item => item,
    };

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// convert a InlineTable to an Item
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_to_item(inline_table: *mut c_void) -> *mut c_void {
    if inline_table.is_null() {
        println!("InlineTable pointer is null");
        return ptr::null_mut();
    }

    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let item = match toml_edit::value(inline_table.clone()) {
        item => item,
    };

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// list the tables in a Document as a multi-line string
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_list_tables(doc: *mut c_void, mut tables: LStrHandle) {
    if doc.is_null() {
        println!("Document pointer is null");
        let _ = tables.set_str("");
        return;
    }

    let doc = unsafe { &mut *(doc as *mut Document) };

    let mut table_list = String::new();

    for table in doc.as_table() {
        table_list.push_str(&format!("{}\n", table.0));
    }

    let _ = tables.set_str(&table_list);
}

// return a pointer to a Table, which can be used in other .dll functions
// takes a Document and a table name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_get_table(doc: *mut c_void, table_name: LStrHandle) -> *mut c_void {
    if doc.is_null() {
        println!("Document pointer is null");
        return ptr::null_mut();
    }
    let doc = unsafe { &mut *(doc as *mut Document) };
    let table_name = table_name.to_rust_string();

    let table = match doc[&table_name].as_table() {
        Some(table) => table,
        None => {
            println!("Unable to find table: {}", table_name);
            return ptr::null_mut();
        }
    };

    let table = Box::new(table.clone());

    Box::into_raw(table) as *mut c_void
}

// set an item in the root table of a Document
// takes a Document, a key, and a Item as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_set_item(
    doc: *mut c_void,
    key: LStrHandle,
    item: *mut c_void,
) -> *mut c_void {
    if doc.is_null() {
        println!("Document is null");
        return ptr::null_mut();
    }
    if item.is_null() {
        println!("Item is null");
        return ptr::null_mut();
    }

    let doc = unsafe { &mut *(doc as *mut Document) };
    let key = key.to_rust_string();
    let item = unsafe { &mut *(item as *mut Item) };

    doc[&key] = item.clone();

    doc as *mut Document as *mut c_void
}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_list_items(table: *mut c_void, mut item_list_output: LStrHandle) {
    if table.is_null() {
        println!("Table is null");
        let _ = item_list_output.set_str("");
        return;
    }

    let table = unsafe { &mut *(table as *mut Table) };

    let mut item_list = String::new();

    for item in table.iter() {
        item_list.push_str(&format!("{}\n", item.0));
    }

    let _ = item_list_output.set_str(&item_list);
}

// remove an item from a Table
// takes a Table and a item name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_remove_item(table: *mut c_void, key: LStrHandle) -> u64 {
    // todo: return -1 on error? need better error return.
    if table.is_null() {
        println!("Table is null");
        return 0;
    }
    let table = unsafe { &mut *(table as *mut Table) };
    let key = key.to_rust_string();

    return if table.contains_key(&key) {
        table.remove(&key);
        1
    } else {
        0
    };
}

// Close a Table and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_close(table: *mut c_void) {
    if table.is_null() {
        println!("Table is null");
        return;
    }
    let table = unsafe { Box::from_raw(table as *mut Table) };
    drop(table);
}

// return a pointer to a Item, which can be used in other .dll functions
// takes a Table and a item name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_get_item(
    table: *mut c_void,
    item_name: LStrHandle,
) -> *mut c_void {
    if table.is_null() {
        println!("Table is null");
        return ptr::null_mut();
    }

    let table = unsafe { &mut *(table as *mut Table) };
    let item_name = item_name.to_rust_string();

    let item = match table[&item_name].clone() {
        item => item,
    };

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// set a Item in a Table
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_set_item(table: *mut c_void, key: LStrHandle, item: *mut c_void) {
    if table.is_null() {
        println!("Table is null");
        return;
    }
    if item.is_null() {
        println!("Item is null");
        return;
    }

    let table = unsafe { &mut *(table as *mut Table) };
    let key = key.to_rust_string();
    let item = unsafe { &mut *(item as *mut Item) };

    table.insert(&key, item.clone());
}

// get the type of a value
// takes a value as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_get_value_type(value: *mut c_void, mut type_name: LStrHandle) {
    if value.is_null() {
        println!("Value is null");
        return type_name.set_str("").expect("Valid Handle");
    }

    let value = unsafe { &mut *(value as *mut Value) };

    let value_type = match value {
        Value::String(_) => "String",
        Value::Integer(_) => "Integer",
        Value::Float(_) => "Float",
        Value::Boolean(_) => "Boolean",
        Value::Datetime(_) => "Datetime",
        Value::Array(_) => "Array",
        Value::InlineTable(_) => "InlineTable",
    };

    type_name.set_str(value_type).expect("Valid Handle");
}

// get the type of a Item
// takes a Item as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_get_type(item: *mut c_void, mut type_name: LStrHandle) {
    // check if item has a null value
    if item.is_null() {
        return type_name.set_str("None").expect("Handle is Valid");
    }

    let item = unsafe { &mut *(item as *mut Item) };

    let item_type = match item {
        Item::None => "None",
        Item::Value(_) => "Value",
        Item::ArrayOfTables(_) => "ArrayOfTables",
        Item::Table(_) => "Table",
    };

    type_name.set_str(item_type).expect("Valid String Handle");
}

// get a value from a Item
// takes a Item as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_into_value(item: *mut c_void) -> *mut c_void {
    if item.is_null() {
        println!("Item is null");
        return ptr::null_mut();
    }

    let item = unsafe { &mut *(item as *mut Item) };

    let value = match item {
        Item::Value(value) => value,
        _ => {
            println!("Item is not a Value");
            return ptr::null_mut();
        }
    };

    let value = Box::new(value.clone());

    Box::into_raw(value) as *mut c_void
}

// get a Table from a Item
// takes a Item as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_into_table(item: *mut c_void) -> *mut c_void {
    if item.is_null() {
        println!("Item is null");
        return ptr::null_mut();
    }

    let item = unsafe { &mut *(item as *mut Item) };

    let table = match item {
        Item::Table(table) => table,
        _ => {
            println!("Item is not a Table");
            return ptr::null_mut();
        }
    };

    let table = Box::new(table.clone());

    Box::into_raw(table) as *mut c_void
}

// get a String typed Value from a value
// takes a value as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_value_get_string(value: *mut c_void, mut value_string: LStrHandle) {
    // need a better error return
    if value.is_null() {
        println!("Value is null");
        return;
    }

    let value = unsafe { &mut *(value as *mut Value) };

    let value = match value {
        Value::String(value) => value,
        _ => {
            println!("Value is not a String");
            return;
        }
    };
    value_string
        .set_str(value.value())
        .expect("String can be written");
}

// get a i64 typed Value from a value
// takes a value as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_value_get_i64(value: *mut c_void) -> i64 {
    // we really need a better way to return an error code. e.g. 0 and -1 are valid values for i64
    if value.is_null() {
        println!("Value is null");
        return 0;
    }

    let value = unsafe { &mut *(value as *mut Value) };

    let value = match value {
        Value::Integer(value) => value,
        _ => {
            println!("Value is not a Integer");
            return 0;
        }
    };

    let return_value = value.clone().into_value();

    return return_value;
}

// get a boolean typed Value from a value
// takes a value as input
// returns a i8 with 1 representing true, 0 representing false, and -1 representing an error
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_value_get_bool(value: *mut c_void) -> i8 {
    if value.is_null() {
        println!("Value is null");
        return -1;
    }

    let value = unsafe { &mut *(value as *mut Value) };

    let value = match value {
        Value::Boolean(value) => value,
        _ => {
            println!("Value is not a Boolean");
            return -1;
        }
    };

    let return_value = value.clone().into_value();

    return return_value as i8;
}

// get an InlineTable typed Value from a value
// takes a value as input and returns a raw pointer to a Table
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_value_get_inline_table(value: *mut c_void) -> *mut c_void {
    if value.is_null() {
        println!("Value is null");
        return ptr::null_mut();
    }

    let value = unsafe { &mut *(value as *mut Value) };

    let value = match value {
        Value::InlineTable(value) => value,
        _ => {
            println!("Value is not a InlineTable");
            return ptr::null_mut();
        }
    };

    let value = Box::new(value.clone());

    Box::into_raw(value) as *mut c_void
}

// create a new Value::String from a string
// takes a LStrHandle as input.
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_new_value_from_string(string: LStrHandle) -> *mut c_void {
    let string = string.to_rust_string();
    let item = toml_edit::value(string.as_ref());

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// create a new Value::Integer from a i64
// takes a i64 as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_new_value_from_i64(integer: i64) -> *mut c_void {
    let item = toml_edit::value(integer);

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// create a new Value::Boolean from a i8
// takes a i8 as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_new_value_from_bool(boolean: i8) -> *mut c_void {
    let boolean = match boolean {
        1 => true,
        0 => false,
        _ => {
            println!("Invalid boolean value");
            return ptr::null_mut();
        }
    };

    let item = toml_edit::value(boolean);

    let item = Box::new(item);

    return Box::into_raw(item) as *mut c_void;
}

// create a new, empty Value::InlineTable
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_new_value_inline_table() -> *mut c_void {
    let item = toml_edit::value(InlineTable::default());

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// create a new, empty Table
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_new() -> *mut c_void {
    let table = Table::default();

    let table = Box::new(table);

    Box::into_raw(table) as *mut c_void
}

// check if an item exists in a table
// takes a *const c_char as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_contains_item(table: *mut c_void, key: LStrHandle) -> i64 {
    if table.is_null() {
        println!("Table is null");
        return -1;
    }
    let table = unsafe { &mut *(table as *mut Table) };
    let key = key.to_rust_string();

    return if table.contains_key(&key) { 1 } else { 0 };
}

// create a new, empty InlineTable
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_new() -> *mut c_void {
    let t = InlineTable::default();

    let t = Box::new(t);

    return Box::into_raw(t) as *mut c_void;
}

// remove an item from a InlineTable
// takes a InlineTable and a item name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_remove_item(
    inline_table: *mut c_void,
    item_name: LStrHandle,
) -> u64 {
    if inline_table.is_null() {
        println!("InlineTable is null");
        return 0;
    }
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };
    let item_name = item_name.to_rust_string();

    return if inline_table.contains_key(&item_name) {
        inline_table.remove(&item_name);
        1
    } else {
        0
    };
}

// check if an item exists in an inline table
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_contains_item(table: *mut c_void, key: LStrHandle) -> i64 {
    if table.is_null() {
        println!("InlineTable is null");
        return -1;
    }

    let table = unsafe { &mut *(table as *mut InlineTable) };
    let key = key.to_rust_string();

    return if table.contains_key(&key) { 1 } else { 0 };
}

// Return a multi-line string of the keynames in an InlineTable
// takes a InlineTable as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_list_items(
    inline_table: *mut c_void,
    mut item_list: LStrHandle,
) {
    if inline_table.is_null() {
        println!("InlineTable is null");
        return;
    }

    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let mut return_string = String::new();

    for (key, _) in inline_table.iter() {
        return_string.push_str(key);
        return_string.push_str("\n");
    }

    let _ = item_list.set_str(&return_string);
}

// Get an value from a InlineTable
// takes a InlineTable as input and a LStrHandle as the keyname
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_get_item(
    inline_table: *mut c_void,
    key: LStrHandle,
) -> *mut c_void {
    if inline_table.is_null() {
        println!("InlineTable is null");
        // review: a null pointer would make more sense?
        // return an empty string
        return CString::new("").unwrap().into_raw() as *mut c_void;
    }
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let key = key.to_rust_string();

    let (_, item) = match inline_table.get_key_value(&key) {
        Some(key_value_pair) => key_value_pair,
        None => {
            println!("Key not found");
            return ptr::null_mut();
        }
    };

    let item = Box::new(item.clone());

    Box::into_raw(item) as *mut c_void
}

// Set an value in an InlineTable
// takes a InlineTable as input and a LStr as the keyname
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_set_item(
    inline_table: *mut c_void,
    key: LStrHandle,
    item: *mut c_void,
) {
    if inline_table.is_null() {
        println!("InlineTable is null");
        return;
    }
    if item.is_null() {
        println!("Item is null");
        return;
    }
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let key = key.to_rust_string();

    let item = unsafe { &mut *(item as *mut Item) };

    // verify that the item is a Item::Value
    match item {
        Item::Value(_) => {}
        _ => {
            println!("Item is not a Item::Value");
            return;
        }
    }

    // convert it to a value
    let value = match item.as_value() {
        Some(value) => value,
        None => {
            println!("Unable to convert item to Value");
            return;
        }
    };

    // insert the value into the inline table
    // They key becomes owned internally so convert to owned first for type compatability.
    inline_table.insert(key.into_owned(), value.clone());
}

// Close an Item and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_close(item: *mut c_void) {
    if item.is_null() {
        return;
    }
    let item = unsafe { Box::from_raw(item as *mut Item) };
    drop(item);
}

// Close a Value and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_value_close(value: *mut c_void) {
    if value.is_null() {
        return;
    }
    let value = unsafe { Box::from_raw(value as *mut Value) };
    drop(value);
}

// Close an InlineTable and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_close(table: *mut c_void) {
    if table.is_null() {
        return;
    }
    let table = unsafe { Box::from_raw(table as *mut InlineTable) };
    drop(table);
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // function to remove leading whitespace from each line in a string.
    fn remove_indentation(s: &str) -> String {
        let mut result = String::new();
        for line in s.lines() {
            let trimmed_line = line.trim_start();
            result.push_str(trimmed_line);
            result.push_str("\n");
        }

        // remove leading and trailing newlines
        result = result.trim().to_string();

        return result;
    }

    // function to assert that two strings are equal, ignoring indentation and leading/trailing newlines.
    fn assert_equal_ignore_indentation(s1: &str, s2: &str) {
        let s1 = remove_indentation(s1);
        let s2 = remove_indentation(s2);
        assert_eq!(s1, s2);
    }
}
