/// maps a read method to an existing register
#[macro_export]
macro_rules! reg_ro {
    ($(#[$doc:meta])* $vis:vis, $name:ident) => {
        paste::paste! {
            $(#[$doc])*
            $vis fn [<read_ $name>](&mut self) -> $crate::Result<u8> {
                Ok($crate::memory_map::$name::read(&mut self.transport_layer)?.into())
            }
        }
    };
    ($(#[$doc:meta])* $vis:vis $name:ident, $ty:ty) => {
        paste::paste! {
            $(#[$doc])*
            $vis fn [<read_ $name>](&mut self) -> $crate::Result<$ty> {
                Ok($crate::memory_map::$name::read(&mut self.transport_layer)?.into())
            }
        }
    };
}

/// maps read and write methods to an existing register
#[macro_export]
macro_rules! reg_rw {
    ($(#[$doc:meta])* $vis:vis $name:ident) => {
        $crate::reg_ro!($name, u8);

        paste::paste! {
            $(#[$doc])*
            $vis fn [<write_ $name>](&mut self, data: u8) -> $crate::Result<()> {
                $crate::memory_map::$name::write(&mut self.transport_layer, data.into())
            }
        }
    };
    ($(#[$doc:meta])* $vis:vis $name:ident, $ty:ty) => {
        $crate::reg_ro!($name, $ty);

        paste::paste! {
            $(#[$doc])*
            $vis fn [<write_ $name>](&mut self, data: $ty) -> $crate::Result<()> {
                $crate::memory_map::$name::write(&mut self.transport_layer, data.into())
            }
        }
    };
}

/// maps registers to an address with owned read and write methods
#[macro_export]
macro_rules! map {
    ($(#[$doc:meta])* $name:ident, $addr:expr) => {
        $(#[$doc])*
        pub(crate) struct $name;

        impl $name {
            pub(crate) const fn get_addr() -> u8 {
                $addr
            }
            pub(crate) fn read<L: $crate::Layer>(transport_layer: &mut L) -> $crate::Result<u8> {
                transport_layer.read_byte(Self::get_addr())
            }
            // TODO: This should be conditional. Not all registers are writable.
            pub(crate) fn write<L: $crate::Layer>(transport_layer: &mut L, data: u8) -> $crate::Result<()> {
                transport_layer.write_byte(Self::get_addr(), data)
            }
        }
    };
}

/// read-only register get method. cached
#[macro_export]
macro_rules! once_ro {
    ($(#[$doc:meta])* $name:ident) => {
        $crate::reg_ro!($name, u8);

        paste::paste! {
            $(#[$doc])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<u8> {
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
    ($(#[$doc:meta])* $name:ident, $ty:ty) => {
        $crate::reg_ro!($name, u8);

        paste::paste! {
            $(#[$doc])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<$ty> {
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
    ($(#[$doc:meta])* $name:ident, $ty:ty, [$msb:ident, $lsb:ident]) => {
        $crate::reg_ro!($msb, u8);
        $crate::reg_ro!($lsb, u8);

        paste::paste! {
            $(#[$doc])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<$ty> {
                match self.$name {
                    Some(v) => Ok(v),
                    None => {
                        let msb = self.[<read_ $msb>]()?;
                        let lsb = self.[<read_ $lsb>]()?;
                        let v = u16::from_le_bytes([lsb, msb]) as $ty;

                        self.$name = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
    ($(#[$doc:meta])* $name:ident, $ty:ty, [$msb:ident, $lsb:ident], |$m:ident, $l:ident| $result:expr) => {
        $crate::reg_ro!($msb, u8);
        $crate::reg_ro!($lsb, u8);

        paste::paste! {
            $(#[$doc])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<$ty> {
                match self.$name {
                    Some(v) => Ok(v),
                    None => {
                        let $m = self.[<read_ $msb>]()?;
                        let $l = self.[<read_ $lsb>]()?;
                        let v = $result as $ty;

                        self.$name = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! bit_pack {
    ($msb:ident, $lsb:ident $(, $ty:ty)?) => {
        u16::from_le_bytes([$lsb, $msb]) $(as $ty)?
    };
    ($msb:ident, $lsb:ident, $xlsb:ident $(, $ty:ty)?) => {
        u32::from_le_bytes([$xlsb, $lsb, $msb, 0]) $(as $ty)?
    };
}

#[macro_export]
macro_rules! live_ro {
    ($(#[$attr:meta])* $name:ident, [$msb:ident, $lsb:ident]) => {
        $crate::reg_ro!($msb, u8);
        $crate::reg_ro!($lsb, u8);
        paste::paste! {
            $(#[$attr])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<u16> {
                let msb = self.[<read_ $msb>]()?;
                let lsb = self.[<read_ $lsb>]()?;

                Ok(bit_pack!(msb, lsb))
            }
        }
    };
    ($(#[$attr:meta])* $name:ident, [$msb:ident, $lsb:ident], |$m:ident, $l:ident| $res:expr) => {
        $crate::reg_ro!($msb, u8);
        $crate::reg_ro!($lsb, u8);
        paste::paste! {
            $(#[$attr])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<u16> {
                let $m = self.[<read_ $msb>]()?;
                let $l = self.[<read_ $lsb>]()?;

                Ok($res)
            }
        }
    };
    ($(#[$attr:meta])* $name:ident, [$msb:ident, $lsb:ident, $xlsb:ident], |$m:ident, $l:ident, $x:ident| $res:expr) => {
        $crate::reg_ro!($msb, u8);
        $crate::reg_ro!($lsb, u8);
        $crate::reg_ro!($xlsb, u8);
        paste::paste! {
            $(#[$attr])*
            pub fn [<get_ $name>](&mut self) -> $crate::Result<u32> {
                let $m = self.[<read_ $msb>]()?;
                let $l = self.[<read_ $lsb>]()?;
                let $x = self.[<read_ $xlsb>]()?;

                Ok($res)
            }
        }
    };
}