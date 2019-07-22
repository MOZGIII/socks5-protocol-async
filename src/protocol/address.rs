use super::constant::*;
use super::shared_internal::*;
use failure::Error;
use futures::prelude::*;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Address {
    IPv4([u8; 4]),
    IPv6([u8; 16]),
    DomainName(Vec<u8>),
}

impl Address {
    pub async fn read<AR>(reader: &mut AR) -> Result<Self, Error>
    where
        AR: AsyncRead + Unpin,
    {
        let address_type = read_u8(reader).await?;
        match AddressType::from_u8(address_type) {
            Some(AddressType::IPv4) => {
                let mut buf = [0u8; 4];
                reader.read_exact(buf.as_mut()).await?;
                Ok(Address::IPv4(buf))
            }
            Some(AddressType::IPv6) => {
                let mut buf = [0u8; 16];
                reader.read_exact(buf.as_mut()).await?;
                Ok(Address::IPv6(buf))
            }
            Some(AddressType::DomainName) => {
                let len = read_u8(reader).await?;
                let v = read_vec(reader, len as usize).await?;
                Ok(Address::DomainName(v))
            }
            _ => Err(crate::error::InvalidAddressTypeError(address_type))?,
        }
    }

    pub async fn write<AW>(&self, writer: &mut AW) -> std::io::Result<()>
    where
        AW: AsyncWrite + Unpin,
    {
        match self {
            Address::IPv4(val) => {
                let head = [AddressType::IPv4 as u8];
                writer.write_all(&head).await?;
                writer.write_all(val).await?;
                Ok(())
            }
            Address::IPv6(val) => {
                let head = [AddressType::IPv6 as u8];
                writer.write_all(&head).await?;
                writer.write_all(val).await?;
                Ok(())
            }
            Address::DomainName(val) => {
                let head = [AddressType::DomainName as u8, val.len() as u8];
                writer.write_all(&head).await?;
                writer.write_all(val).await?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::*;

    #[test]
    fn read_invalid_type() {
        let mut reader = std::io::Cursor::new(&[0x2]);
        let future = Address::read(&mut reader);
        let result = extract_future_output(future);
        let err = result.unwrap_err();
        let err = err
            .downcast::<crate::error::InvalidAddressTypeError>()
            .unwrap();
        assert_eq!(err, crate::error::InvalidAddressTypeError(0x2));
    }

    #[test]
    fn read_ipv4_ok() {
        let mut reader = std::io::Cursor::new(&[AddressType::IPv4 as u8, 127, 0, 0, 1]);
        let future = Address::read(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), Address::IPv4([127, 0, 0, 1]));
    }

    #[test]
    fn read_ipv6_ok() {
        let mut reader = std::io::Cursor::new(&[
            AddressType::IPv6 as u8,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0xFF,
        ]);
        let future = Address::read(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(
            result.unwrap(),
            Address::IPv6([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF])
        );
    }

    #[test]
    fn read_domain_name_ok() {
        let mut reader = std::io::Cursor::new(&[
            AddressType::DomainName as u8,
            11,
            'e' as u8,
            'x' as u8,
            'a' as u8,
            'm' as u8,
            'p' as u8,
            'l' as u8,
            'e' as u8,
            '.' as u8,
            'c' as u8,
            'o' as u8,
            'm' as u8,
        ]);
        let future = Address::read(&mut reader);
        let result = extract_future_output(future);
        assert_eq!(
            result.unwrap(),
            Address::DomainName("example.com".as_bytes().to_vec())
        );
    }

    #[test]
    fn write_ipv4_ok() {
        let mut writer = std::io::Cursor::new([0u8; 1 + 4]);
        let res = Address::IPv4([127, 0, 0, 1]);
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), ());
        assert_eq!(writer.into_inner(), [0x1, 127, 0, 0, 1])
    }

    #[test]
    fn write_ipv6_ok() {
        let mut writer = std::io::Cursor::new([0u8; 1 + 16]);
        let res = Address::IPv6([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF]);
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), ());
        assert_eq!(
            writer.into_inner(),
            [0x4, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xFF]
        )
    }

    #[test]
    fn write_domain_name_ok() {
        let mut writer = std::io::Cursor::new([0u8; 1 + 1 + 11]);
        let res = Address::DomainName("example.com".as_bytes().to_vec());
        let future = res.write(&mut writer);
        let result = extract_future_output(future);
        assert_eq!(result.unwrap(), ());
        assert_eq!(
            writer.into_inner(),
            [
                0x3, 11, 'e' as u8, 'x' as u8, 'a' as u8, 'm' as u8, 'p' as u8, 'l' as u8,
                'e' as u8, '.' as u8, 'c' as u8, 'o' as u8, 'm' as u8,
            ]
        )
    }
}
