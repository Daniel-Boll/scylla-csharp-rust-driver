use std::{ffi::c_char, sync::Arc};

use scylla::SessionBuilder;

use crate::{argconv, session::SessionWrapper, RUNTIME};

#[repr(C)]
pub struct ClusterBuilder {
  pub builder: SessionBuilder,
}

#[no_mangle]
pub extern "C" fn cluster_builder_new() -> *mut ClusterBuilder {
  Box::into_raw(Box::new(ClusterBuilder {
    builder: SessionBuilder::new(),
  }))
}

#[no_mangle]
pub unsafe extern "C" fn cluster_builder_set_contact_points(
  builder: *mut ClusterBuilder,
  contact_points: *const c_char,
) {
  let builder = argconv::ptr_to_ref_mut(builder);
  let contact_points = argconv::cstr_to_str(contact_points).expect("contact_points is null");

  builder.builder = builder.builder.clone().known_node(contact_points);
}

// NOTE: This is somewhat useless, for now I don't know what C# driver can do to Cluster to make it useful
#[no_mangle]
pub unsafe extern "C" fn cluster_builder_build() {}

#[no_mangle]
pub unsafe extern "C" fn cluster_connect(
  cluster: *mut ClusterBuilder,
  keyspace: *const c_char,
) -> *mut SessionWrapper {
  let cluster = argconv::ptr_to_ref_mut(cluster);
  let keyspace = argconv::cstr_to_str(keyspace).expect("keyspace is null");

  cluster.builder = cluster.builder.clone().use_keyspace(keyspace, false);

  let session = RUNTIME.block_on(cluster.builder.build()).unwrap();

  Box::into_raw(Box::new(SessionWrapper {
    session: Arc::new(session),
  }))
}
