#[doc = "Register `W0_16` reader"]
pub type R = crate::R<W0_16Spec>;
#[doc = "Register `W0_16` writer"]
pub type W = crate::W<W0_16Spec>;
#[doc = "Field `PWORD` reader - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PwordR = crate::FieldReader<u32>;
#[doc = "Field `PWORD` writer - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
pub type PwordW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&self) -> PwordR {
        PwordR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&mut self) -> PwordW<W0_16Spec> {
        PwordW::new(self, 0)
    }
}
#[doc = "Word pin registers for all port GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`w0_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W0_16Spec;
impl crate::RegisterSpec for W0_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w0_16::R`](R) reader structure"]
impl crate::Readable for W0_16Spec {}
#[doc = "`write(|w| ..)` method takes [`w0_16::W`](W) writer structure"]
impl crate::Writable for W0_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W0_16 to value 0"]
impl crate::Resettable for W0_16Spec {
    const RESET_VALUE: u32 = 0;
}
