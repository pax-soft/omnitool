#[cfg(feature = "earley-omnitool")]
pub extern crate earley_omnitool;

#[cfg(feature = "earley-omnitool")]
pub use earley_omnitool as earley;

#[cfg(feature = "bit-vec-omnitool")]
pub extern crate bit_vec_omnitool;

#[cfg(feature = "bit-vec-omnitool")]
pub use bit_vec_omnitool as bit_vec;

#[cfg(feature = "bit-set-omnitool")]
pub extern crate bit_set_omnitool;

#[cfg(feature = "bit-set-omnitool")]
pub use bit_set_omnitool as bit_set;