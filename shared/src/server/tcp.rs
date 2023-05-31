use crate::server::error::ServerError;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

use super::{proxy_protocol, Listener};
use async_trait::async_trait;

pub struct TcpServer {
    inner: TcpListener,
}

impl TcpServer {
    pub async fn bind(addr: impl ToSocketAddrs) -> super::error::ServerResult<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { inner: listener })
    }
}

#[async_trait]
impl Listener for TcpServer {
    type Connection = TcpStream;
    type Error = ServerError;
    async fn accept(&mut self) -> Result<Self::Connection, Self::Error> {
        let (conn, _socket_addr) = self.inner.accept().await?;
        Ok(conn)
    }
}

pub struct TcpServerWithProxyProtocol {
    inner: TcpListener,
}

impl TcpServerWithProxyProtocol {
    pub async fn bind(addr: impl ToSocketAddrs) -> super::error::ServerResult<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { inner: listener })
    }
}

#[async_trait]
impl Listener for TcpServerWithProxyProtocol {
    type Connection = proxy_protocol::AcceptedConn<TcpStream>;
    type Error = ServerError;
    async fn accept(&mut self) -> Result<Self::Connection, Self::Error> {
        let (conn, _socket_addr) = self.inner.accept().await?;
        let proxy_protocol_conn = proxy_protocol::try_parse_proxy_protocol(conn).await?;
        Ok(proxy_protocol_conn)
    }
}
