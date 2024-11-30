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
use std::sync::Arc;

use arc_swap::ArcSwap;
use bytes::BufMut;
use tokio::net::UdpSocket;

use crate::Transport;

/// Udp transport implementation.
pub struct UdpTransport {
    io: Arc<ArcSwap<UdpSocket>>,
    bind: SocketAddr,
}

impl UdpTransport {
    pub async fn new(bind: SocketAddr) -> std::io::Result<Self> {
        let io = UdpSocket::bind(bind).await?;
        let bind = io.local_addr()?;
        let io = Arc::new(ArcSwap::from_pointee(io));

        Ok(UdpTransport { io, bind })
    }

    pub fn from_std(io: std::net::UdpSocket) -> std::io::Result<Self> {
        let io = UdpSocket::from_std(io)?;
        let bind = io.local_addr()?;
        let io = Arc::new(ArcSwap::from_pointee(io));

        Ok(UdpTransport { io, bind })
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.bind
    }
}

#[async_trait::async_trait]
impl Transport for UdpTransport {
    async fn send(&self, buf: &[u8], to: SocketAddr) -> std::io::Result<usize> {
        self.io.load().send_to(buf, to).await
    }

    async fn recv(&self, buf: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        self.io.load().recv_from(buf).await
    }

    async fn recv_buf<B: BufMut + Send>(
        &self,
        buf: &mut B,
    ) -> std::io::Result<(usize, SocketAddr)> {
        self.io.load().recv_buf_from(buf).await
    }

    fn clone(&self) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        Ok(UdpTransport {
            io: self.io.clone(),
            bind: self.bind,
        })
    }

    fn local_port(&self) -> Option<u16> {
        self.io.load().local_addr().ok().map(|addr| addr.port())
    }
}
