#[doc = "Register `INTENSET0` reader"]
pub type R = crate::R<Intenset0Spec>;
#[doc = "Register `INTENSET0` writer"]
pub type W = crate::W<Intenset0Spec>;
#[doc = "Field `INTEN` reader - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
pub type IntenR = crate::FieldReader<u32>;
#[doc = "Field `INTEN` writer - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
pub type IntenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Enable read and set for DMA channel n. Bit n corresponds to DMA channel n. The number of bits = number of DMA channels in this device. Other bits are reserved. 0 = interrupt for DMA channel is disabled. 1 = interrupt for DMA channel is enabled."]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<Intenset0Spec> {
        IntenW::new(self, 0)
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenset0Spec;
impl crate::RegisterSpec for Intenset0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset0::R`](R) reader structure"]
impl crate::Readable for Intenset0Spec {}
#[doc = "`write(|w| ..)` method takes [`intenset0::W`](W) writer structure"]
impl crate::Writable for Intenset0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET0 to value 0"]
impl crate::Resettable for Intenset0Spec {
    const RESET_VALUE: u32 = 0;
}
