use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    SetBrightness(u8),
    GetBrightness,
    PowerOff,
    Reboot,
    EnableScreen(u8),
    DisableScreen(u8),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    SetBrightnessSuccess,
    SetBrightnessError,
    GetBrightnessSuccess(u8),
    GetBrightnessError,
    GenericSuccess,
    GenericError,
}

#[derive(Serialize, Deserialize)]
pub struct ToDaemon(u64, Request);
impl ToDaemon {
    pub fn new(id: u64, req: Request) -> Self {
        Self(id, req)
    }

    pub fn request(&self) -> &Request {
        &self.1
    }

    pub fn id(&self) -> u64 {
        self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct FromDaemon(u64, Response);
impl FromDaemon {
    pub fn new(id: u64, resp: Response) -> Self {
        Self(id, resp)
    }

    pub fn response(&self) -> &Response {
        &self.1
    }
}
