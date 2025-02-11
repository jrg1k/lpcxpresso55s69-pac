#[doc = "Register `CPUCFG` reader"]
pub type R = crate::R<CpucfgSpec>;
#[doc = "Register `CPUCFG` writer"]
pub type W = crate::W<CpucfgSpec>;
#[doc = "Enable CPU1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1enable {
    #[doc = "0: CPU1 is disable (Processor in reset)."]
    Disable = 0,
    #[doc = "1: CPU1 is enable."]
    Enable = 1,
}
impl From<Cpu1enable> for bool {
    #[inline(always)]
    fn from(variant: Cpu1enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1ENABLE` reader - Enable CPU1."]
pub type Cpu1enableR = crate::BitReader<Cpu1enable>;
impl Cpu1enableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1enable {
        match self.bits {
            false => Cpu1enable::Disable,
            true => Cpu1enable::Enable,
        }
    }
    #[doc = "CPU1 is disable (Processor in reset)."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu1enable::Disable
    }
    #[doc = "CPU1 is enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1enable::Enable
    }
}
#[doc = "Field `CPU1ENABLE` writer - Enable CPU1."]
pub type Cpu1enableW<'a, REG> = crate::BitWriter<'a, REG, Cpu1enable>;
impl<'a, REG> Cpu1enableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CPU1 is disable (Processor in reset)."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1enable::Disable)
    }
    #[doc = "CPU1 is enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1enable::Enable)
    }
}
impl R {
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline(always)]
    pub fn cpu1enable(&self) -> Cpu1enableR {
        Cpu1enableR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable CPU1."]
    #[inline(always)]
    pub fn cpu1enable(&mut self) -> Cpu1enableW<CpucfgSpec> {
        Cpu1enableW::new(self, 2)
    }
}
#[doc = "CPUs configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpucfgSpec;
impl crate::RegisterSpec for CpucfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpucfg::R`](R) reader structure"]
impl crate::Readable for CpucfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cpucfg::W`](W) writer structure"]
impl crate::Writable for CpucfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCFG to value 0x02"]
impl crate::Resettable for CpucfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
