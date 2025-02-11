#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PwrctrlSpec>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PwrctrlSpec>;
#[doc = "Field `RAMON` reader - Power on the PUF RAM."]
pub type RamonR = crate::BitReader;
#[doc = "Field `RAMON` writer - Power on the PUF RAM."]
pub type RamonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMSTAT` reader - PUF RAM status."]
pub type RamstatR = crate::BitReader;
#[doc = "Field `RAMSTAT` writer - PUF RAM status."]
pub type RamstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&self) -> RamonR {
        RamonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&self) -> RamstatR {
        RamstatR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on the PUF RAM."]
    #[inline(always)]
    pub fn ramon(&mut self) -> RamonW<PwrctrlSpec> {
        RamonW::new(self, 0)
    }
    #[doc = "Bit 1 - PUF RAM status."]
    #[inline(always)]
    pub fn ramstat(&mut self) -> RamstatW<PwrctrlSpec> {
        RamstatW::new(self, 1)
    }
}
#[doc = "PUF RAM Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctrlSpec;
impl crate::RegisterSpec for PwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0xf8"]
impl crate::Resettable for PwrctrlSpec {
    const RESET_VALUE: u32 = 0xf8;
}
