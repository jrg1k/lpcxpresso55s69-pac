#[doc = "Register `ABORT0` writer"]
pub type W = crate::W<Abort0Spec>;
#[doc = "Field `ABORTCTRL` writer - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
pub type AbortctrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Abort control for DMA channel 0. Bit n corresponds to DMA channel n. 0 = no effect. 1 = aborts DMA operations on channel n."]
    #[inline(always)]
    pub fn abortctrl(&mut self) -> AbortctrlW<Abort0Spec> {
        AbortctrlW::new(self, 0)
    }
}
#[doc = "Channel Abort control for all DMA channels.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abort0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Abort0Spec;
impl crate::RegisterSpec for Abort0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`abort0::W`](W) writer structure"]
impl crate::Writable for Abort0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ABORT0 to value 0"]
impl crate::Resettable for Abort0Spec {
    const RESET_VALUE: u32 = 0;
}
