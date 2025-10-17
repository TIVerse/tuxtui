//! # tuxtui-macros
//!
//! Procedural macros for the tuxtui TUI library.
//!
//! This crate provides convenience macros for common operations in tuxtui.
//!
//! ## Macros
//!
//! - `border!`: Create border configurations easily
//!
//! ## Example
//!
//! ```ignore
//! use tuxtui_macros::border;
//!
//! let borders = border!(All);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input};

/// Create a border configuration.
///
/// # Example
///
/// ```ignore
/// use tuxtui_macros::border;
///
/// let borders = border!(All);
/// let borders = border!(Top | Bottom);
/// ```
#[proc_macro]
pub fn border(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);

    let expanded = quote! {
        tuxtui::widgets::block::BorderType::#ident
    };

    TokenStream::from(expanded)
}

/// Derive macro for creating styled components (placeholder for future expansion).
#[proc_macro_derive(Styled)]
pub fn derive_styled(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
