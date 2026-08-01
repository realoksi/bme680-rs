/// Maps a physical register to a new `struct`.
macro_rules! physical {
    ($name:ident i2c($i2c:expr) spi($spi:expr, $page:expr) ) => {
        paste::paste! {
            pub(crate) struct [<$name:camel>];
            impl Register for [<$name:camel>] {
                const I2C_ADDR: u8 = $i2c;
                const SPI_ADDR: u8 = $spi;
                const PAGE: u8 = $page;
            }
        }
    };
}

pub(crate) use physical;

/// Maps read-only get methods to logical registers.
macro_rules! logical_ro {
    (once: $name:ident $ty:ty) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                match self.[<$name:snake>] {
                    Some(v) => Ok(v),
                    None => {
                        let v = self.bus.read::<[<$name:camel>]>()? as $ty;
                        self.[<$name:snake>] = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
    (once: $name:ident $ty:ty [$msb:ident, $lsb:ident]) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                match self.[<$name:snake>] {
                    Some(v) => Ok(v),
                    None => {
                        let msb = self.bus.read::<[<$msb:camel>]>()?;
                        let lsb = self.bus.read::<[<$lsb:camel>]>()?;
                        let v = u16::from_le_bytes([lsb, msb]) as $ty;
                        self.[<$name:snake>] = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
    (once: $name:ident $ty:ty [$msb:ident, $lsb:ident] |$m:ident, $l:ident| $result:expr) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                match self.[<$name:snake>] {
                    Some(v) => Ok(v),
                    None => {
                        let $m = self.bus.read::<[<$msb:camel>]>()?;
                        let $l = self.bus.read::<[<$lsb:camel>]>()?;
                        let v = $result as $ty;
                        self.[<$name:snake>] = Some(v);
                        Ok(v)
                    }
                }
            }
        }
    };
    (always: $name:ident $ty:ty) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                let v = self.bus.read::<[<$name:camel>]>()?;
                Ok(v as $ty)
            }
        }
    };
    (always: $name:ident $ty:ty |$v:ident| $result:expr) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                let $v = self.bus.read::<[<$name:camel>]>()?;
                Ok($result as $ty)
            }
        }
    };
    (always: $name:ident $ty:ty [$msb:ident, $lsb:ident]) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                let msb = self.bus.read::<[<$msb:camel>]>()?;
                let lsb = self.bus.read::<[<$lsb:camel>]>()?;
                Ok(u16::from_le_bytes([lsb, msb]) as $ty)
            }
        }
    };
    (always: $name:ident $ty:ty [$msb:ident, $lsb:ident] |$m:ident, $l:ident| $result:expr) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                let $m = self.bus.read::<[<$msb:camel>]>()?;
                let $l = self.bus.read::<[<$lsb:camel>]>()?;
                Ok($result as $ty)
            }
        }
    };
    (always: $name:ident $ty:ty [$msb:ident, $lsb:ident, $xlsb:ident] |$m:ident, $l:ident, $x:ident| $result:expr) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                let $m = self.bus.read::<[<$msb:camel>]>()?;
                let $l = self.bus.read::<[<$lsb:camel>]>()?;
                let $x = self.bus.read::<[<$xlsb:camel>]>()?;

                Ok($result)
            }
        }
    };
}

pub(crate) use logical_ro;

/// Maps read-write get and set methods to logical registers.
macro_rules! logical_rw {
    (always: $name:ident $ty:ty) => {
        paste::paste! {
            #[doc = "Gets `" $name "`."]
            pub fn [<get_ $name:snake>](&mut self) -> Result<$ty> {
                self.bus.read::<[<$name:camel>]>()
            }
            #[doc = "Sets `" $name "`."]
            pub fn [<set_ $name:snake>](&mut self, value: $ty) -> Result<()> {
                self.bus.write::<[<$name:camel>]>(value as u8)
            }
        }
    };
}

pub(crate) use logical_rw;
