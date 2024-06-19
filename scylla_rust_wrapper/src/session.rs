use std::{ffi::c_char, sync::Arc};

use scylla::Session;

use crate::{argconv, query::QueryResultWrapper, RUNTIME};

#[repr(C)]
pub struct SessionWrapper {
  pub session: Arc<Session>,
}

#[no_mangle]
pub unsafe extern "C" fn session_execute(
  session: *mut SessionWrapper,
  query: *const c_char,
) -> *mut QueryResultWrapper {
  let cluster = argconv::ptr_to_ref(session);
  let query = argconv::cstr_to_str(query)
    .expect("query is null")
    .to_string();

  let result = RUNTIME.block_on(cluster.session.query(query, &[])).unwrap();
  Box::into_raw(Box::new(QueryResultWrapper { result }))
}
