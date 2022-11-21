#[allow(unused, dead_code)]
pub trait ForceConvert {
    /// Default implementation
    /// 
    /// bytes are usually `&[u8]` or `[u8]`
    fn force_into<T>(self) -> T
    where
        T: TryFrom<Self> + Default,
        Self: TryInto<T>
    {
        self.try_into().unwrap_or_default()
    }
}
