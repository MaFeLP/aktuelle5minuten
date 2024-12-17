//!
//! This module contains user-facing routes that also handle templating and displaying
//!

mod dates_route;
mod index_route;
mod pdfcreate_route;
mod pdflist_route;
mod tinder_route;

pub(crate) use dates_route::dates;
pub(crate) use index_route::index;
pub(crate) use pdfcreate_route::pdfcreate;
pub(crate) use pdflist_route::pdflist;
pub(crate) use tinder_route::tinder;
