//! Structs representing headers relevant in a WebSocket context.
//!
//! These headers are commonly used in WebSocket requests and responses.
//! The `Header` trait from the `hyper` crate is used.

pub use self::key::WebSocketKey;
pub use self::accept::WebSocketAccept;
#[cfg(feature="handshake")]
pub use self::protocol::WebSocketProtocol;
#[cfg(feature="handshake")]
pub use self::version::WebSocketVersion;
#[cfg(feature="handshake")]
pub use self::extensions::WebSocketExtensions;
#[cfg(feature="handshake")]
pub use self::origin::Origin;
#[cfg(feature="handshake")]
pub use hyper::header::*;

mod accept;
mod key;
#[cfg(feature="handshake")]
mod protocol;
#[cfg(feature="handshake")]
mod version;
#[cfg(feature="handshake")]
pub mod extensions;
#[cfg(feature="handshake")]
mod origin;
