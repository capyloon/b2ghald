use b2ghald::messages::*;
use bincode::Options;
use log::{error, info};
use std::io::{Error, ErrorKind};
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let stream = UnixStream::connect("/tmp/b2ghald.sock").map_err(|err| {
        error!("Failed to connect: {}", err);
        err
    })?;

    info!("Connected to b2ghald on /tmp/b2ghald.sock");

    let config = bincode::DefaultOptions::new().with_native_endian();
    let request = ToDaemon::new(1, Request::GetBrightness);
    config
        .serialize_into(&stream, &request)
        .map_err(|_| Error::new(ErrorKind::Other, "bincode error"))?;

    if let Ok(message) = config.deserialize_from::<_, FromDaemon>(&stream) {
        match message.response() {
            Response::GetBrightnessSuccess(value) => {
                info!("Brightness is {}", value);
            }
            _ => {
                error!("Unexpected message: {:?}", message.response());
            }
        }
    } else {
        error!("Failed to deserialize messages.");
    }

    Ok(())
}
