#[doc = "Register `IRQSET` writer"]
pub type W = crate::W<IrqsetSpec>;
#[doc = "Field `INTREQSET` writer - Writing 1 sets the corresponding bit in the IRQ0 register."]
pub type IntreqsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub fn intreqset(&mut self) -> IntreqsetW<IrqsetSpec> {
        IntreqsetW::new(self, 0)
    }
}
#[doc = "Set bits in IRQ0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqsetSpec;
impl crate::RegisterSpec for IrqsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irqset::W`](W) writer structure"]
impl crate::Writable for IrqsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSET to value 0"]
impl crate::Resettable for IrqsetSpec {
    const RESET_VALUE: u32 = 0;
}
