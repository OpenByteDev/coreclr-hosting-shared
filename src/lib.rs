#![no_std]
#![warn(clippy::pedantic, clippy::cargo, unsafe_op_in_unsafe_fn)]
#![allow(
    clippy::missing_safety_doc,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::doc_markdown,
    non_camel_case_types, dead_code
)]

//! Shared bindings for the [coreclr hosting components](https://github.com/dotnet/runtime/blob/main/docs/design/features/native-hosting.md).
//!
//! ## Related crates
//! - [nethost-sys](https://crates.io/crates/nethost-sys) - bindings and downloader for the nethost library.
//! - [hostfxr-sys](https://crates.io/crates/hostfxr-sys) - bindings for the hostfxr library.
//! - [netcorehost](https://crates.io/crates/netcorehost) - rusty wrapper over the nethost and hostfxr libraries.
//!
//! ## Additional Information
//! - [Hosting layer APIs](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/hosting-layer-apis.md)
//! - [Native hosting](https://github.com/dotnet/core-setup/blob/master/Documentation/design-docs/native-hosting.md#runtime-properties)
//! - [Write a custom .NET Core host to control the .NET runtime from your native code](https://docs.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting)
//! 
//! ## License
//! Licensed under the MIT license ([LICENSE](https://github.com/OpenByteDev/netcorehost/blob/master/LICENSE) or <https://opensource.org/licenses/MIT>)

/// Module for constants related to the coreclr hosting components or useful for interacting with them.
mod consts;
pub use consts::*;

/// Module for type aliases used in the coreclr hosting components.
mod type_aliases;
pub use type_aliases::*;

/// Module for status codes returned by the coreclr hosting components.
mod status_code;
pub use status_code::*;
