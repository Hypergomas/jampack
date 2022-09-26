#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub(crate) use web::*;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use native::*;

mod res;
mod jam;
mod jar;
mod stove;
mod builtin;

pub use res::*;
pub use jam::*;
pub use jar::*;
pub use stove::*;
pub use builtin::*;