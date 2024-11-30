/*
 * Copyright 2024 RingNet
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 */

use std::net::SocketAddr;

pub use async_trait::async_trait;
use bytes::BufMut;
#[cfg(feature = "std")]
pub use std_impl::UdpTransport;

#[cfg(feature = "std")]
mod std_impl;

#[async_trait::async_trait]
pub trait Transport: Send + Sync {
    /// Sends data to the given address.
    async fn send(&self, buf: &[u8], to: SocketAddr) -> std::io::Result<usize>;

    /// Receives data from remote.
    async fn recv(&self, buf: &mut [u8]) -> std::io::Result<(usize, SocketAddr)>;

    /// Receives data from remote, advancing the buffer's internal cursor.
    async fn recv_buf<B: BufMut + Send>(&self, buf: &mut B)
        -> std::io::Result<(usize, SocketAddr)>;

    /// Clone this transport.
    fn clone(&self) -> std::io::Result<Self>
    where
        Self: Sized;

    fn local_port(&self) -> Option<u16>;
}
