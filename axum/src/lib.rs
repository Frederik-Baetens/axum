//! axum is a web application framework that focuses on ergonomics and modularity.
//!
//! # Table of contents
//!
//! - [High level features](#high-level-features)
//! - [Compatibility](#compatibility)
//! - [Example](#example)
//! - [Routing](#routing)
//! - [Handlers](#handlers)
//! - [Extractors](#extractors)
//! - [Responses](#responses)
//! - [Error handling](#error-handling)
//! - [Middleware](#middleware)
//! - [Sharing state with handlers](#sharing-state-with-handlers)
//! - [Required dependencies](#required-dependencies)
//! - [Examples](#examples)
//! - [Feature flags](#feature-flags)
//!
//! # High level features
//!
//! - Route requests to handlers with a macro free API.
//! - Declaratively parse requests using extractors.
//! - Simple and predictable error handling model.
//! - Generate responses with minimal boilerplate.
//! - Take full advantage of the [`tower`] and [`tower-http`] ecosystem of
//!   middleware, services, and utilities.
//!
//! In particular the last point is what sets `axum` apart from other frameworks.
//! `axum` doesn't have its own middleware system but instead uses
//! [`tower::Service`]. This means `axum` gets timeouts, tracing, compression,
//! authorization, and more, for free. It also enables you to share middleware with
//! applications written using [`hyper`] or [`tonic`].
//!
//! # Compatibility
//!
//! axum is designed to work with [tokio] and [hyper]. Runtime and
//! transport layer independence is not a goal, at least for the time being.
//!
//! # Example
//!
//! The "Hello, World!" of axum is:
//!
//! ```rust,no_run
//! use axum::{
//!     routing::get,
//!     Router,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     // build our application with a single route
//!     let app = Router::new().route("/", get(|| async { "Hello, World!" }));
//!
//!     // run it with hyper on localhost:3000
//!     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! # Routing
//!
//! [`Router`] is used to setup which paths goes to which services:
//!
//! ```rust
//! use axum::{Router, routing::get};
//!
//! // our router
//! let app = Router::new()
//!     .route("/", get(root))
//!     .route("/foo", get(foo))
//!     .route("/foo/bar", get(foo_bar));
//!
//! // which calls one of these handlers
//! async fn root() {}
//! async fn foo() {}
//! async fn foo_bar() {}
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//!
//! See [`Router`] for more details on routing.
//!
//! # Handlers
//!
#![doc = include_str!("docs/handlers_intro.md")]
//!
//! See [`handler`](crate::handler) for more details on handlers.
//!
//! # Extractors
//!
//! An extractor is a type that implements [`FromRequest`]. Extractors is how
//! you pick apart the incoming request to get the parts your handler needs.
//!
//! ```rust
//! use axum::extract::{Path, Query, Json};
//! use std::collections::HashMap;
//!
//! // `Path` gives you the path parameters and deserializes them.
//! async fn path(Path(user_id): Path<u32>) {}
//!
//! // `Query` gives you the query parameters and deserializes them.
//! async fn query(Query(params): Query<HashMap<String, String>>) {}
//!
//! // Buffer the request body and deserialize it as JSON into a
//! // `serde_json::Value`. `Json` supports any type that implements
//! // `serde::Deserialize`.
//! async fn json(Json(payload): Json<serde_json::Value>) {}
//! ```
//!
//! See [`extract`](crate::extract) for more details on extractors.
//!
//! # Responses
//!
//! Anything that implements [`IntoResponse`] can be returned from handlers.
//!
//! ```rust,no_run
//! use axum::{
//!     body::Body,
//!     routing::get,
//!     response::Json,
//!     Router,
//! };
//! use serde_json::{Value, json};
//!
//! // `&'static str` becomes a `200 OK` with `content-type: text/plain`
//! async fn plain_text() -> &'static str {
//!     "foo"
//! }
//!
//! // `Json` gives a content-type of `application/json` and works with any type
//! // that implements `serde::Serialize`
//! async fn json() -> Json<Value> {
//!     Json(json!({ "data": 42 }))
//! }
//!
//! let app = Router::new()
//!     .route("/plain_text", get(plain_text))
//!     .route("/json", get(json));
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//!
//! See [`response`](crate::response) for more details on building responses.
//!
//! # Error handling
//!
//! axum aims to have a simple and predictable error handling model. That means
//! it is simple to convert errors into responses and you are guaranteed that
//! all errors are handled.
//!
//! See [`error_handling`](crate::error_handling) for more details on axum's
//! error handling model and how to handle errors gracefully.
//!
//! # Middleware
//!
#![doc = include_str!("docs/middleware.md")]
//!
//! # Sharing state with handlers
//!
//! It is common to share some state between handlers for example to share a
//! pool of database connections or clients to other services. That can be done
//! using the [`AddExtension`] middleware (applied with [`AddExtensionLayer`])
//! and the [`Extension`](crate::extract::Extension) extractor:
//!
//! ```rust,no_run
//! use axum::{
//!     AddExtensionLayer,
//!     extract::Extension,
//!     routing::get,
//!     Router,
//! };
//! use std::sync::Arc;
//!
//! struct State {
//!     // ...
//! }
//!
//! let shared_state = Arc::new(State { /* ... */ });
//!
//! let app = Router::new()
//!     .route("/", get(handler))
//!     .layer(AddExtensionLayer::new(shared_state));
//!
//! async fn handler(
//!     Extension(state): Extension<Arc<State>>,
//! ) {
//!     // ...
//! }
//! # async {
//! # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
//! # };
//! ```
//!
//! # Required dependencies
//!
//! To use axum there are a few dependencies you have pull in as well:
//!
//! ```toml
//! [dependencies]
//! axum = "<latest-version>"
//! hyper = { version = "<latest-version>", features = ["full"] }
//! tokio = { version = "<latest-version>", features = ["full"] }
//! tower = "<latest-version>"
//! ```
//!
//! The `"full"` feature for hyper and tokio isn't strictly necessary but it's
//! the easiest way to get started.
//!
//! Note that [`hyper::Server`] is re-exported by axum so if thats all you need
//! then you don't have to explicitly depend on hyper.
//!
//! Tower isn't strictly necessary either but helpful for testing. See the
//! testing example in the repo to learn more about testing axum apps.
//!
//! # Examples
//!
//! The axum repo contains [a number of examples][examples] that show how to put all the
//! pieces together.
//!
//! # Feature flags
//!
//! axum uses a set of [feature flags] to reduce the amount of compiled and
//! optional dependencies.
//!
//! The following optional features are available:
//!
//! Name | Description | Default?
//! ---|---|---
//! `headers` | Enables extracting typed headers via [`TypedHeader`] | No
//! `http1` | Enables hyper's `http1` feature | Yes
//! `http2` | Enables hyper's `http2` feature | No
//! `json` | Enables the [`Json`] type and some similar convenience functionality | Yes
//! `multipart` | Enables parsing `multipart/form-data` requests with [`Multipart`] | No
//! `tower-log` | Enables `tower`'s `log` feature | Yes
//! `ws` | Enables WebSockets support via [`extract::ws`] | No
//!
//! [`TypedHeader`]: crate::extract::TypedHeader
//! [`Multipart`]: crate::extract::Multipart
//! [`tower`]: https://crates.io/crates/tower
//! [`tower-http`]: https://crates.io/crates/tower-http
//! [`tokio`]: http://crates.io/crates/tokio
//! [`hyper`]: http://crates.io/crates/hyper
//! [`tonic`]: http://crates.io/crates/tonic
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section
//! [`IntoResponse`]: crate::response::IntoResponse
//! [`Timeout`]: tower::timeout::Timeout
//! [examples]: https://github.com/tokio-rs/axum/tree/main/examples
//! [`Router::merge`]: crate::routing::Router::merge
//! [`axum::Server`]: hyper::server::Server
//! [`OriginalUri`]: crate::extract::OriginalUri
//! [`Service`]: tower::Service
//! [`Service::poll_ready`]: tower::Service::poll_ready
//! [`tower::Service`]: tower::Service
//! [`handle_error`]: error_handling::HandleErrorExt::handle_error
//! [tower-guides]: https://github.com/tower-rs/tower/tree/master/guides
//! [`Uuid`]: https://docs.rs/uuid/latest/uuid/
//! [`FromRequest`]: crate::extract::FromRequest
//! [`HeaderMap`]: http::header::HeaderMap
//! [`Request`]: http::Request
//! [customize-extractor-error]: https://github.com/tokio-rs/axum/blob/main/examples/customize-extractor-error/src/main.rs
//! [axum-debug]: https://docs.rs/axum-debug
//! [`debug_handler`]: https://docs.rs/axum-debug/latest/axum_debug/attr.debug_handler.html
//! [`Handler`]: crate::handler::Handler
//! [`Infallible`]: std::convert::Infallible

#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

#[macro_use]
pub(crate) mod macros;

mod add_extension;
mod clone_box_service;
mod error;
#[cfg(feature = "json")]
mod json;
mod util;

pub mod body;
pub mod error_handling;
pub mod extract;
pub mod handler;
pub mod response;
pub mod routing;

#[cfg(test)]
mod test_helpers;

pub use add_extension::{AddExtension, AddExtensionLayer};
#[doc(no_inline)]
pub use async_trait::async_trait;
#[doc(no_inline)]
pub use http;
#[doc(no_inline)]
pub use hyper::Server;

#[doc(inline)]
#[cfg(feature = "json")]
pub use self::json::Json;
#[doc(inline)]
pub use self::{error::Error, routing::Router};

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
