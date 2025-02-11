#[doc = "Register `PRINCE_REGION1_IV_CODE13` reader"]
pub type R = crate::R<PrinceRegion1IvCodePrinceRegion1IvCode13Spec>;
#[doc = "Register `PRINCE_REGION1_IV_CODE13` writer"]
pub type W = crate::W<PrinceRegion1IvCodePrinceRegion1IvCode13Spec>;
#[doc = "Field `FIELD` reader - no description available"]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - no description available"]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - no description available"]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<PrinceRegion1IvCodePrinceRegion1IvCode13Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region1_iv_code_prince_region1_iv_code13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region1_iv_code_prince_region1_iv_code13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceRegion1IvCodePrinceRegion1IvCode13Spec;
impl crate::RegisterSpec for PrinceRegion1IvCodePrinceRegion1IvCode13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_region1_iv_code_prince_region1_iv_code13::R`](R) reader structure"]
impl crate::Readable for PrinceRegion1IvCodePrinceRegion1IvCode13Spec {}
#[doc = "`write(|w| ..)` method takes [`prince_region1_iv_code_prince_region1_iv_code13::W`](W) writer structure"]
impl crate::Writable for PrinceRegion1IvCodePrinceRegion1IvCode13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRINCE_REGION1_IV_CODE13 to value 0"]
impl crate::Resettable for PrinceRegion1IvCodePrinceRegion1IvCode13Spec {
    const RESET_VALUE: u32 = 0;
}
