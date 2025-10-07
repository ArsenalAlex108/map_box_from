#![deny(missing_docs)]

//! # map_box_from
//! 
//! Adds [Box]-ed versions of [From] and [Into] traits - allowing implementations for unsized type parameters and following looser guidelines.
//! 
//! # Reasoning
//! 
//! Due to the invasiveness of the identity conversion implementation `impl<T> From<T> for T` and the lack of negative trait implementation, some conversion implementations like `impl<T> From<T> for Unit` or `impl<T> From<T> for NewType<T>` are currently impossible in stable Rust.
//! Compromises will have to made to specify to the compiler that `Self` is not part of `T`, using one of the following methods:
//! - Place an arbitrary bound on `T` and make `Self` not satisfy that bound, i.e. using a widely implemented trait like [Debug] or [Unpin] and unimplement them on `Self`
//! - Define bespoke traits fitting the usecase
//! 
//! This crate chooses the second method and provide [MapBoxFrom] and [MapBoxInto] traits that instead operates on [Box] to allow the use of unsized type parameters. While this crate maps all existing [Into] implementations into [MapBoxFrom] implementations, unsized type parameters are untouched and users are free to add new blanket implementations while avoiding conflict with blanket implemetations in [Sized] land.
//! 
//! **Note:** `TryMapBoxFrom` and `TryMapBoxInto` have not been added due to some considerations about how their blanket implementations should be added.
//! 
//! [Debug]: std::fmt::Debug
//! [MapBoxFrom]: convert::MapBoxFrom
//! [MapBoxInto]: convert::MapBoxInto

/// Traits for defining conversions between [Box] type parameters.
pub mod convert;
