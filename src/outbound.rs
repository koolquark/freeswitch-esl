use std::net::SocketAddr;

use tokio::net::{TcpListener, ToSocketAddrs};

use crate::{connection::EslConnection, EslConnectionType, EslError};

use tracing::trace;

pub struct Outbound {
    listener: TcpListener,
}
impl Outbound {
    pub(crate) async fn bind(addr: impl ToSocketAddrs) -> Result<Self, EslError> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }
    pub async fn accept(&self) -> Result<(EslConnection, SocketAddr), EslError> {
        let (stream, addr) = self.listener.accept().await?;
        trace!("accepted incomming connection");
        let connection =
            EslConnection::with_tcpstream(stream, "None", EslConnectionType::Outbound, None).await?;
        Ok((connection, addr))
    }
}
