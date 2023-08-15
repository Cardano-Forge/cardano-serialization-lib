use crate::*;

pub(super) const GENESIS_KEY_DELEGATION_INDEX: u64 = 5;

impl cbor_event::se::Serialize for GenesisKeyDelegation {
    fn serialize<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_array(cbor_event::Len::Len(4))?;
        self.serialize_as_embedded_group(serializer)
    }
}

impl SerializeEmbeddedGroup for GenesisKeyDelegation {
    fn serialize_as_embedded_group<'se, W: Write>(
        &self,
        serializer: &'se mut Serializer<W>,
    ) -> cbor_event::Result<&'se mut Serializer<W>> {
        serializer.write_unsigned_integer(GENESIS_KEY_DELEGATION_INDEX)?;
        self.genesishash.serialize(serializer)?;
        self.genesis_delegate_hash.serialize(serializer)?;
        self.vrf_keyhash.serialize(serializer)?;
        Ok(serializer)
    }
}

impl Deserialize for GenesisKeyDelegation {
    fn deserialize<R: BufRead + Seek>(raw: &mut Deserializer<R>) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            let len = raw.array()?;
            let ret = Self::deserialize_as_embedded_group(raw, len);
            match len {
                cbor_event::Len::Indefinite => match raw.special()? {
                    CBORSpecial::Break => {}
                    _ => return Err(DeserializeFailure::EndingBreakMissing.into()),
                },
                _ => {}
            }
            ret
        })()
        .map_err(|e| e.annotate("GenesisKeyDelegation"))
    }
}

impl DeserializeEmbeddedGroup for GenesisKeyDelegation {
    fn deserialize_as_embedded_group<R: BufRead + Seek>(
        raw: &mut Deserializer<R>,
        len: cbor_event::Len,
    ) -> Result<Self, DeserializeError> {
        (|| -> Result<_, DeserializeError> {
            if let cbor_event::Len::Len(n) = len {
                if n != 4 {
                    return Err(DeserializeFailure::CBOR(cbor_event::Error::WrongLen(
                        4,
                        len,
                        "(cert_index, genesishash, genesis_delegate_hash, vrf_keyhash)",
                    ))
                    .into());
                }
            }

            let cert_index = raw.unsigned_integer()?;
            if cert_index != GENESIS_KEY_DELEGATION_INDEX {
                return Err(DeserializeFailure::FixedValueMismatch {
                    found: Key::Uint(cert_index),
                    expected: Key::Uint(GENESIS_KEY_DELEGATION_INDEX),
                }
                .into());
            }
            Ok(())
        })()
        .map_err(|e| e.annotate("cert_index"))?;
        let genesishash =
            (|| -> Result<_, DeserializeError> { Ok(GenesisHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("genesishash"))?;
        let genesis_delegate_hash =
            (|| -> Result<_, DeserializeError> { Ok(GenesisDelegateHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("genesis_delegate_hash"))?;
        let vrf_keyhash =
            (|| -> Result<_, DeserializeError> { Ok(VRFKeyHash::deserialize(raw)?) })()
                .map_err(|e| e.annotate("vrf_keyhash"))?;
        Ok(GenesisKeyDelegation {
            genesishash,
            genesis_delegate_hash,
            vrf_keyhash,
        })
    }
}
