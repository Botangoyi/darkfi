use async_std::{future::timeout, net::TcpStream};
use std::{net::SocketAddr, time::Duration};
use url::Url;

use crate::{Error, Result};

use super::{
    transport::{TcpTransport, TlsTransport},
    Channel, ChannelPtr, SettingsPtr, Transport,
};

/// Create outbound socket connections.
pub struct Connector {
    settings: SettingsPtr,
}

impl Connector {
    /// Create a new connector with default network settings.
    pub fn new(settings: SettingsPtr) -> Self {
        Self { settings }
    }

    /// Establish an outbound connection.
    pub async fn connect(&self, hosturl: SocketAddr) -> Result<ChannelPtr> {
        let mut url = Url::parse(&hosturl.to_string())?;
        url.set_host(Some("tcp"))?;
        let result =
            timeout(Duration::from_secs(self.settings.connect_timeout_seconds.into()), async {
                match url.scheme() {
                    "tcp" => {
                        let transport = TcpTransport::new(None, 1024);
                        let stream = transport.dial(url).unwrap().await.unwrap();
                        Channel::new(Box::new(stream), hosturl).await
                    }
                    "tls" => {
                        let transport = TlsTransport::new(None, 1024);
                        let stream = transport.dial(url).unwrap().await.unwrap();
                        Channel::new(Box::new(stream), hosturl).await
                    }
                    _ => unimplemented!(),
                }
            })
            .await
            .unwrap();
        Ok(result)
    }
}