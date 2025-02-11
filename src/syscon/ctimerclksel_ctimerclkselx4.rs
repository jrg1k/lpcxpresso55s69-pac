#[doc = "Register `CTIMERCLKSELX4` reader"]
pub type R = crate::R<CtimerclkselCtimerclkselx4Spec>;
#[doc = "Register `CTIMERCLKSELX4` writer"]
pub type W = crate::W<CtimerclkselCtimerclkselx4Spec>;
#[doc = "Field `DATA` reader - Data array value"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data array value"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<CtimerclkselCtimerclkselx4Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtimerclkselCtimerclkselx4Spec;
impl crate::RegisterSpec for CtimerclkselCtimerclkselx4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctimerclksel_ctimerclkselx4::R`](R) reader structure"]
impl crate::Readable for CtimerclkselCtimerclkselx4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctimerclksel_ctimerclkselx4::W`](W) writer structure"]
impl crate::Writable for CtimerclkselCtimerclkselx4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTIMERCLKSELX4 to value 0"]
impl crate::Resettable for CtimerclkselCtimerclkselx4Spec {
    const RESET_VALUE: u32 = 0;
}
