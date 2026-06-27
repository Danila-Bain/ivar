#![feature(float_masks)]
#![feature(const_cmp)]
#![feature(const_trait_impl)]
#![feature(
    strict_provenance_lints,
    must_not_suspend,
    non_exhaustive_omitted_patterns_lint
)]
#![no_std]
#![deny(
    clippy::float_arithmetic,
    reason = "Using operations with rounding to nearest is suspicious in the context of implementing interval arithmetic"
)]
#![deny(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::pedantic,
    // clippy::nursery,

    // ── Restrictions (strict subset useful for scientific/numerical code) ─
    clippy::absolute_paths,
    clippy::alloc_instead_of_core,
    clippy::allow_attributes,
    clippy::allow_attributes_without_reason,
    // clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    // clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    // clippy::exhaustive_enums,
    // clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::fn_to_numeric_cast_any,
    clippy::format_push_string,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_assert_message,
    clippy::missing_asserts_for_indexing,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mixed_read_write_in_expression,
    clippy::modulo_arithmetic,
    clippy::multiple_unsafe_ops_per_block,
    clippy::mutex_atomic,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::partial_pub_fields,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::pub_without_shorthand,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    // clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::semicolon_inside_block,
    clippy::semicolon_outside_block,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_slice,
    clippy::implicit_clone,
    clippy::try_err,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,

    // ── Rustc lints ───────────────────────────────────────────────────────
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    fuzzy_provenance_casts,
    keyword_idents,
    let_underscore_drop,
    lossy_provenance_casts,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    must_not_suspend,
    non_ascii_idents,
    non_exhaustive_omitted_patterns,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unnameable_types,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    // unstable_features,
    // unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    clippy::todo,
    // clippy::missing_docs_in_private_items,
    // missing_docs,
)]
#![warn(clippy::cargo, clippy::complexity, clippy::perf)]
//! # Examples
//!
//! ## Basic usage
//! ```
//! use ivar::Interval;
//!
//! let x = Interval::new(1., 4.);
//! let y = Interval::new(10., 20.);
//!
//! assert_eq!(x + y, Interval::new(11., 24.));
//! assert_eq!(x.sqrt(), Interval::new(1., 2.));
//! ```
//!
//! ## Available constructors and constants
//! ```
//! use ivar::{Interval, iv};
//!
//! let a = Interval::new(1., 2.);
//! let b = Interval::new_singleton(3.);
//! let c = iv!(1., 2.);
//! let d = iv!(3.);
//!
//! assert_eq!(a, c);
//! assert_eq!(b, d);
//!
//! let nai_1 = Interval::NAI;
//! let nai_2 = Interval::nai();
//!
//! let empty_1 = Interval::EMPTY;
//! let empty_2 = Interval::empty();
//!
//! let entire_1 = Interval::ENTIRE;
//! let entire_2 = Interval::entire();
//! ```
//!
//! # Roadmap for IEEE 1788-2015
//!
//! - The standard distinguishes between four specification levels:
//!   - Level 1: mathematical description of interval operations
//!   - Level 2: specification of interval operations with finite-precision. The elements of the finite set of finite-precision intervals are called interval datums.
//!   - Level 3: specification of possible representations of intervals.
//!   - Level 4: specification of interchange encodings to be used to share interval datums between standard-conforming software (which can be different from internal representation).
//!
//! - The standard defines three *accuracy modes*: _tightest_, _accurate_, and _valid_. Here tightest is the strongest requirement, and valid is the weakest. *Tightest* interval functions produce the finite-precision convex hull of an exact interval image of the corresponding point function. The standard requires that the folloiwng functions must be tightest: all the basic operations (neg, add, sub, mul, div, recip, sqr, sqrt, and fma), all integer functions (`sign`, `ceil`, `floor`, `trunc`, `round_ties_even`, and `round_ties_odd`), and all absmax functions (`abs`, `max`, `min`). Note that since the outputs of integer and absmax functions are representable in f64, they are exact on the set of finite-precision intervals. *Valid* function output is only required to contain the image of the corresponding point function at the input interval. *Accurate* functions are valid functions, the output of which is contained in next-out finite-precision interval hull of an image of the corresponding point function at the next-out input interval. Here, by next-out interval we understand the interval, the bounds of which were extended outwards to the next representable floating point number. For inf-sup types (which include `f64`) all standard functions must be accurate.

#[derive(Clone, Copy, Debug)]
pub struct Interval(f64, f64);

/// Implementation of constructors for [`Interval`], including macro [`iv`], methods [`Interval::new`], [`Interval::new_singleton`], and more.
pub(crate) mod constructors;
/// Definitions of common interval constants, such as `EMPTY`, `ENTIRE`, `NAI`, etc, and tightest intervals enclosing some mathematical constants, such as `PI`, `E`, etc.
pub(crate) mod consts;
/// Implementation of numeric functions of integrals, including `inf`, `sup`, `mid`, `rad`, and more.
pub(crate) mod numeric;
/// Implementation of set operations of intervals, such as intersection (`inter`), interval hull (`hull`), and more.
pub(crate) mod set_operations;

/// Implementation of inverval comparisons
pub(crate) mod cmp;
pub use cmp::Overlap;

/// Implementation of arithmetical operations for f64 with controlled rounding direction
pub mod rounded_arithmetic;

/// Implementation of forward elementary functions on intervals
pub(crate) mod elementary_functions;
/// Implementation of reverse elementary functions on intervals
pub(crate) mod reverse_elementary_functions;

/// Implementation of decorated intervals
pub(crate) mod decorations;
pub use decorations::{DInterval, Decoration};

/// Extending basic binary interval operations (`+-*/`) to references and bare `f64` values.
pub(crate) mod forward_operations;

/// Implementation of [`num_traits::Num`] and [`num_traits::Float`] for intervals
#[cfg(feature = "num_traits")]
pub mod impl_num_traits;

#[cfg(feature = "complex")]
pub mod complex_box;
pub use complex_box::ComplexBox;
