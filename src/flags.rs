#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct MethodAttributes(pub(crate) u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct TypeAttributes(pub(crate) u32);

impl MethodAttributes {
    pub fn special(&self) -> bool {
        self.0 & 0b1000_0000_0000 != 0
    }
}

impl TypeAttributes {
    pub fn windows_runtime(&self) -> bool {
        self.0 & 0b100_0000_0000_0000 != 0
    }
    pub fn interface(&self) -> bool {
        self.0 & 0b10_0000 != 0
    }
}
