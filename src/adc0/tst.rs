#[doc = "Register `TST` reader"]
pub type R = crate::R<TstSpec>;
#[doc = "Register `TST` writer"]
pub type W = crate::W<TstSpec>;
#[doc = "Calibration Sample Time Long\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CstLong {
    #[doc = "0: Normal sample time. Minimum sample time of 3 ADCK cycles."]
    CstLong0 = 0,
    #[doc = "1: Increased sample time. 67 ADCK cycles total sample time."]
    CstLong1 = 1,
}
impl From<CstLong> for bool {
    #[inline(always)]
    fn from(variant: CstLong) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CST_LONG` reader - Calibration Sample Time Long"]
pub type CstLongR = crate::BitReader<CstLong>;
impl CstLongR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CstLong {
        match self.bits {
            false => CstLong::CstLong0,
            true => CstLong::CstLong1,
        }
    }
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn is_cst_long_0(&self) -> bool {
        *self == CstLong::CstLong0
    }
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_cst_long_1(&self) -> bool {
        *self == CstLong::CstLong1
    }
}
#[doc = "Field `CST_LONG` writer - Calibration Sample Time Long"]
pub type CstLongW<'a, REG> = crate::BitWriter<'a, REG, CstLong>;
impl<'a, REG> CstLongW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn cst_long_0(self) -> &'a mut crate::W<REG> {
        self.variant(CstLong::CstLong0)
    }
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn cst_long_1(self) -> &'a mut crate::W<REG> {
        self.variant(CstLong::CstLong1)
    }
}
#[doc = "Force M-side positive offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foffm {
    #[doc = "0: Normal operation. No forced offset."]
    Foffm0 = 0,
    #[doc = "1: Test configuration. Forced positive offset on MDAC."]
    Foffm1 = 1,
}
impl From<Foffm> for bool {
    #[inline(always)]
    fn from(variant: Foffm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFFM` reader - Force M-side positive offset"]
pub type FoffmR = crate::BitReader<Foffm>;
impl FoffmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Foffm {
        match self.bits {
            false => Foffm::Foffm0,
            true => Foffm::Foffm1,
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn is_foffm_0(&self) -> bool {
        *self == Foffm::Foffm0
    }
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    #[inline(always)]
    pub fn is_foffm_1(&self) -> bool {
        *self == Foffm::Foffm1
    }
}
#[doc = "Field `FOFFM` writer - Force M-side positive offset"]
pub type FoffmW<'a, REG> = crate::BitWriter<'a, REG, Foffm>;
impl<'a, REG> FoffmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Foffm::Foffm0)
    }
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    #[inline(always)]
    pub fn foffm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Foffm::Foffm1)
    }
}
#[doc = "Force P-side positive offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foffp {
    #[doc = "0: Normal operation. No forced offset."]
    Foffp0 = 0,
    #[doc = "1: Test configuration. Forced positive offset on PDAC."]
    Foffp1 = 1,
}
impl From<Foffp> for bool {
    #[inline(always)]
    fn from(variant: Foffp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFFP` reader - Force P-side positive offset"]
pub type FoffpR = crate::BitReader<Foffp>;
impl FoffpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Foffp {
        match self.bits {
            false => Foffp::Foffp0,
            true => Foffp::Foffp1,
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn is_foffp_0(&self) -> bool {
        *self == Foffp::Foffp0
    }
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    #[inline(always)]
    pub fn is_foffp_1(&self) -> bool {
        *self == Foffp::Foffp1
    }
}
#[doc = "Field `FOFFP` writer - Force P-side positive offset"]
pub type FoffpW<'a, REG> = crate::BitWriter<'a, REG, Foffp>;
impl<'a, REG> FoffpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Foffp::Foffp0)
    }
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    #[inline(always)]
    pub fn foffp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Foffp::Foffp1)
    }
}
#[doc = "Force M-side negative offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foffm2 {
    #[doc = "0: Normal operation. No forced offset."]
    Foffm2_0 = 0,
    #[doc = "1: Test configuration. Forced negative offset on MDAC."]
    Foffm2_1 = 1,
}
impl From<Foffm2> for bool {
    #[inline(always)]
    fn from(variant: Foffm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFFM2` reader - Force M-side negative offset"]
pub type Foffm2R = crate::BitReader<Foffm2>;
impl Foffm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Foffm2 {
        match self.bits {
            false => Foffm2::Foffm2_0,
            true => Foffm2::Foffm2_1,
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn is_foffm2_0(&self) -> bool {
        *self == Foffm2::Foffm2_0
    }
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    #[inline(always)]
    pub fn is_foffm2_1(&self) -> bool {
        *self == Foffm2::Foffm2_1
    }
}
#[doc = "Field `FOFFM2` writer - Force M-side negative offset"]
pub type Foffm2W<'a, REG> = crate::BitWriter<'a, REG, Foffm2>;
impl<'a, REG> Foffm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffm2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Foffm2::Foffm2_0)
    }
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    #[inline(always)]
    pub fn foffm2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Foffm2::Foffm2_1)
    }
}
#[doc = "Force P-side negative offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Foffp2 {
    #[doc = "0: Normal operation. No forced offset."]
    Foffp2_0 = 0,
    #[doc = "1: Test configuration. Forced negative offset on PDAC."]
    Foffp2_1 = 1,
}
impl From<Foffp2> for bool {
    #[inline(always)]
    fn from(variant: Foffp2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOFFP2` reader - Force P-side negative offset"]
pub type Foffp2R = crate::BitReader<Foffp2>;
impl Foffp2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Foffp2 {
        match self.bits {
            false => Foffp2::Foffp2_0,
            true => Foffp2::Foffp2_1,
        }
    }
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn is_foffp2_0(&self) -> bool {
        *self == Foffp2::Foffp2_0
    }
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    #[inline(always)]
    pub fn is_foffp2_1(&self) -> bool {
        *self == Foffp2::Foffp2_1
    }
}
#[doc = "Field `FOFFP2` writer - Force P-side negative offset"]
pub type Foffp2W<'a, REG> = crate::BitWriter<'a, REG, Foffp2>;
impl<'a, REG> Foffp2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffp2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Foffp2::Foffp2_0)
    }
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    #[inline(always)]
    pub fn foffp2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Foffp2::Foffp2_1)
    }
}
#[doc = "Enable test configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testen {
    #[doc = "0: Normal operation. Test configuration not enabled."]
    Testen0 = 0,
    #[doc = "1: Hardware BIST Test in progress."]
    Testen1 = 1,
}
impl From<Testen> for bool {
    #[inline(always)]
    fn from(variant: Testen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTEN` reader - Enable test configuration"]
pub type TestenR = crate::BitReader<Testen>;
impl TestenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testen {
        match self.bits {
            false => Testen::Testen0,
            true => Testen::Testen1,
        }
    }
    #[doc = "Normal operation. Test configuration not enabled."]
    #[inline(always)]
    pub fn is_testen_0(&self) -> bool {
        *self == Testen::Testen0
    }
    #[doc = "Hardware BIST Test in progress."]
    #[inline(always)]
    pub fn is_testen_1(&self) -> bool {
        *self == Testen::Testen1
    }
}
#[doc = "Field `TESTEN` writer - Enable test configuration"]
pub type TestenW<'a, REG> = crate::BitWriter<'a, REG, Testen>;
impl<'a, REG> TestenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. Test configuration not enabled."]
    #[inline(always)]
    pub fn testen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Testen::Testen0)
    }
    #[doc = "Hardware BIST Test in progress."]
    #[inline(always)]
    pub fn testen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Testen::Testen1)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline(always)]
    pub fn cst_long(&self) -> CstLongR {
        CstLongR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline(always)]
    pub fn foffm(&self) -> FoffmR {
        FoffmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline(always)]
    pub fn foffp(&self) -> FoffpR {
        FoffpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline(always)]
    pub fn foffm2(&self) -> Foffm2R {
        Foffm2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline(always)]
    pub fn foffp2(&self) -> Foffp2R {
        Foffp2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline(always)]
    pub fn testen(&self) -> TestenR {
        TestenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline(always)]
    pub fn cst_long(&mut self) -> CstLongW<TstSpec> {
        CstLongW::new(self, 0)
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline(always)]
    pub fn foffm(&mut self) -> FoffmW<TstSpec> {
        FoffmW::new(self, 8)
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline(always)]
    pub fn foffp(&mut self) -> FoffpW<TstSpec> {
        FoffpW::new(self, 9)
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline(always)]
    pub fn foffm2(&mut self) -> Foffm2W<TstSpec> {
        Foffm2W::new(self, 10)
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline(always)]
    pub fn foffp2(&mut self) -> Foffp2W<TstSpec> {
        Foffp2W::new(self, 11)
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline(always)]
    pub fn testen(&mut self) -> TestenW<TstSpec> {
        TestenW::new(self, 23)
    }
}
#[doc = "ADC Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstSpec;
impl crate::RegisterSpec for TstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tst::R`](R) reader structure"]
impl crate::Readable for TstSpec {}
#[doc = "`write(|w| ..)` method takes [`tst::W`](W) writer structure"]
impl crate::Writable for TstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TstSpec {
    const RESET_VALUE: u32 = 0;
}
