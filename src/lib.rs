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
mod util;

pub use res::*;
pub(crate) use lock::*;
pub use jam::*;
pub use jar::*;
pub use stove::*;