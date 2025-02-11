#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VersionSpec>;
#[doc = "Field `VERSION` reader - Version of the PUF module."]
pub type VersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Version of the PUF module."]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(self.bits)
    }
}
#[doc = "PUF version register.\n\nYou can [`read`](crate::Reg::read) this register and get [`version::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VersionSpec;
impl crate::RegisterSpec for VersionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VersionSpec {}
#[doc = "`reset()` method sets VERSION to value 0"]
impl crate::Resettable for VersionSpec {
    const RESET_VALUE: u32 = 0;
}
