#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum ProtocolVersion {
    Socks5 = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Primitive)]
pub enum AuthMethod {
    NoAuthenticationRequired = 0x00,
    GSSAPI = 0x01,
    UsernamePassword = 0x02,
    NoAcceptableMethods = 0xFF,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Primitive)]
pub enum CommandType {
    Connect = 0x01,
    Bind = 0x02,
    UdpAssociate = 0x03,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Primitive)]
pub enum AddressType {
    IPv4 = 0x01,
    DomainName = 0x03,
    IPv6 = 0x04,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Primitive)]
pub enum ReplyCode {
    Succeeded = 0x00,
    GeneralSocksServerFailure = 0x01,
    ConnectionNotAllowedByRuleset = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefused = 0x05,
    TtlExpired = 0x06,
    CommandNotSupported = 0x07,
    AddressTypeNotSupported = 0x08,
}
