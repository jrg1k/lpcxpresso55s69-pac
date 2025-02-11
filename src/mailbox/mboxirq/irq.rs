#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IrqSpec>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IrqSpec>;
#[doc = "Field `INTREQ` reader - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
pub type IntreqR = crate::FieldReader<u32>;
#[doc = "Field `INTREQ` writer - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
pub type IntreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[inline(always)]
    pub fn intreq(&self) -> IntreqR {
        IntreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[inline(always)]
    pub fn intreq(&mut self) -> IntreqW<IrqSpec> {
        IntreqW::new(self, 0)
    }
}
#[doc = "Interrupt request register for the Cortex-M0+ CPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqSpec;
impl crate::RegisterSpec for IrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IrqSpec {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IrqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IrqSpec {
    const RESET_VALUE: u32 = 0;
}
