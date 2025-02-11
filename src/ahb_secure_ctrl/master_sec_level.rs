#[doc = "Register `MASTER_SEC_LEVEL` reader"]
pub type R = crate::R<MasterSecLevelSpec>;
#[doc = "Register `MASTER_SEC_LEVEL` writer"]
pub type W = crate::W<MasterSecLevelSpec>;
#[doc = "Micro-Cortex M33 (CPU1) Code bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1c {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Cpu1c> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1c {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1c {}
#[doc = "Field `CPU1C` reader - Micro-Cortex M33 (CPU1) Code bus."]
pub type Cpu1cR = crate::FieldReader<Cpu1c>;
impl Cpu1cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1c {
        match self.bits {
            0 => Cpu1c::EnumNsNp,
            1 => Cpu1c::EnumNsP,
            2 => Cpu1c::EnumSNp,
            3 => Cpu1c::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Cpu1c::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Cpu1c::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Cpu1c::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Cpu1c::EnumSP
    }
}
#[doc = "Field `CPU1C` writer - Micro-Cortex M33 (CPU1) Code bus."]
pub type Cpu1cW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu1c, crate::Safe>;
impl<'a, REG> Cpu1cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1c::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1c::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1c::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1c::EnumSP)
    }
}
#[doc = "Micro-Cortex M33 (CPU1) System bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1s {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Cpu1s> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1s) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1s {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1s {}
#[doc = "Field `CPU1S` reader - Micro-Cortex M33 (CPU1) System bus."]
pub type Cpu1sR = crate::FieldReader<Cpu1s>;
impl Cpu1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1s {
        match self.bits {
            0 => Cpu1s::EnumNsNp,
            1 => Cpu1s::EnumNsP,
            2 => Cpu1s::EnumSNp,
            3 => Cpu1s::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Cpu1s::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Cpu1s::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Cpu1s::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Cpu1s::EnumSP
    }
}
#[doc = "Field `CPU1S` writer - Micro-Cortex M33 (CPU1) System bus."]
pub type Cpu1sW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu1s, crate::Safe>;
impl<'a, REG> Cpu1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1s::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1s::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1s::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1s::EnumSP)
    }
}
#[doc = "USB Full Speed Device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbfsd {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Usbfsd> for u8 {
    #[inline(always)]
    fn from(variant: Usbfsd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbfsd {
    type Ux = u8;
}
impl crate::IsEnum for Usbfsd {}
#[doc = "Field `USBFSD` reader - USB Full Speed Device."]
pub type UsbfsdR = crate::FieldReader<Usbfsd>;
impl UsbfsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfsd {
        match self.bits {
            0 => Usbfsd::EnumNsNp,
            1 => Usbfsd::EnumNsP,
            2 => Usbfsd::EnumSNp,
            3 => Usbfsd::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Usbfsd::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Usbfsd::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Usbfsd::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Usbfsd::EnumSP
    }
}
#[doc = "Field `USBFSD` writer - USB Full Speed Device."]
pub type UsbfsdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbfsd, crate::Safe>;
impl<'a, REG> UsbfsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsd::EnumSP)
    }
}
#[doc = "System DMA 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdma0 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Sdma0> for u8 {
    #[inline(always)]
    fn from(variant: Sdma0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdma0 {
    type Ux = u8;
}
impl crate::IsEnum for Sdma0 {}
#[doc = "Field `SDMA0` reader - System DMA 0."]
pub type Sdma0R = crate::FieldReader<Sdma0>;
impl Sdma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma0 {
        match self.bits {
            0 => Sdma0::EnumNsNp,
            1 => Sdma0::EnumNsP,
            2 => Sdma0::EnumSNp,
            3 => Sdma0::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdma0::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdma0::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdma0::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdma0::EnumSP
    }
}
#[doc = "Field `SDMA0` writer - System DMA 0."]
pub type Sdma0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdma0, crate::Safe>;
impl<'a, REG> Sdma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma0::EnumSP)
    }
}
#[doc = "SDIO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdio {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Sdio> for u8 {
    #[inline(always)]
    fn from(variant: Sdio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdio {
    type Ux = u8;
}
impl crate::IsEnum for Sdio {}
#[doc = "Field `SDIO` reader - SDIO."]
pub type SdioR = crate::FieldReader<Sdio>;
impl SdioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdio {
        match self.bits {
            0 => Sdio::EnumNsNp,
            1 => Sdio::EnumNsP,
            2 => Sdio::EnumSNp,
            3 => Sdio::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdio::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdio::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdio::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdio::EnumSP
    }
}
#[doc = "Field `SDIO` writer - SDIO."]
pub type SdioW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdio, crate::Safe>;
impl<'a, REG> SdioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdio::EnumSP)
    }
}
#[doc = "Power Quad.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pq {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Pq> for u8 {
    #[inline(always)]
    fn from(variant: Pq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pq {
    type Ux = u8;
}
impl crate::IsEnum for Pq {}
#[doc = "Field `PQ` reader - Power Quad."]
pub type PqR = crate::FieldReader<Pq>;
impl PqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pq {
        match self.bits {
            0 => Pq::EnumNsNp,
            1 => Pq::EnumNsP,
            2 => Pq::EnumSNp,
            3 => Pq::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Pq::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Pq::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Pq::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Pq::EnumSP
    }
}
#[doc = "Field `PQ` writer - Power Quad."]
pub type PqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pq, crate::Safe>;
impl<'a, REG> PqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Pq::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Pq::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Pq::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Pq::EnumSP)
    }
}
#[doc = "Hash.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hash {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Hash> for u8 {
    #[inline(always)]
    fn from(variant: Hash) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hash {
    type Ux = u8;
}
impl crate::IsEnum for Hash {}
#[doc = "Field `HASH` reader - Hash."]
pub type HashR = crate::FieldReader<Hash>;
impl HashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hash {
        match self.bits {
            0 => Hash::EnumNsNp,
            1 => Hash::EnumNsP,
            2 => Hash::EnumSNp,
            3 => Hash::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Hash::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Hash::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Hash::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Hash::EnumSP
    }
}
#[doc = "Field `HASH` writer - Hash."]
pub type HashW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hash, crate::Safe>;
impl<'a, REG> HashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Hash::EnumSP)
    }
}
#[doc = "USB Full speed Host.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbfsh {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Usbfsh> for u8 {
    #[inline(always)]
    fn from(variant: Usbfsh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbfsh {
    type Ux = u8;
}
impl crate::IsEnum for Usbfsh {}
#[doc = "Field `USBFSH` reader - USB Full speed Host."]
pub type UsbfshR = crate::FieldReader<Usbfsh>;
impl UsbfshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfsh {
        match self.bits {
            0 => Usbfsh::EnumNsNp,
            1 => Usbfsh::EnumNsP,
            2 => Usbfsh::EnumSNp,
            3 => Usbfsh::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Usbfsh::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Usbfsh::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Usbfsh::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Usbfsh::EnumSP
    }
}
#[doc = "Field `USBFSH` writer - USB Full speed Host."]
pub type UsbfshW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbfsh, crate::Safe>;
impl<'a, REG> UsbfshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfsh::EnumSP)
    }
}
#[doc = "System DMA 1 security level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdma1 {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Sdma1> for u8 {
    #[inline(always)]
    fn from(variant: Sdma1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdma1 {
    type Ux = u8;
}
impl crate::IsEnum for Sdma1 {}
#[doc = "Field `SDMA1` reader - System DMA 1 security level."]
pub type Sdma1R = crate::FieldReader<Sdma1>;
impl Sdma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdma1 {
        match self.bits {
            0 => Sdma1::EnumNsNp,
            1 => Sdma1::EnumNsP,
            2 => Sdma1::EnumSNp,
            3 => Sdma1::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Sdma1::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Sdma1::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Sdma1::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Sdma1::EnumSP
    }
}
#[doc = "Field `SDMA1` writer - System DMA 1 security level."]
pub type Sdma1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdma1, crate::Safe>;
impl<'a, REG> Sdma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Sdma1::EnumSP)
    }
}
#[doc = "MASTER_SEC_LEVEL write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MasterSecLevelLock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<MasterSecLevelLock> for u8 {
    #[inline(always)]
    fn from(variant: MasterSecLevelLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MasterSecLevelLock {
    type Ux = u8;
}
impl crate::IsEnum for MasterSecLevelLock {}
#[doc = "Field `MASTER_SEC_LEVEL_LOCK` reader - MASTER_SEC_LEVEL write-lock."]
pub type MasterSecLevelLockR = crate::FieldReader<MasterSecLevelLock>;
impl MasterSecLevelLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MasterSecLevelLock> {
        match self.bits {
            1 => Some(MasterSecLevelLock::Blocked),
            2 => Some(MasterSecLevelLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == MasterSecLevelLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == MasterSecLevelLock::Writable
    }
}
#[doc = "Field `MASTER_SEC_LEVEL_LOCK` writer - MASTER_SEC_LEVEL write-lock."]
pub type MasterSecLevelLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, MasterSecLevelLock>;
impl<'a, REG> MasterSecLevelLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSecLevelLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(MasterSecLevelLock::Writable)
    }
}
impl R {
    #[doc = "Bits 4:5 - Micro-Cortex M33 (CPU1) Code bus."]
    #[inline(always)]
    pub fn cpu1c(&self) -> Cpu1cR {
        Cpu1cR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Micro-Cortex M33 (CPU1) System bus."]
    #[inline(always)]
    pub fn cpu1s(&self) -> Cpu1sR {
        Cpu1sR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - USB Full Speed Device."]
    #[inline(always)]
    pub fn usbfsd(&self) -> UsbfsdR {
        UsbfsdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - System DMA 0."]
    #[inline(always)]
    pub fn sdma0(&self) -> Sdma0R {
        Sdma0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn sdio(&self) -> SdioR {
        SdioR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Power Quad."]
    #[inline(always)]
    pub fn pq(&self) -> PqR {
        PqR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Hash."]
    #[inline(always)]
    pub fn hash(&self) -> HashR {
        HashR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - USB Full speed Host."]
    #[inline(always)]
    pub fn usbfsh(&self) -> UsbfshR {
        UsbfshR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level."]
    #[inline(always)]
    pub fn sdma1(&self) -> Sdma1R {
        Sdma1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_LEVEL write-lock."]
    #[inline(always)]
    pub fn master_sec_level_lock(&self) -> MasterSecLevelLockR {
        MasterSecLevelLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Micro-Cortex M33 (CPU1) Code bus."]
    #[inline(always)]
    pub fn cpu1c(&mut self) -> Cpu1cW<MasterSecLevelSpec> {
        Cpu1cW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Micro-Cortex M33 (CPU1) System bus."]
    #[inline(always)]
    pub fn cpu1s(&mut self) -> Cpu1sW<MasterSecLevelSpec> {
        Cpu1sW::new(self, 6)
    }
    #[doc = "Bits 8:9 - USB Full Speed Device."]
    #[inline(always)]
    pub fn usbfsd(&mut self) -> UsbfsdW<MasterSecLevelSpec> {
        UsbfsdW::new(self, 8)
    }
    #[doc = "Bits 10:11 - System DMA 0."]
    #[inline(always)]
    pub fn sdma0(&mut self) -> Sdma0W<MasterSecLevelSpec> {
        Sdma0W::new(self, 10)
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn sdio(&mut self) -> SdioW<MasterSecLevelSpec> {
        SdioW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Power Quad."]
    #[inline(always)]
    pub fn pq(&mut self) -> PqW<MasterSecLevelSpec> {
        PqW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Hash."]
    #[inline(always)]
    pub fn hash(&mut self) -> HashW<MasterSecLevelSpec> {
        HashW::new(self, 20)
    }
    #[doc = "Bits 22:23 - USB Full speed Host."]
    #[inline(always)]
    pub fn usbfsh(&mut self) -> UsbfshW<MasterSecLevelSpec> {
        UsbfshW::new(self, 22)
    }
    #[doc = "Bits 24:25 - System DMA 1 security level."]
    #[inline(always)]
    pub fn sdma1(&mut self) -> Sdma1W<MasterSecLevelSpec> {
        Sdma1W::new(self, 24)
    }
    #[doc = "Bits 30:31 - MASTER_SEC_LEVEL write-lock."]
    #[inline(always)]
    pub fn master_sec_level_lock(&mut self) -> MasterSecLevelLockW<MasterSecLevelSpec> {
        MasterSecLevelLockW::new(self, 30)
    }
}
#[doc = "master secure level register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_sec_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_sec_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterSecLevelSpec;
impl crate::RegisterSpec for MasterSecLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_sec_level::R`](R) reader structure"]
impl crate::Readable for MasterSecLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`master_sec_level::W`](W) writer structure"]
impl crate::Writable for MasterSecLevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASTER_SEC_LEVEL to value 0x8000_0000"]
impl crate::Resettable for MasterSecLevelSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
