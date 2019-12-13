use super::constant;
use failure::Error;
use futures_io::AsyncWrite;
use futures_util::AsyncWriteExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthMethodNegotiationReply {
    pub selected_method: u8,
}

impl AuthMethodNegotiationReply {
    pub async fn write<AW>(&self, writer: &mut AW) -> Result<(), Error>
    where
        AW: AsyncWrite + Unpin,
    {
        let buf = [constant::protocol_version::SOCKS5, self.selected_method];
        writer.write_all(&buf).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::*;
    use futures_util::io::Cursor;

    #[test]
    fn happy_path() {
        let mut writer = [0u8; 2];
        let mut writer = Cursor::new(&mut writer[..]);
        let res = AuthMethodNegotiationReply {
            selected_method: 0xFF,
        };
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), ());
        assert_eq!(writer.into_inner(), [5, 0xFF])
    }

    #[test]
    fn unable_to_write_whole_thing() {
        let mut writer = [0u8; 1];
        let mut writer = Cursor::new(&mut writer[..]);
        let res = AuthMethodNegotiationReply {
            selected_method: 0xFF,
        };
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        let err = err.downcast::<std::io::Error>().unwrap();
        assert_eq!(err.kind(), std::io::ErrorKind::WriteZero);
        assert_eq!(writer.into_inner(), [5])
    }
}
