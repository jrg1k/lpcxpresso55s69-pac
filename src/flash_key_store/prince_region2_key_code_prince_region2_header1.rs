#[doc = "Register `PRINCE_REGION2_HEADER1` reader"]
pub type R = crate::R<PrinceRegion2KeyCodePrinceRegion2Header1Spec>;
#[doc = "Register `PRINCE_REGION2_HEADER1` writer"]
pub type W = crate::W<PrinceRegion2KeyCodePrinceRegion2Header1Spec>;
#[doc = "Field `TYPE` reader - ."]
pub type TypeR = crate::FieldReader;
#[doc = "Field `TYPE` writer - ."]
pub type TypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INDEX` reader - ."]
pub type IndexR = crate::FieldReader;
#[doc = "Field `INDEX` writer - ."]
pub type IndexW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SIZE` reader - ."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - ."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ."]
    #[inline(always)]
    pub fn type_(&mut self) -> TypeW<PrinceRegion2KeyCodePrinceRegion2Header1Spec> {
        TypeW::new(self, 0)
    }
    #[doc = "Bits 8:11 - ."]
    #[inline(always)]
    pub fn index(&mut self) -> IndexW<PrinceRegion2KeyCodePrinceRegion2Header1Spec> {
        IndexW::new(self, 8)
    }
    #[doc = "Bits 24:29 - ."]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<PrinceRegion2KeyCodePrinceRegion2Header1Spec> {
        SizeW::new(self, 24)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`prince_region2_key_code_prince_region2_header1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prince_region2_key_code_prince_region2_header1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrinceRegion2KeyCodePrinceRegion2Header1Spec;
impl crate::RegisterSpec for PrinceRegion2KeyCodePrinceRegion2Header1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prince_region2_key_code_prince_region2_header1::R`](R) reader structure"]
impl crate::Readable for PrinceRegion2KeyCodePrinceRegion2Header1Spec {}
#[doc = "`write(|w| ..)` method takes [`prince_region2_key_code_prince_region2_header1::W`](W) writer structure"]
impl crate::Writable for PrinceRegion2KeyCodePrinceRegion2Header1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRINCE_REGION2_HEADER1 to value 0"]
impl crate::Resettable for PrinceRegion2KeyCodePrinceRegion2Header1Spec {
    const RESET_VALUE: u32 = 0;
}
