#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `CLK_DIVIDER0` reader - Clock divider-0 value."]
pub type ClkDivider0R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER0` writer - Clock divider-0 value."]
pub type ClkDivider0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock divider-0 value."]
    #[inline(always)]
    pub fn clk_divider0(&self) -> ClkDivider0R {
        ClkDivider0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider-0 value."]
    #[inline(always)]
    pub fn clk_divider0(&mut self) -> ClkDivider0W<ClkdivSpec> {
        ClkDivider0W::new(self, 0)
    }
}
#[doc = "Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0;
}
