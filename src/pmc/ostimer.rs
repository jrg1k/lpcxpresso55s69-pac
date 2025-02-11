#[doc = "Register `OSTIMER` reader"]
pub type R = crate::R<OstimerSpec>;
#[doc = "Register `OSTIMER` writer"]
pub type W = crate::W<OstimerSpec>;
#[doc = "Field `SOFTRESET` reader - Active high reset."]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - Active high reset."]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCKENABLE` reader - Enable OSTIMER 32 KHz clock."]
pub type ClockenableR = crate::BitReader;
#[doc = "Field `CLOCKENABLE` writer - Enable OSTIMER 32 KHz clock."]
pub type ClockenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPDWAKEUPENABLE` reader - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DpdwakeupenableR = crate::BitReader;
#[doc = "Field `DPDWAKEUPENABLE` writer - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
pub type DpdwakeupenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32KPD` reader - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type Osc32kpdR = crate::BitReader;
#[doc = "Field `OSC32KPD` writer - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
pub type Osc32kpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&self) -> ClockenableR {
        ClockenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&self) -> DpdwakeupenableR {
        DpdwakeupenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&self) -> Osc32kpdR {
        Osc32kpdR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high reset."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SoftresetW<OstimerSpec> {
        SoftresetW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable OSTIMER 32 KHz clock."]
    #[inline(always)]
    pub fn clockenable(&mut self) -> ClockenableW<OstimerSpec> {
        ClockenableW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake up enable in Deep Power Down mode (To be used in Enable Deep Power Down mode)."]
    #[inline(always)]
    pub fn dpdwakeupenable(&mut self) -> DpdwakeupenableW<OstimerSpec> {
        DpdwakeupenableW::new(self, 2)
    }
    #[doc = "Bit 3 - Oscilator 32KHz (either FRO32KHz or XTAL32KHz according to RTCOSC32K."]
    #[inline(always)]
    pub fn osc32kpd(&mut self) -> Osc32kpdW<OstimerSpec> {
        Osc32kpdW::new(self, 3)
    }
}
#[doc = "OS Timer control register \\[Reset by: PoR, Brown Out Detectors Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`ostimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OstimerSpec;
impl crate::RegisterSpec for OstimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ostimer::R`](R) reader structure"]
impl crate::Readable for OstimerSpec {}
#[doc = "`write(|w| ..)` method takes [`ostimer::W`](W) writer structure"]
impl crate::Writable for OstimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSTIMER to value 0x08"]
impl crate::Resettable for OstimerSpec {
    const RESET_VALUE: u32 = 0x08;
}
