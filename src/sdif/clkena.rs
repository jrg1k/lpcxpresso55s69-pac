#[doc = "Register `CLKENA` reader"]
pub type R = crate::R<ClkenaSpec>;
#[doc = "Register `CLKENA` writer"]
pub type W = crate::W<ClkenaSpec>;
#[doc = "Field `CCLK0_ENABLE` reader - Clock-enable control for SD card 0 clock."]
pub type Cclk0EnableR = crate::BitReader;
#[doc = "Field `CCLK0_ENABLE` writer - Clock-enable control for SD card 0 clock."]
pub type Cclk0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCLK1_ENABLE` reader - Clock-enable control for SD card 1 clock."]
pub type Cclk1EnableR = crate::BitReader;
#[doc = "Field `CCLK1_ENABLE` writer - Clock-enable control for SD card 1 clock."]
pub type Cclk1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCLK0_LOW_POWER` reader - Low-power control for SD card 0 clock."]
pub type Cclk0LowPowerR = crate::BitReader;
#[doc = "Field `CCLK0_LOW_POWER` writer - Low-power control for SD card 0 clock."]
pub type Cclk0LowPowerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCLK1_LOW_POWER` reader - Low-power control for SD card 1 clock."]
pub type Cclk1LowPowerR = crate::BitReader;
#[doc = "Field `CCLK1_LOW_POWER` writer - Low-power control for SD card 1 clock."]
pub type Cclk1LowPowerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&self) -> Cclk0EnableR {
        Cclk0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&self) -> Cclk1EnableR {
        Cclk1EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&self) -> Cclk0LowPowerR {
        Cclk0LowPowerR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&self) -> Cclk1LowPowerR {
        Cclk1LowPowerR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock-enable control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_enable(&mut self) -> Cclk0EnableW<ClkenaSpec> {
        Cclk0EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock-enable control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_enable(&mut self) -> Cclk1EnableW<ClkenaSpec> {
        Cclk1EnableW::new(self, 1)
    }
    #[doc = "Bit 16 - Low-power control for SD card 0 clock."]
    #[inline(always)]
    pub fn cclk0_low_power(&mut self) -> Cclk0LowPowerW<ClkenaSpec> {
        Cclk0LowPowerW::new(self, 16)
    }
    #[doc = "Bit 17 - Low-power control for SD card 1 clock."]
    #[inline(always)]
    pub fn cclk1_low_power(&mut self) -> Cclk1LowPowerW<ClkenaSpec> {
        Cclk1LowPowerW::new(self, 17)
    }
}
#[doc = "Clock Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkenaSpec;
impl crate::RegisterSpec for ClkenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkena::R`](R) reader structure"]
impl crate::Readable for ClkenaSpec {}
#[doc = "`write(|w| ..)` method takes [`clkena::W`](W) writer structure"]
impl crate::Writable for ClkenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKENA to value 0"]
impl crate::Resettable for ClkenaSpec {
    const RESET_VALUE: u32 = 0;
}
