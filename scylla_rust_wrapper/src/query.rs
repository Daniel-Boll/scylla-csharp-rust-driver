use std::ffi::{c_char, c_int, c_void, CString};

use scylla::{frame::response::result::ColumnType, QueryResult};

use crate::argconv;

#[repr(C)]
pub struct QueryResultWrapper {
  pub result: QueryResult,
}

#[repr(C)]
pub struct ScyllaRow {
  len: usize,
}

#[no_mangle]
pub unsafe extern "C" fn row_set_get_rows(rs: *mut QueryResult) -> ScyllaRow {
  let rs = argconv::ptr_to_ref_mut(rs);
  let rows = rs.rows.as_ref().unwrap();
  ScyllaRow { len: rows.len() }
}

#[no_mangle]
pub unsafe extern "C" fn row_get_value_int(
  result: *const QueryResult,
  index: usize,
  column: *const c_char,
) -> i32 {
  let result = argconv::ptr_to_ref(result);
  let column = argconv::cstr_to_str(column).expect("column is null");

  let (column_index, column_spec) = result
    .col_specs
    .iter()
    .enumerate()
    .find(|(_i, spec)| spec.name == column)
    .expect("column not found");
  let row = result
    .rows
    .as_ref()
    .expect("rows is null")
    .get(index)
    .expect("index out of bounds");

  match column_spec.typ.clone() {
    ColumnType::Int => row
      .columns
      .as_slice()
      .iter()
      .nth(column_index)
      .expect("index out of bounds")
      .as_ref()
      .expect("column is null")
      .as_int()
      .expect("not an int"),

    ColumnType::BigInt => todo!(),
    ColumnType::SmallInt => todo!(),
    ColumnType::TinyInt => todo!(),
    ColumnType::Varint => todo!(),
    _ => panic!("not an int"),
  }
}

#[no_mangle]
pub unsafe extern "C" fn row_get_value_string(
  result: *const QueryResult,
  index: usize,
  column: *const c_char,
) -> *const c_char {
  let result = argconv::ptr_to_ref(result);
  let column = argconv::cstr_to_str(column).expect("column is null");

  let (column_index, column_spec) = result
    .col_specs
    .iter()
    .enumerate()
    .find(|(_i, spec)| spec.name == column)
    .expect("column not found");
  let row = result
    .rows
    .as_ref()
    .expect("rows is null")
    .get(index)
    .expect("index out of bounds");

  match column_spec.typ.clone() {
    ColumnType::Ascii => Box::into_raw(Box::new(
      row
        .columns
        .as_slice()
        .iter()
        .nth(column_index)
        .expect("index out of bounds")
        .as_ref()
        .expect("column is null")
        .as_ascii()
        .expect("not an ascii or text")
        .to_string(),
    )) as *const c_char,
    ColumnType::Text => CString::new(
      row
        .columns
        .as_slice()
        .iter()
        .nth(column_index)
        .expect("index out of bounds")
        .as_ref()
        .expect("column is null")
        .as_text()
        .expect("not an ascii or text")
        .to_string(),
    )
    .unwrap()
    .into_raw(),
    _ => panic!("not a string"),
  }
}
