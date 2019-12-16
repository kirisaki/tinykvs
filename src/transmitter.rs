use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::Receiver;
use tokio::net::udp::SendHalf;

use crate::types::*;
use crate::store::*;

pub struct Client {
    pub sock: SendHalf,
    pub chan: Receiver<Packet>,
    pub store: Store,
}

impl Client {
    pub async fn run(self) -> Result<(), std::io::Error> {
        let Client {mut sock, mut chan, store} = self;
        while let Some(p) = chan.recv().await {
            let (resp, dest) = p.parse()?.respond(store.clone()).await?;
            sock.send_to(&resp, &dest).await?;
        };

        Ok(())
    }
}
