#[doc = "Register `CC_SOCU_DFLT` reader"]
pub type R = crate::R<CcSocuDfltSpec>;
#[doc = "Register `CC_SOCU_DFLT` writer"]
pub type W = crate::W<CcSocuDfltSpec>;
#[doc = "Non Secure non-invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Niden> for bool {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` reader - Non Secure non-invasive debug fixed state"]
pub type NidenR = crate::BitReader<Niden>;
impl NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Niden {
        match self.bits {
            false => Niden::Disable,
            true => Niden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Niden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Niden::Enable
    }
}
#[doc = "Field `NIDEN` writer - Non Secure non-invasive debug fixed state"]
pub type NidenW<'a, REG> = crate::BitWriter<'a, REG, Niden>;
impl<'a, REG> NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Enable)
    }
}
#[doc = "Non Secure debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Non Secure debug fixed state"]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::Disable,
            true => Dbgen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbgen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbgen::Enable
    }
}
#[doc = "Field `DBGEN` writer - Non Secure debug fixed state"]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Enable)
    }
}
#[doc = "Secure non-invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spniden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Spniden> for bool {
    #[inline(always)]
    fn from(variant: Spniden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPNIDEN` reader - Secure non-invasive debug fixed state"]
pub type SpnidenR = crate::BitReader<Spniden>;
impl SpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spniden {
        match self.bits {
            false => Spniden::Disable,
            true => Spniden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spniden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spniden::Enable
    }
}
#[doc = "Field `SPNIDEN` writer - Secure non-invasive debug fixed state"]
pub type SpnidenW<'a, REG> = crate::BitWriter<'a, REG, Spniden>;
impl<'a, REG> SpnidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spniden::Enable)
    }
}
#[doc = "Secure invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Spiden> for bool {
    #[inline(always)]
    fn from(variant: Spiden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDEN` reader - Secure invasive debug fixed state"]
pub type SpidenR = crate::BitReader<Spiden>;
impl SpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiden {
        match self.bits {
            false => Spiden::Disable,
            true => Spiden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spiden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spiden::Enable
    }
}
#[doc = "Field `SPIDEN` writer - Secure invasive debug fixed state"]
pub type SpidenW<'a, REG> = crate::BitWriter<'a, REG, Spiden>;
impl<'a, REG> SpidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spiden::Enable)
    }
}
#[doc = "JTAG TAP fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tapen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Tapen> for bool {
    #[inline(always)]
    fn from(variant: Tapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPEN` reader - JTAG TAP fixed state"]
pub type TapenR = crate::BitReader<Tapen>;
impl TapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tapen {
        match self.bits {
            false => Tapen::Disable,
            true => Tapen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tapen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tapen::Enable
    }
}
#[doc = "Field `TAPEN` writer - JTAG TAP fixed state"]
pub type TapenW<'a, REG> = crate::BitWriter<'a, REG, Tapen>;
impl<'a, REG> TapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tapen::Enable)
    }
}
#[doc = "CPU1 (Micro cortex M33) invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1Dbgen {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cpu1Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Cpu1Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_DBGEN` reader - CPU1 (Micro cortex M33) invasive debug fixed state"]
pub type Cpu1DbgenR = crate::BitReader<Cpu1Dbgen>;
impl Cpu1DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1Dbgen {
        match self.bits {
            false => Cpu1Dbgen::Disable,
            true => Cpu1Dbgen::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu1Dbgen::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1Dbgen::Enable
    }
}
#[doc = "Field `CPU1_DBGEN` writer - CPU1 (Micro cortex M33) invasive debug fixed state"]
pub type Cpu1DbgenW<'a, REG> = crate::BitWriter<'a, REG, Cpu1Dbgen>;
impl<'a, REG> Cpu1DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Dbgen::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Dbgen::Enable)
    }
}
#[doc = "ISP Boot Command fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspCmdEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<IspCmdEn> for bool {
    #[inline(always)]
    fn from(variant: IspCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_CMD_EN` reader - ISP Boot Command fixed state"]
pub type IspCmdEnR = crate::BitReader<IspCmdEn>;
impl IspCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspCmdEn {
        match self.bits {
            false => IspCmdEn::Disable,
            true => IspCmdEn::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IspCmdEn::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IspCmdEn::Enable
    }
}
#[doc = "Field `ISP_CMD_EN` writer - ISP Boot Command fixed state"]
pub type IspCmdEnW<'a, REG> = crate::BitWriter<'a, REG, IspCmdEn>;
impl<'a, REG> IspCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IspCmdEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IspCmdEn::Enable)
    }
}
#[doc = "FA Command fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaCmdEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<FaCmdEn> for bool {
    #[inline(always)]
    fn from(variant: FaCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FA_CMD_EN` reader - FA Command fixed state"]
pub type FaCmdEnR = crate::BitReader<FaCmdEn>;
impl FaCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FaCmdEn {
        match self.bits {
            false => FaCmdEn::Disable,
            true => FaCmdEn::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FaCmdEn::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FaCmdEn::Enable
    }
}
#[doc = "Field `FA_CMD_EN` writer - FA Command fixed state"]
pub type FaCmdEnW<'a, REG> = crate::BitWriter<'a, REG, FaCmdEn>;
impl<'a, REG> FaCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FaCmdEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FaCmdEn::Enable)
    }
}
#[doc = "Flash Mass Erase Command fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeCmdEn {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<MeCmdEn> for bool {
    #[inline(always)]
    fn from(variant: MeCmdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ME_CMD_EN` reader - Flash Mass Erase Command fixed state"]
pub type MeCmdEnR = crate::BitReader<MeCmdEn>;
impl MeCmdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MeCmdEn {
        match self.bits {
            false => MeCmdEn::Disable,
            true => MeCmdEn::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MeCmdEn::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MeCmdEn::Enable
    }
}
#[doc = "Field `ME_CMD_EN` writer - Flash Mass Erase Command fixed state"]
pub type MeCmdEnW<'a, REG> = crate::BitWriter<'a, REG, MeCmdEn>;
impl<'a, REG> MeCmdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MeCmdEn::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MeCmdEn::Enable)
    }
}
#[doc = "CPU1 (Micro cortex M33) non-invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1Niden {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<Cpu1Niden> for bool {
    #[inline(always)]
    fn from(variant: Cpu1Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1_NIDEN` reader - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
pub type Cpu1NidenR = crate::BitReader<Cpu1Niden>;
impl Cpu1NidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1Niden {
        match self.bits {
            false => Cpu1Niden::Disable,
            true => Cpu1Niden::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu1Niden::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1Niden::Enable
    }
}
#[doc = "Field `CPU1_NIDEN` writer - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
pub type Cpu1NidenW<'a, REG> = crate::BitWriter<'a, REG, Cpu1Niden>;
impl<'a, REG> Cpu1NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Niden::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1Niden::Enable)
    }
}
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub type InverseValueR = crate::FieldReader<u16>;
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub type InverseValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn niden(&self) -> NidenR {
        NidenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non Secure debug fixed state"]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn spniden(&self) -> SpnidenR {
        SpnidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure invasive debug fixed state"]
    #[inline(always)]
    pub fn spiden(&self) -> SpidenR {
        SpidenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - JTAG TAP fixed state"]
    #[inline(always)]
    pub fn tapen(&self) -> TapenR {
        TapenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU1 (Micro cortex M33) invasive debug fixed state"]
    #[inline(always)]
    pub fn cpu1_dbgen(&self) -> Cpu1DbgenR {
        Cpu1DbgenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ISP Boot Command fixed state"]
    #[inline(always)]
    pub fn isp_cmd_en(&self) -> IspCmdEnR {
        IspCmdEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FA Command fixed state"]
    #[inline(always)]
    pub fn fa_cmd_en(&self) -> FaCmdEnR {
        FaCmdEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Mass Erase Command fixed state"]
    #[inline(always)]
    pub fn me_cmd_en(&self) -> MeCmdEnR {
        MeCmdEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
    #[inline(always)]
    pub fn cpu1_niden(&self) -> Cpu1NidenR {
        Cpu1NidenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> InverseValueR {
        InverseValueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn niden(&mut self) -> NidenW<CcSocuDfltSpec> {
        NidenW::new(self, 0)
    }
    #[doc = "Bit 1 - Non Secure debug fixed state"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<CcSocuDfltSpec> {
        DbgenW::new(self, 1)
    }
    #[doc = "Bit 2 - Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn spniden(&mut self) -> SpnidenW<CcSocuDfltSpec> {
        SpnidenW::new(self, 2)
    }
    #[doc = "Bit 3 - Secure invasive debug fixed state"]
    #[inline(always)]
    pub fn spiden(&mut self) -> SpidenW<CcSocuDfltSpec> {
        SpidenW::new(self, 3)
    }
    #[doc = "Bit 4 - JTAG TAP fixed state"]
    #[inline(always)]
    pub fn tapen(&mut self) -> TapenW<CcSocuDfltSpec> {
        TapenW::new(self, 4)
    }
    #[doc = "Bit 5 - CPU1 (Micro cortex M33) invasive debug fixed state"]
    #[inline(always)]
    pub fn cpu1_dbgen(&mut self) -> Cpu1DbgenW<CcSocuDfltSpec> {
        Cpu1DbgenW::new(self, 5)
    }
    #[doc = "Bit 6 - ISP Boot Command fixed state"]
    #[inline(always)]
    pub fn isp_cmd_en(&mut self) -> IspCmdEnW<CcSocuDfltSpec> {
        IspCmdEnW::new(self, 6)
    }
    #[doc = "Bit 7 - FA Command fixed state"]
    #[inline(always)]
    pub fn fa_cmd_en(&mut self) -> FaCmdEnW<CcSocuDfltSpec> {
        FaCmdEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Flash Mass Erase Command fixed state"]
    #[inline(always)]
    pub fn me_cmd_en(&mut self) -> MeCmdEnW<CcSocuDfltSpec> {
        MeCmdEnW::new(self, 8)
    }
    #[doc = "Bit 9 - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
    #[inline(always)]
    pub fn cpu1_niden(&mut self) -> Cpu1NidenW<CcSocuDfltSpec> {
        Cpu1NidenW::new(self, 9)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&mut self) -> InverseValueW<CcSocuDfltSpec> {
        InverseValueW::new(self, 16)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_socu_dflt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_socu_dflt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSocuDfltSpec;
impl crate::RegisterSpec for CcSocuDfltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_socu_dflt::R`](R) reader structure"]
impl crate::Readable for CcSocuDfltSpec {}
#[doc = "`write(|w| ..)` method takes [`cc_socu_dflt::W`](W) writer structure"]
impl crate::Writable for CcSocuDfltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_SOCU_DFLT to value 0"]
impl crate::Resettable for CcSocuDfltSpec {
    const RESET_VALUE: u32 = 0;
}
