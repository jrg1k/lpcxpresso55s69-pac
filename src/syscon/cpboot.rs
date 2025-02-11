#[doc = "Register `CPBOOT` reader"]
pub type R = crate::R<CpbootSpec>;
#[doc = "Register `CPBOOT` writer"]
pub type W = crate::W<CpbootSpec>;
#[doc = "Field `CPBOOT` reader - Coprocessor Boot Address for CPU1."]
pub type CpbootR = crate::FieldReader<u32>;
#[doc = "Field `CPBOOT` writer - Coprocessor Boot Address for CPU1."]
pub type CpbootW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub fn cpboot(&self) -> CpbootR {
        CpbootR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Coprocessor Boot Address for CPU1."]
    #[inline(always)]
    pub fn cpboot(&mut self) -> CpbootW<CpbootSpec> {
        CpbootW::new(self, 0)
    }
}
#[doc = "Coprocessor Boot Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpboot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpboot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpbootSpec;
impl crate::RegisterSpec for CpbootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpboot::R`](R) reader structure"]
impl crate::Readable for CpbootSpec {}
#[doc = "`write(|w| ..)` method takes [`cpboot::W`](W) writer structure"]
impl crate::Writable for CpbootSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPBOOT to value 0"]
impl crate::Resettable for CpbootSpec {
    const RESET_VALUE: u32 = 0;
}
