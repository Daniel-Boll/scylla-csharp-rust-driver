use lazy_static::lazy_static;
use tokio::runtime::Runtime;

#[allow(clippy::missing_safety_doc)]
pub mod argconv;
#[allow(clippy::missing_safety_doc)]
pub mod cluster_builder;
#[allow(clippy::missing_safety_doc)]
pub mod query;
#[allow(clippy::missing_safety_doc)]
pub mod session;
#[allow(non_camel_case_types)]
pub mod types;
#[allow(clippy::missing_safety_doc)]
pub mod utils;

lazy_static! {
  pub static ref RUNTIME: Runtime = Runtime::new().unwrap();
}
