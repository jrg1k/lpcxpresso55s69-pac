#[doc = "Register `IRQCLR` writer"]
pub type W = crate::W<IrqclrSpec>;
#[doc = "Field `INTREQCLR` writer - Writing 1 clears the corresponding bit in the IRQ0 register."]
pub type IntreqclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Writing 1 clears the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub fn intreqclr(&mut self) -> IntreqclrW<IrqclrSpec> {
        IntreqclrW::new(self, 0)
    }
}
#[doc = "Clear bits in IRQ0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqclrSpec;
impl crate::RegisterSpec for IrqclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irqclr::W`](W) writer structure"]
impl crate::Writable for IrqclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQCLR to value 0"]
impl crate::Resettable for IrqclrSpec {
    const RESET_VALUE: u32 = 0;
}
