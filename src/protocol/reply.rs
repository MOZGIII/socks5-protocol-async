use super::address::Address;
use super::constant;
use futures::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub reply_code: u8,
    pub address: Address,
    pub port: u16,
}

impl Reply {
    pub async fn write<AW>(&self, writer: &mut AW) -> std::io::Result<()>
    where
        AW: AsyncWrite + Unpin,
    {
        let mut buf = Vec::with_capacity(3 + (2 + 0xFF) + 2);
        buf.extend(&[constant::protocol_version::SOCKS5, self.reply_code, 0x00]);
        self.address.write(&mut buf).await?;
        buf.extend(&self.port.to_be_bytes());
        writer.write_all(&buf).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn happy_path() {
        let mut writer = std::io::Cursor::new([0u8; 10]);
        let res = Reply {
            reply_code: 0x01,
            address: Address::IPv4([127, 0, 0, 1]),
            port: 80,
        };
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), ());
        assert_eq!(
            writer.into_inner(),
            [5u8, 0x01, 0x00, 0x01, 127, 0, 0, 1, 00, 80]
        )
    }

    #[test]
    fn unable_to_write_whole_thing() {
        let mut writer = std::io::Cursor::new([0u8; 2]);
        let res = Reply {
            reply_code: 0x01,
            address: Address::IPv4([127, 0, 0, 1]),
            port: 80,
        };
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::WriteZero);
        assert_eq!(writer.into_inner(), [5u8, 0x01])
    }
}
