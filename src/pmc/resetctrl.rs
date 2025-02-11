#[doc = "Register `RESETCTRL` reader"]
pub type R = crate::R<ResetctrlSpec>;
#[doc = "Register `RESETCTRL` writer"]
pub type W = crate::W<ResetctrlSpec>;
#[doc = "Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dpdwakeupresetenable {
    #[doc = "0: Reset event from DEEP POWER DOWN mode is disable."]
    Disable = 0,
    #[doc = "1: Reset event from DEEP POWER DOWN mode is enable."]
    Enable = 1,
}
impl From<Dpdwakeupresetenable> for bool {
    #[inline(always)]
    fn from(variant: Dpdwakeupresetenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` reader - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DpdwakeupresetenableR = crate::BitReader<Dpdwakeupresetenable>;
impl DpdwakeupresetenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dpdwakeupresetenable {
        match self.bits {
            false => Dpdwakeupresetenable::Disable,
            true => Dpdwakeupresetenable::Enable,
        }
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dpdwakeupresetenable::Disable
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dpdwakeupresetenable::Enable
    }
}
#[doc = "Field `DPDWAKEUPRESETENABLE` writer - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
pub type DpdwakeupresetenableW<'a, REG> = crate::BitWriter<'a, REG, Dpdwakeupresetenable>;
impl<'a, REG> DpdwakeupresetenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset event from DEEP POWER DOWN mode is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dpdwakeupresetenable::Disable)
    }
    #[doc = "Reset event from DEEP POWER DOWN mode is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dpdwakeupresetenable::Enable)
    }
}
#[doc = "BOD VBAT reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodvbatresetenable {
    #[doc = "0: BOD VBAT reset is disable."]
    Disable = 0,
    #[doc = "1: BOD VBAT reset is enable."]
    Enable = 1,
}
impl From<Bodvbatresetenable> for bool {
    #[inline(always)]
    fn from(variant: Bodvbatresetenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODVBATRESETENABLE` reader - BOD VBAT reset enable."]
pub type BodvbatresetenableR = crate::BitReader<Bodvbatresetenable>;
impl BodvbatresetenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodvbatresetenable {
        match self.bits {
            false => Bodvbatresetenable::Disable,
            true => Bodvbatresetenable::Enable,
        }
    }
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bodvbatresetenable::Disable
    }
    #[doc = "BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bodvbatresetenable::Enable
    }
}
#[doc = "Field `BODVBATRESETENABLE` writer - BOD VBAT reset enable."]
pub type BodvbatresetenableW<'a, REG> = crate::BitWriter<'a, REG, Bodvbatresetenable>;
impl<'a, REG> BodvbatresetenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOD VBAT reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodvbatresetenable::Disable)
    }
    #[doc = "BOD VBAT reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodvbatresetenable::Enable)
    }
}
#[doc = "BOD CORE reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bodcoreresetenable {
    #[doc = "0: BOD CORE reset is disable."]
    Disable = 0,
    #[doc = "1: BOD CORE reset is enable."]
    Enable = 1,
}
impl From<Bodcoreresetenable> for bool {
    #[inline(always)]
    fn from(variant: Bodcoreresetenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODCORERESETENABLE` reader - BOD CORE reset enable."]
pub type BodcoreresetenableR = crate::BitReader<Bodcoreresetenable>;
impl BodcoreresetenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bodcoreresetenable {
        match self.bits {
            false => Bodcoreresetenable::Disable,
            true => Bodcoreresetenable::Enable,
        }
    }
    #[doc = "BOD CORE reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bodcoreresetenable::Disable
    }
    #[doc = "BOD CORE reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bodcoreresetenable::Enable
    }
}
#[doc = "Field `BODCORERESETENABLE` writer - BOD CORE reset enable."]
pub type BodcoreresetenableW<'a, REG> = crate::BitWriter<'a, REG, Bodcoreresetenable>;
impl<'a, REG> BodcoreresetenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BOD CORE reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodcoreresetenable::Disable)
    }
    #[doc = "BOD CORE reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bodcoreresetenable::Enable)
    }
}
#[doc = "Software reset enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrresetenable {
    #[doc = "0: Software reset is disable."]
    Disable = 0,
    #[doc = "1: Software reset is enable."]
    Enable = 1,
}
impl From<Swrresetenable> for bool {
    #[inline(always)]
    fn from(variant: Swrresetenable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRRESETENABLE` reader - Software reset enable."]
pub type SwrresetenableR = crate::BitReader<Swrresetenable>;
impl SwrresetenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrresetenable {
        match self.bits {
            false => Swrresetenable::Disable,
            true => Swrresetenable::Enable,
        }
    }
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Swrresetenable::Disable
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Swrresetenable::Enable
    }
}
#[doc = "Field `SWRRESETENABLE` writer - Software reset enable."]
pub type SwrresetenableW<'a, REG> = crate::BitWriter<'a, REG, Swrresetenable>;
impl<'a, REG> SwrresetenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software reset is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Swrresetenable::Disable)
    }
    #[doc = "Software reset is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Swrresetenable::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&self) -> DpdwakeupresetenableR {
        DpdwakeupresetenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetenable(&self) -> BodvbatresetenableR {
        BodvbatresetenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOD CORE reset enable."]
    #[inline(always)]
    pub fn bodcoreresetenable(&self) -> BodcoreresetenableR {
        BodcoreresetenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&self) -> SwrresetenableR {
        SwrresetenableR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up from DEEP POWER DOWN reset event (either from wake up I/O or RTC or OS Event Timer)."]
    #[inline(always)]
    pub fn dpdwakeupresetenable(&mut self) -> DpdwakeupresetenableW<ResetctrlSpec> {
        DpdwakeupresetenableW::new(self, 0)
    }
    #[doc = "Bit 1 - BOD VBAT reset enable."]
    #[inline(always)]
    pub fn bodvbatresetenable(&mut self) -> BodvbatresetenableW<ResetctrlSpec> {
        BodvbatresetenableW::new(self, 1)
    }
    #[doc = "Bit 2 - BOD CORE reset enable."]
    #[inline(always)]
    pub fn bodcoreresetenable(&mut self) -> BodcoreresetenableW<ResetctrlSpec> {
        BodcoreresetenableW::new(self, 2)
    }
    #[doc = "Bit 3 - Software reset enable."]
    #[inline(always)]
    pub fn swrresetenable(&mut self) -> SwrresetenableW<ResetctrlSpec> {
        SwrresetenableW::new(self, 3)
    }
}
#[doc = "Reset Control \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`resetctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetctrlSpec;
impl crate::RegisterSpec for ResetctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetctrl::R`](R) reader structure"]
impl crate::Readable for ResetctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`resetctrl::W`](W) writer structure"]
impl crate::Writable for ResetctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETCTRL to value 0"]
impl crate::Resettable for ResetctrlSpec {
    const RESET_VALUE: u32 = 0;
}
