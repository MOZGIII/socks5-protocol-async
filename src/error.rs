use failure::Fail;

#[derive(Debug, Fail, PartialEq, Eq)]
#[fail(display = "invalid protocol version: {}", _0)]
pub struct InvalidProtocolVersionError(pub u8);

#[derive(Debug, Fail, PartialEq, Eq)]
#[fail(display = "invalid address type: {}", _0)]
pub struct InvalidAddressTypeError(pub u8);
