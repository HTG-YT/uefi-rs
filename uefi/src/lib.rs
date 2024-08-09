//! Rusty wrapper for the [Unified Extensible Firmware Interface][UEFI].
//!
//! This crate makes it easy to develop Rust software that leverages **safe**,
//! **convenient**, and **performant** abstractions for [UEFI] functionality.
//!
//! See the [Rust UEFI Book] for a tutorial, how-tos, and overviews of some
//! important UEFI concepts. For more details of UEFI, see the latest [UEFI
//! Specification][spec].
//!
//! # Value-add and Use Cases
//!
//! `uefi` supports writing code for both pre- and post-exit boot services
//! epochs, but its true strength shines when you create UEFI images that heavily
//! interact with UEFI boot services. Still, you have the flexibility to just
//! integrate selected types and abstractions into your project, for example to
//! parse the UEFI memory map.
//!
//! _Note that for producing UEFI images, you also need to use a corresponding
//! `uefi` compiler target of Rust, such as `x86_64-unknown-uefi`._
//!
//! ## Example Use Cases
//!
//! This library significantly simplifies the process of creating **UEFI images**
//! by abstracting away much of the UEFI API complexity and by providing
//! convenient wrappers. When we mention UEFI images, we are talking about UEFI
//! applications, UEFI boot service drivers, and EFI runtime service drivers,
//! which typically have the `.efi` file extension. For instance, an UEFI
//! application could be an OS-specific loader, similar to _GRUB_ or _Limine_.
//!
//! Additionally, you can use this crate in non-UEFI images (such as a kernel
//! in ELF format) to perform tasks like parsing the UEFI memory map embedded in
//! the boot information provided by a bootloader. It also enables access to
//! UEFI runtime services from a non-UEFI image kernel.
//!
//! # Supported Architectures
//!
//! `uefi` is compatible with all platforms that both the Rust compiler and
//! UEFI support, such as `i686`, `x86_64`, and `aarch64`. Please note that we
//! can't test all possible hardware/firmware/platform combinations in CI.
//!
//! # Crate organisation
//!
//! The top-level module contains some of the most used types and macros,
//! including the [`Handle`] and [`Result`] types, the [`CStr16`] and
//! [`CString16`] types for working with UCS-2 strings, and the [`entry`] and
//! [`guid`] macros.
//!
//! ## Tables
//!
//! The [`SystemTable`] provides access to almost everything in UEFI. It comes
//! in two flavors:
//! - `SystemTable<Boot>`: for boot-time applications such as bootloaders,
//!   provides access to both boot and runtime services.
//! - `SystemTable<Runtime>`: for operating systems after boot services have
//!   been exited.
//!
//! ## Protocols
//!
//! When boot services are active, most functionality is provided via UEFI
//! protocols. Protocols provide operations such as reading and writing files,
//! drawing to the screen, sending and receiving network requests, and much
//! more. The list of protocols that are actually available when running an
//! application depends on the device. For example, a PC with no network card
//! may not provide network protocols.
//!
//! See the [`BootServices`] documentation for details of how to open a
//! protocol, and see the [`proto`] module for protocol implementations. New
//! protocols can be defined with the [`unsafe_protocol`] macro.
//!
//! ## Optional Cargo crate features
//!
//! - `alloc`: Enable functionality requiring the [`alloc`] crate from
//!   the Rust standard library. For example, methods that return a
//!   `Vec` rather than filling a statically-sized array. This requires
//!   a global allocator; you can use the `global_allocator` feature or
//!   provide your own. This is independent of internal direct usages of the
//!   UEFI boot service allocator which may happen anyway, where necessary.
//! - `global_allocator`: Set [`allocator::Allocator`] as the global Rust
//!   allocator. This is a simple allocator that relies on the UEFI pool
//!   allocator. You can choose to provide your own allocator instead of
//!   using this feature, or no allocator at all if you don't need to
//!   dynamically allocate any memory.
//! - `logger`: Logging implementation for the standard [`log`] crate
//!   that prints output to the UEFI console. No buffering is done; this
//!   is not a high-performance logger.
//! - `panic_handler`: Add a default panic handler that logs to `stdout`.
//! - `unstable`: Enable functionality that depends on [unstable
//!   features] in the nightly compiler.
//!   As example, in conjunction with the `alloc`-feature, this gate allows
//!   the `allocator_api` on certain functions.
//! - `qemu`: Enable some code paths to adapt their execution when executed
//!   in QEMU, such as using the special `qemu-exit` device when the panic
//!   handler is called.
//!
//! Some of these features, such as the `logger` or `panic_handler` features,
//! only unfold their potential when you invoke `uefi::helpers::init` as soon
//! as possible in your application.
//!
//! # Discuss and Contribute
//!
//! For general discussions, feel free to join us in our [Zulip] and ask
//! your questions there.
//!
//! Further, you can submit bugs and also ask questions in our [issue tracker].
//! Contributions in the form of a PR are also highly welcome. Check our
//! [contributing guide][contributing] for details.
//!
//! # MSRV
//! <!-- Keep in Sync with README! -->
//!
//! The minimum supported Rust version is currently 1.70.
//! Our policy is to support at least the past two stable releases.
//!
//! # License
//! <!-- Keep in Sync with README! -->
//!
//! The code in this repository is licensed under the Mozilla Public License 2.
//! This license allows you to use the crate in proprietary programs, but any
//! modifications to the files must be open-sourced.
//!
//! The full text of the license is available in the [license file][LICENSE].
//!
//! # Terminology
//!
//! Both "EFI" and "UEFI" can be used interchangeably, such as "UEFI image" or
//! "EFI image". We prefer "UEFI" in our crate and its documentation.
//!
//! [LICENSE]: https://github.com/rust-osdev/uefi-rs/blob/main/uefi/LICENSE
//! [Rust UEFI Book]: https://rust-osdev.github.io/uefi-rs/HEAD/
//! [UEFI]: https://uefi.org/
//! [Zulip]: https://rust-osdev.zulipchat.com
//! [`BootServices`]: table::boot::BootServices
//! [`GlobalAlloc`]: alloc::alloc::GlobalAlloc
//! [`SystemTable`]: table::SystemTable
//! [`unsafe_protocol`]: proto::unsafe_protocol
//! [contributing]: https://github.com/rust-osdev/uefi-rs/blob/main/CONTRIBUTING.md
//! [issue tracker]: https://github.com/rust-osdev/uefi-rs/issues
//! [spec]: https://uefi.org/specifications
//! [unstable features]: https://doc.rust-lang.org/unstable-book/

#![cfg_attr(all(feature = "unstable", feature = "alloc"), feature(allocator_api))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]
#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::ptr_as_ptr,
    clippy::use_self,
    missing_debug_implementations,
    missing_docs,
    unused
)]

#[cfg(feature = "alloc")]
extern crate alloc;
// allow referring to self as ::uefi for macros to work universally (from this crate and from others)
// see https://github.com/rust-lang/rust/issues/54647
extern crate self as uefi;
#[macro_use]
extern crate uefi_raw;

#[macro_use]
pub mod data_types;
pub mod allocator;
pub mod boot;
#[cfg(feature = "alloc")]
pub mod fs;
pub mod helpers;
pub mod mem;
pub mod prelude;
pub mod proto;
pub mod runtime;
pub mod system;
pub mod table;

pub(crate) mod polyfill;

mod macros;
mod result;
mod util;

#[cfg(feature = "alloc")]
pub use data_types::CString16;
pub use data_types::{CStr16, CStr8, Char16, Char8, Event, Guid, Handle, Identify};
pub use result::{Error, Result, ResultExt, Status, StatusExt};
/// Re-export ucs2_cstr so that it can be used in the implementation of the
/// cstr16 macro. It is hidden since it's not intended to be used directly.
#[doc(hidden)]
pub use ucs2::ucs2_cstr;
pub use uefi_macros::entry;
pub use uguid::guid;
