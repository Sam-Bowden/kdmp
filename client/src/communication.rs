use crate::error_menu::ErrorMenu;
use bincode;
use communication::Request;
use communication::Response;
use std::io::Read;
use std::os::unix::net::UnixStream;

pub trait RequestOps {
    fn send_command(&self) -> Result<Response, ErrorMenu>;
}

impl RequestOps for Request {
    fn send_command(&self) -> Result<Response, ErrorMenu> {
        let Ok(mut stream) = UnixStream::connect("/tmp/kdmp.sock") else {
            return Err(ErrorMenu::FailedToConnectToDaemon);
        };

        if let Err(_) = bincode::serialize_into(&stream, &self) {
            return Err(ErrorMenu::FailedToConnectToDaemon);
        }

        stream.shutdown(std::net::Shutdown::Write).unwrap();

        let mut buffer = vec![];
        stream.read_to_end(&mut buffer).unwrap();

        Ok(bincode::deserialize(&buffer).unwrap())
    }
}
