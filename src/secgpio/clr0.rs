#[doc = "Register `CLR0` writer"]
pub type W = crate::W<Clr0Spec>;
#[doc = "Field `CLRP` writer - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
pub type ClrpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp(&mut self) -> ClrpW<Clr0Spec> {
        ClrpW::new(self, 0)
    }
}
#[doc = "Clear port for all port GPIO pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clr0Spec;
impl crate::RegisterSpec for Clr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr0::W`](W) writer structure"]
impl crate::Writable for Clr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR0 to value 0"]
impl crate::Resettable for Clr0Spec {
    const RESET_VALUE: u32 = 0;
}
