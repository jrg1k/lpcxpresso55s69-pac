#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Register `VERSION` writer"]
pub type W = crate::W<VersionSpec>;
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
    pub fn field(&mut self) -> FieldW<VersionSpec> {
        FieldW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VersionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VERSION to value 0"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0;
}
