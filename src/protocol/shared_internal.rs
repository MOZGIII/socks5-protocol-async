use super::constant;
use failure::Error;
use futures::prelude::*;

pub async fn read_u8<AR>(reader: &mut AR) -> std::io::Result<u8>
where
    AR: AsyncRead + Unpin,
{
    let mut buf = [0u8; 1];
    reader.read_exact(buf.as_mut()).await?;
    Ok(buf[0])
}

pub async fn read_u16<AR>(reader: &mut AR) -> std::io::Result<u16>
where
    AR: AsyncRead + Unpin,
{
    let mut buf = [0u8; 2];
    reader.read_exact(buf.as_mut()).await?;
    Ok(u16::from_be_bytes(buf))
}

pub async fn read_vec<AR>(reader: &mut AR, len: usize) -> std::io::Result<Vec<u8>>
where
    AR: AsyncRead + Unpin,
{
    let mut v = Vec::<u8>::with_capacity(len);
    v.resize(len, 0);
    reader.read_exact(&mut v).await?;
    Ok(v)
}

pub async fn read_version<AR>(reader: &mut AR) -> Result<(), Error>
where
    AR: AsyncRead + Unpin,
{
    let version = read_u8(reader).await?;
    if version != constant::protocol_version::SOCKS5 as u8 {
        return Err(crate::error::InvalidProtocolVersionError(version))?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn read_u8_happy_path() {
        let mut reader = std::io::Cursor::new(&[0xFE]);
        let future = read_u8(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), 0xFE)
    }

    #[test]
    fn read_u8_not_enough_data() {
        let mut reader = std::io::Cursor::new(&[]);
        let future = read_u8(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn read_u16_happy_path() {
        let mut reader = std::io::Cursor::new(&[0xBE, 0xEF]);
        let future = read_u16(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), 0xBEEF)
    }

    #[test]
    fn read_u16_not_enough_data() {
        let mut reader = std::io::Cursor::new(&[]);
        let future = read_u16(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn read_u16_not_enough_data_half() {
        let mut reader = std::io::Cursor::new(&[0xBE]);
        let future = read_u16(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn read_vec_happy_path() {
        let mut reader = std::io::Cursor::new(&[0xBE, 0xEF]);
        let future = read_vec(&mut reader, 2);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), vec![0xBE, 0xEF])
    }

    #[test]
    fn read_vec_not_enough_data() {
        let mut reader = std::io::Cursor::new(&[]);
        let future = read_vec(&mut reader, 2);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[test]
    fn read_vec_not_enough_data_partial() {
        let mut reader = std::io::Cursor::new(&[0xBE]);
        let future = read_vec(&mut reader, 2);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }
}
