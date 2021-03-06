use communication::decrypt;
use communication::Communicate;

use failure::Error;
use mitm_handshake::MitmHandshake;

use result::ResultOptionExt;
pub struct MitmSession<T: Communicate> {
    client_stream: T,
    server_stream: T,
    client_key: Option<Vec<u8>>,
    server_key: Option<Vec<u8>>,
}

impl<T: Communicate> MitmSession<T> {
    pub fn new<U: MitmHandshake<T>>(
        mut client_stream: T,
        mut server_stream: T,
    ) -> Result<MitmSession<T>, Error> {
        let key = U::handshake(&mut client_stream, &mut server_stream)?;
        let (client_key, server_key) = (key.clone(), key);
        Ok(MitmSession {
            client_stream,
            server_stream,
            client_key: Some(client_key),
            server_key: Some(server_key),
        })
    }

    pub fn send_server(&mut self, message: &[u8]) -> Result<(), Error> {
        self.server_stream.send(message)
    }

    pub fn send_client(&mut self, message: &[u8]) -> Result<(), Error> {
        self.client_stream.send(message)
    }

    pub fn receive_server(&mut self) -> Result<Option<Vec<u8>>, Error> {
        self.server_stream.receive()
    }

    pub fn receive_client(&mut self) -> Result<Option<Vec<u8>>, Error> {
        self.client_stream.receive()
    }

    pub fn decrypt_client(&self, message: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.client_key
            .as_ref()
            .map(|key| decrypt(message, key))
            .invert()
    }

    pub fn decrypt_server(&self, message: &[u8]) -> Result<Option<Vec<u8>>, Error> {
        self.server_key
            .as_ref()
            .map(|key| decrypt(message, key))
            .invert()
    }

    pub fn server_stream(&self) -> &T {
        &self.server_stream
    }
}
