/// A helper macro for accessing read-only registers from `memory_map`.
#[macro_export]
macro_rules! reg_ro {
    ($name:ident, $output:expr) => {
        paste::paste! {
            pub fn[<read_ $name>](&mut self) -> Result<$output> {
                Ok(memory_map::$name::read(&mut self.transport_layer)?.into())
            }
        }
    };
}

/// A helper macro for accessing read-write registers from `memory_map`.
#[macro_export]
macro_rules! reg_rw {
    ($name:ident, $input:expr) => {
        paste::paste! {
            pub fn[<read_ $name>](&mut self) -> Result<$input> {
                Ok(memory_map::$name::read(&mut self.transport_layer)?.into())
            }
        }

        paste::paste! {
            pub fn[<write_ $name>](&mut self, data: $input) -> Result<()> {
                memory_map::$name::write(&mut self.transport_layer, data.into())
            }
        }
    };
}

/// A helper macro for mapping registers.
#[macro_export]
macro_rules! map {
    ($name:ident, $addr:expr) => {
        pub(crate) struct $name;

        impl $name {
            pub(crate) const fn get_addr() -> u8 {
                $addr
            }
            pub(crate) fn read<L: Layer>(transport_layer: &mut L) -> crate::Result<u8> {
                transport_layer.read_byte(Self::get_addr())
            }
            // TODO: This should be conditional. Not all registers are writable.
            pub(crate) fn write<L: Layer>(transport_layer: &mut L, data: u8) -> crate::Result<()> {
                transport_layer.write_byte(Self::get_addr(), data)
            }
        }
    };
}

#[macro_export]
macro_rules! get_ro {
    ($name:ident, $ty:ty, [$lsb:ident, $msb:ident]) => {
        reg_ro!($lsb, u8);
        reg_ro!($msb, u8);

        paste::paste! {
            pub fn[<get_ $name>](&mut self) -> Result<$ty> {
                match self.$name {
                    Some(v) => Ok(v),
                    None => {
                        let lsb = self.[<read_ $lsb>]()?;
                        let msb = self.[<read_ $msb>]()?;
                        let v = u16::from_le_bytes([lsb, msb]) as $ty;

                        self.$name = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
    ($name:ident, $ty:ty) => {
        reg_ro!($name, u8);

        paste::paste! {
            pub fn[<get_ $name>](&mut self) -> Result<$ty> {
                match self.$name {
                    Some(v) => Ok(v),
                    None => {
                        let v = self.[<read_ $name>]()? as $ty;

                        self.$name = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
    ($name:ident) => {
        reg_ro!($name, u8);

        paste::paste! {
            pub fn[<get_ $name>](&mut self) -> Result<u8> {
                match self.$name {
                    Some(v) => Ok(v),
                    None => {
                        let v = self.[<read_ $name>]()?;

                        self.$name = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
}
