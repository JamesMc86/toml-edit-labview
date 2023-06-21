use libc::c_char;
use std::{
    // error::Error,
    ffi::{
        CStr,
        CString,
        c_void,
    },
    // fs,
    ptr,
    str::FromStr
};
use toml_edit::{
    Document,
    InlineTable,
    Item,
    Table,
    Value,
};

// dll exported function to return a pointer to a Document, which can be used in other .dll functions
// takes a TOML string as an input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_from_string (
    toml_str: *const c_char,
) -> *mut c_void {
    let toml_str = unsafe { CStr::from_ptr(toml_str).to_string_lossy().into_owned() };

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

// dll exported function to return a toml string from a Document
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_to_string (
    doc: *mut c_void,
) -> *mut c_char {
    let doc = unsafe { &mut *(doc as *mut Document) };

    let toml_str = match Document::to_string(doc) {
        toml_str => toml_str,
    };

    let raw_string = match CString::new(toml_str).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}



// dll exported function to return a pointer to the root Table of a Document
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_get_root_table (
    doc: *mut c_void,
) -> *mut c_void {
    let doc = unsafe { &mut *(doc as *mut Document) };

    let table = match doc.as_table() {
        table => table
    };

    let table = Box::new(table.clone());

    Box::into_raw(table) as *mut c_void

}

// dll exported function to convert from a Table to a toml string
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_to_string (
    table: *mut c_void,
) -> *mut c_char {
    let table = unsafe { &mut *(table as *mut Table) };

    let toml_str = match Table::to_string(table) {
        toml_str => toml_str,
    };

    let raw_string = match CString::new(toml_str).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}

// dll exported function to convert a Table to an Item
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_to_item (
    table: *mut c_void,
) -> *mut c_void {
    let table = unsafe { &mut *(table as *mut Table) };

    let item = match Item::Table(table.clone()) {
        item => item,
    };

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// dll exported function to convert a InlineTable to an Item
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_to_item (
    inline_table: *mut c_void,
) -> *mut c_void {
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let item = match  toml_edit::value(inline_table.clone()) {
        item => item,
    };

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}


// dll exported function to list the tables in a Document as a multi-line string
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_list_tables (
    doc: *mut c_void,
) -> *mut c_char {
    let doc = unsafe { &mut *(doc as *mut Document) };

    let mut table_list = String::new();

    for table in doc.as_table() {
        table_list.push_str(&format!("{}\n", table.0));
    }

    let raw_string = match CString::new(table_list).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}


// dll exported function to return a pointer to a Table, which can be used in other .dll functions
// takes a Document and a table name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_get_table (
    doc: *mut c_void,
    table_name: *const c_char,
) -> *mut c_void {
    let doc = unsafe { &mut *(doc as *mut Document) };
    let table_name = unsafe { CStr::from_ptr(table_name).to_string_lossy().into_owned() };

    let table = match doc[table_name.as_str()].as_table() {
        Some(table) => table,
        None => {
            println!("Unable to find table: {}", table_name);
            return ptr::null_mut();
        }
    };

    let table = Box::new(table.clone());

    Box::into_raw(table) as *mut c_void
}

// dll exported function to set an item in the root table of a Document
// takes a Document, a key, and a Item as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_doc_set_item (
    doc: *mut c_void,
    key: *const c_char,
    item: *mut c_void,
) -> *mut c_void {
    let doc = unsafe { &mut *(doc as *mut Document) };
    let key = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };
    let item = unsafe { &mut *(item as *mut Item) };

    doc[key.as_str()] = item.clone();

    doc as *mut Document as *mut c_void

}


#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_list_items (
    table: *mut c_void,
) -> *mut c_char {
    let table = unsafe { &mut *(table as *mut Table) };

    let mut item_list = String::new();

    for item in table.iter() {
        item_list.push_str(&format!("{}\n", item.0));
    }

    let raw_string = match CString::new(item_list).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}

// dll exported function to remove an item from a Table
// takes a Table and a item name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_remove_item (
    table: *mut c_void,
    key: *const c_char,
) -> u64 {
    let table = unsafe { &mut *(table as *mut Table) };
    let key = unsafe { CStr::from_ptr(key).to_string_lossy().into_owned() };

    return if table.contains_key(key.as_str()) {
        table.remove(key.as_str());
        1
    } else {
        0
    }
}

// dll exported function to remove an item from a InlineTable
// takes a InlineTable and a item name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_remove_item (
    inline_table: *mut c_void,
    item_name: *const c_char,
) -> u64 {
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };
    let item_name = unsafe { CStr::from_ptr(item_name).to_string_lossy().into_owned() };

    return if inline_table.contains_key(item_name.as_str()) {
        inline_table.remove(item_name.as_str());
        1
    } else {
        0
    }
}


// dll exported function to return a pointer to a Item, which can be used in other .dll functions
// takes a Table and a item name as inputs
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_get_item (
    table: *mut c_void,
    item_name: *const c_char,
) -> *mut c_void {
    let table = unsafe { &mut *(table as *mut Table) };
    let item_name = unsafe { CStr::from_ptr(item_name).to_string_lossy().into_owned() };

    let item = match table[item_name.as_str()].clone() {
        item => item,
        // _ => {
        //     println!("Unable to find item: {}", item_name);
        //     return ptr::null_mut();
        // }
    };

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// dll exported function to get the type of a Item
// takes a Item as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_get_item_type (
    item: *mut c_void,
) -> *mut c_char {
    let item = unsafe { &mut *(item as *mut Item) };

    let item_type = match item {
        Item::None => "None",
        Item::Value(_) => "Value",
        Item::ArrayOfTables(_) => "ArrayOfTables",
        Item::Table(_) => "Table",
    };

    let raw_string = match CString::new(item_type).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}


// dll exported function to get the type of a value
// takes a value as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_get_value_type (
    value: *mut c_void,
) -> *mut c_char {
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

    let raw_string = match CString::new(value_type).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}

// dll exported function to get a value from a Item
// takes a Item as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_into_value (
    item: *mut c_void,
) -> *mut c_void {
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


// dll exported function to get a Table from a Item
// takes a Item as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_item_into_table (
    item: *mut c_void,
) -> *mut c_void {
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


// dll exported function to get a String typed Value from a value
// takes a value as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_get_value_string (
    value: *mut c_void,
) -> *mut c_char {
    let value = unsafe { &mut *(value as *mut Value) };

    let value = match value {
        Value::String(value) => value,
        _ => {
            println!("Value is not a String");
            return CString::new("").unwrap().into_raw();
        }
    };

    let return_value = value.clone().into_value();

    let raw_string = match CString::new(return_value).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}

// dll exported function to get a i64 typed Value from a value
// takes a value as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_get_value_i64 (
    value: *mut c_void,
) -> i64 {
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

// dll exported function to get an InlineTable typed Value from a value
// takes a value as input and returns a raw pointer to a Table
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_get_value_inline_table (
    value: *mut c_void,
) -> *mut c_void {
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


// dll exported function to create a new Value::String from a string
// takes a *const c_char as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_new_value_string (
    string: *const c_char,
) -> *mut c_void {
    let string = unsafe { CStr::from_ptr(string).to_str().unwrap() };

    let item = toml_edit::value(string);

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// dll exported function to create a new Value::Integer from a i64
// takes a i64 as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_new_value_i64 (
    integer: i64,
) -> *mut c_void {
    let item = toml_edit::value(integer);

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// dll exported function to create a new, empty Value::InlineTable
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_new_value_inline_table (
) -> *mut c_void {
    let item = toml_edit::value(InlineTable::default());

    let item = Box::new(item);

    Box::into_raw(item) as *mut c_void
}

// dll exported function to create a new, empty Table
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_new_table (
) -> *mut c_void {
    let table = Table::default();

    let table = Box::new(table);

    Box::into_raw(table) as *mut c_void
}


// dll exported function to check if an item exists in a table
// takes a *const c_char as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_contains_item (
    table: *mut c_void,
    key: *const c_char,
) -> i64 {
    let table = unsafe { &mut *(table as *mut Table) };
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };

    return if table.contains_key(key) { 1 } else { 0 };

}

// dll exported function to check if an item exists in an inline table
// takes a *const c_char as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_contains_item (
    table: *mut c_void,
    key: *const c_char,
) -> i64 {
    let table = unsafe { &mut *(table as *mut InlineTable) };
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };

    return if table.contains_key(key) { 1 } else { 0 };

}

// dll exported function to set a Item in a Table
// takes a *const c_char as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_table_set_item (
    table: *mut c_void,
    key: *const c_char,
    item: *mut c_void,
) {
    let table = unsafe { &mut *(table as *mut Table) };
    let key = unsafe { CStr::from_ptr(key).to_str().unwrap() };
    let item = unsafe { &mut *(item as *mut Item) };

    table.insert(key, item.clone());
}

// dll exported to return a multi-line string of the keynames in an InlineTable
// takes a InlineTable as input
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_list_items (
    inline_table: *mut c_void,
) -> *mut c_char {
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let mut return_string = String::new();

    for (key, _) in inline_table.iter() {
        return_string.push_str(key);
        return_string.push_str("\n");
    }

    let raw_string = match CString::new(return_string).unwrap().into_raw() {
        ptr if ptr.is_null() => {
            println!("Unable to allocate memory for string");
            return CString::new("").unwrap().into_raw();
        },
        ptr => ptr,
    };

    return raw_string;
}

// dll exported function to get an value from a InlineTable
// takes a InlineTable as input and a *const c_char as the keyname
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_get_item (
    inline_table: *mut c_void,
    key: *const c_char,
) -> *mut c_void {
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let key = match unsafe { CStr::from_ptr(key).to_str() } {
        Ok(key) => key,
        Err(_) => {
            println!("Unable to convert key to string");
            return ptr::null_mut();
        }
    };

    let (_, item) = match inline_table.get_key_value(key) {
        Some(key_value_pair) => key_value_pair,
        None => {
            println!("Key not found");
            return ptr::null_mut();
        }
    };

    let item = Box::new(item.clone());

    Box::into_raw(item) as *mut c_void

}

// dll exported function to set an value to a InlineTable
// takes a InlineTable as input and a *const c_char as the keyname
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_inline_table_set_item (
    inline_table: *mut c_void,
    key: *const c_char,
    item: *mut c_void,
) {
    let inline_table = unsafe { &mut *(inline_table as *mut InlineTable) };

    let key = match unsafe { CStr::from_ptr(key).to_str() } {
        Ok(key) => key,
        Err(_) => {
            println!("Unable to convert key to string");
            return;
        }
    };

    let item = unsafe { &mut *(item as *mut Item) };

    // verify that the item is a Item::Value
    match item {
        Item::Value(_) => {},
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
    inline_table.insert(key, value.clone());

}


// dll exported function to close the Document and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_close (
    doc: *mut c_void,
) {
    let doc = unsafe { Box::from_raw(doc as *mut Document) };
    drop(doc);
}

// dll exported function to close the Table and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_close_table (
    table: *mut c_void,
) {
    let table = unsafe { Box::from_raw(table as *mut Table) };
    drop(table);
}

// dll exported function to close the Item and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_close_item (
    item: *mut c_void,
) {
    let item = unsafe { Box::from_raw(item as *mut Item) };
    drop(item);
}

// dll exported function to close the Value and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_close_value (
    value: *mut c_void,
) {
    let value = unsafe { Box::from_raw(value as *mut Value) };
    drop(value);
}

// dll exported function to close the InlineTable and free the memory
#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn toml_edit_close_inline_table (
    table: *mut c_void,
) {
    let table = unsafe { Box::from_raw(table as *mut InlineTable) };
    drop(table);
}


// a DLL function that frees the memory allocated for a string
#[no_mangle]
pub extern "C" fn memory_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
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
