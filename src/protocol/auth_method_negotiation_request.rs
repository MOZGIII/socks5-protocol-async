use super::shared_internal::*;
use failure::Error;
use futures::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthMethodNegotiationRequest {
    pub methods: Vec<u8>,
}

impl AuthMethodNegotiationRequest {
    pub async fn read<AR>(reader: &mut AR) -> Result<Self, Error>
    where
        AR: AsyncRead + Unpin,
    {
        read_version(reader).await?;

        let nmethods = read_u8(reader).await?;
        let methods = read_vec(reader, nmethods as usize).await?;

        Ok(Self { methods })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn happy_path() {
        let mut reader = std::io::Cursor::new(&[
            5, // Version
            2, // Auth Methods Number
            0xBE, 0xEF, // The methods
        ]);
        let future = AuthMethodNegotiationRequest::read(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(
            result.unwrap(),
            AuthMethodNegotiationRequest {
                methods: vec![0xBE, 0xEF]
            }
        )
    }

    #[test]
    fn invalid_version() {
        let mut reader = std::io::Cursor::new(&[1]);
        let future = AuthMethodNegotiationRequest::read(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        let err = err
            .downcast::<crate::error::InvalidProtocolVersionError>()
            .unwrap();
        assert_eq!(err, crate::error::InvalidProtocolVersionError(1))
    }

    #[test]
    fn not_enough_data() {
        let mut reader = std::io::Cursor::new(&[]);
        let future = AuthMethodNegotiationRequest::read(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        let err = err.downcast::<std::io::Error>().unwrap();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);
    }
}
