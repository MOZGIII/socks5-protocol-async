pub mod protocol_version {
    pub type ProtocolVersion = u8;

    pub const SOCKS5: ProtocolVersion = 5;
}

pub mod auth_method {
    pub type AuthMethod = u8;

    pub const NO_AUTHENTICATION_REQUIRED: AuthMethod = 0x00;
    pub const GSSAPI: AuthMethod = 0x01;
    pub const USERNAME_PASSWORD: AuthMethod = 0x02;
    pub const NO_ACCEPTABLE_METHODS: AuthMethod = 0xFF;

}

pub mod command_type {
    pub type CommandType = u8;

    pub const CONNECT: CommandType = 0x01;
    pub const BIND: CommandType = 0x02;
    pub const UDP_ASSOCIATE: CommandType = 0x03;
}

pub mod address_type {
    pub type AddressType = u8;

    pub const IPV4: AddressType = 0x01;
    pub const DOMAIN_NAME: AddressType = 0x03;
    pub const IPV6: AddressType = 0x04;
}

pub mod reply_code {
    pub type ReplyCode = u8;

    pub const SUCCEEDED: ReplyCode = 0x00;
    pub const GENERAL_SOCKS_SERVER_FAILURE: ReplyCode = 0x01;
    pub const CONNECTION_NOT_ALLOWED_BY_RULESET: ReplyCode = 0x02;
    pub const NETWORK_UNREACHABLE: ReplyCode = 0x03;
    pub const HOST_UNREACHABLE: ReplyCode = 0x04;
    pub const CONNECTION_REFUSED: ReplyCode = 0x05;
    pub const TTL_EXPIRED: ReplyCode = 0x06;
    pub const COMMAND_NOT_SUPPORTED: ReplyCode = 0x07;
    pub const ADDRESS_TYPE_NOT_SUPPORTED: ReplyCode = 0x08;
}
