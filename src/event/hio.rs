use super::hevent::Hevent;

pub enum HioType {
    Unknown = 0,
    Stdin = 0x01,
    Stdout = 0x02,
    Stderr = 0x04,
    Stdio = 0x0F,
    File = 0x10,
    Ip = 0x0100,
    SockRaw = 0x0F00,
    Udp = 0x1000,
    Kcp = 0x2000,
    Dtls = 0x10000,
    SockDgraw = 0xFF000,
    Tcp = 0x100000,
    Ssl = 0x1000000,
    Tls = 0x10000010,
    Socket = 0xFFFFF00,
}

pub struct Hio<'a> {
    pub hevent: Hevent<'a>,

    pub r#type: HioType,
}
