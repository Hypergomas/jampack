#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub(crate) use web::*;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::*;

mod res;
mod lock;
mod jam;
mod jar;
mod stove;
mod builtin;
mod util;

pub use res::*;
pub(crate) use lock::*;
pub use jam::*;
pub use jar::*;
pub use stove::*;
pub use builtin::*;

// TODO: Implement SHA-1 "file changed" checks