#![feature(custom_inner_attributes)]
#![feature(debug_closure_helpers)]
#![feature(decl_macro)]

pub use self::app::App;

mod app;
mod r#const;
mod markdown;
mod presets;
mod utils;
