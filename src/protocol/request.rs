use super::address::Address;
use super::shared_internal::*;
use failure::Error;
use futures_io::AsyncRead;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub command: u8,
    pub address: super::address::Address,
    pub port: u16,
}

impl Request {
    pub async fn read<AR>(reader: &mut AR) -> Result<Self, Error>
    where
        AR: AsyncRead + Unpin,
    {
        super::shared_internal::read_version(reader).await?;

        let command = read_u8(reader).await?;
        let _reserved = read_u8(reader).await?;
        let address = Address::read(reader).await?;
        let port = read_u16(reader).await?;

        Ok(Self {
            command,
            address,
            port,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::*;
    use futures_util::io::Cursor;

    #[test]
    fn happy_path() {
        let mut reader = Cursor::new(&[
            5,    // Version
            0x01, // Command
            0x00, // Reserved octet
            0x01, 127, 0, 0, 1, // Address
            0x00, 80, // Port
        ]);
        let future = Request::read(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(
            result.unwrap(),
            Request {
                command: 0x01,
                address: Address::IPv4([127, 0, 0, 1]),
                port: 80,
            }
        )
    }

    #[test]
    fn invalid_version() {
        let mut reader = Cursor::new(&[1]);
        let future = Request::read(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        let err = err
            .downcast::<crate::error::InvalidProtocolVersionError>()
            .unwrap();
        assert_eq!(err, crate::error::InvalidProtocolVersionError(1))
    }

    #[test]
    fn not_enough_data() {
        let mut reader = Cursor::new(&[]);
        let future = Request::read(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        let err = err.downcast::<std::io::Error>().unwrap();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }
}
