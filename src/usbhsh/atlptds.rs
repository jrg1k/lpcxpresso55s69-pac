#[doc = "Register `ATLPTDS` reader"]
pub type R = crate::R<AtlptdsSpec>;
#[doc = "Register `ATLPTDS` writer"]
pub type W = crate::W<AtlptdsSpec>;
#[doc = "Field `ATL_SKIP` reader - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type AtlSkipR = crate::FieldReader<u32>;
#[doc = "Field `ATL_SKIP` writer - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
pub type AtlSkipW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn atl_skip(&self) -> AtlSkipR {
        AtlSkipR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - When a bit in the PTD Skip Map is set to logic 1, the corresponding PTD will be skipped, independent of the V bit setting."]
    #[inline(always)]
    pub fn atl_skip(&mut self) -> AtlSkipW<AtlptdsSpec> {
        AtlSkipW::new(self, 0)
    }
}
#[doc = "Skip map for each ATL PTD\n\nYou can [`read`](crate::Reg::read) this register and get [`atlptds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atlptds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AtlptdsSpec;
impl crate::RegisterSpec for AtlptdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atlptds::R`](R) reader structure"]
impl crate::Readable for AtlptdsSpec {}
#[doc = "`write(|w| ..)` method takes [`atlptds::W`](W) writer structure"]
impl crate::Writable for AtlptdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATLPTDS to value 0"]
impl crate::Resettable for AtlptdsSpec {
    const RESET_VALUE: u32 = 0;
}
