//! A registry of external functions that can be called in Inko source code.
use crate::object_pointer::ObjectPointer;
use crate::process::RcProcess;
use crate::runtime_error::RuntimeError;
use crate::vm::state::RcState;
use ahash::AHashMap;

/// Defines a setup() function that registers all the given external functions.
macro_rules! register {
    ($($name:ident),*) => {
        pub fn setup(
            functions: &mut crate::external_functions::ExternalFunctions
        ) -> Result<(), String> {
            $(
                functions.add(stringify!($name), $name)?;
            )*
            Ok(())
        }
    }
}

mod blocks;
mod env;
mod ffi;
mod fs;
mod hasher;
mod modules;
mod process;
mod random;
mod socket;
mod stdio;
mod time;

/// A external function that can be called from Inko source code.
pub type ExternalFunction = fn(
    &RcState,
    &RcProcess,
    &[ObjectPointer],
) -> Result<ObjectPointer, RuntimeError>;

/// A collection of external functions.
pub struct ExternalFunctions {
    mapping: AHashMap<String, ExternalFunction>,
}

impl ExternalFunctions {
    /// Creates a collection of external functions and registers all functions
    /// that Inko ships with.
    pub fn setup() -> Result<Self, String> {
        let mut instance = Self::new();

        random::setup(&mut instance)?;
        fs::setup(&mut instance)?;
        stdio::setup(&mut instance)?;
        env::setup(&mut instance)?;
        time::setup(&mut instance)?;
        hasher::setup(&mut instance)?;
        blocks::setup(&mut instance)?;
        ffi::setup(&mut instance)?;
        modules::setup(&mut instance)?;
        socket::setup(&mut instance)?;
        process::setup(&mut instance)?;

        Ok(instance)
    }

    /// Creates a new empty collection of external functions.
    pub fn new() -> Self {
        Self {
            mapping: AHashMap::default(),
        }
    }

    /// Adds a new external function with the given name.
    pub fn add<I: Into<String>>(
        &mut self,
        name: I,
        function: ExternalFunction,
    ) -> Result<(), String> {
        let name: String = name.into();

        if self.mapping.contains_key(&name) {
            return Err(format!(
                "The external function {} is already defined",
                name
            ));
        }

        self.mapping.insert(name, function);
        Ok(())
    }

    /// Looks up a external function by its name.
    pub fn get(&self, name: &str) -> Result<ExternalFunction, String> {
        self.mapping.get(name).cloned().ok_or_else(|| {
            format!("The external function {} is undefined", name)
        })
    }
}