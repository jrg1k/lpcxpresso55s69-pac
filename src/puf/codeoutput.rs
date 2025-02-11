#[doc = "Register `CODEOUTPUT` reader"]
pub type R = crate::R<CodeoutputSpec>;
#[doc = "Field `CODEOUT` reader - AC/KC output data"]
pub type CodeoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AC/KC output data"]
    #[inline(always)]
    pub fn codeout(&self) -> CodeoutR {
        CodeoutR::new(self.bits)
    }
}
#[doc = "PUF Code Output register\n\nYou can [`read`](crate::Reg::read) this register and get [`codeoutput::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodeoutputSpec;
impl crate::RegisterSpec for CodeoutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codeoutput::R`](R) reader structure"]
impl crate::Readable for CodeoutputSpec {}
#[doc = "`reset()` method sets CODEOUTPUT to value 0"]
impl crate::Resettable for CodeoutputSpec {
    const RESET_VALUE: u32 = 0;
}
