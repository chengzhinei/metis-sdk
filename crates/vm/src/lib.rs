#[cfg(feature = "compiler")]
pub mod compiler;
#[cfg(feature = "compiler")]
pub use compiler::{CompilerContext, CompilerHandler, ExtCompileWorker, FetchedFnResult};
#[cfg(feature = "compiler")]
pub mod pool;

#[cfg(feature = "inference")]
pub mod inference;
#[cfg(feature = "inference")]
pub use inference::{
    DEFAULT_MODEL_PATH, GAS_PER_INFERENCE_TOKEN, INFERENCE_PRECOMPILE_ADDRESS, InferencePrecompiles,
};

pub mod env;
pub mod error;
mod runtime;

pub mod message;
pub mod actor_interface;

pub use error::Error;

pub const EMPTY_ARR: [(); 0] = [(); 0]; // Based on how it's done in `Tester`.

macro_rules! define_code {
    ($name:ident { code_id: $code_id:literal }) => {
        paste::paste! {
            /// Position of the actor in the builtin actor bundle manifest.
            pub const [<$name _ACTOR_CODE_ID>]: u32 = $code_id;
        }
    };
}

macro_rules! define_id {
    ($name:ident { id: $id:literal }) => {
        paste::paste! {
            pub const [<$name _ACTOR_ID>]: fvm_shared::ActorID = $id;
            pub const [<$name _ACTOR_ADDR>]: fvm_shared::address::Address = fvm_shared::address::Address::new_id([<$name _ACTOR_ID>]);
        }
    };
}

macro_rules! define_singleton {
    ($name:ident { id: $id:literal, code_id: $code_id:literal }) => {
        define_id!($name { id: $id });
        define_code!($name { code_id: $code_id });
    };
}

#[cfg(test)]
mod tests;