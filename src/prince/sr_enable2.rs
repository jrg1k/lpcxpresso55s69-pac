#[doc = "Register `SR_ENABLE2` reader"]
pub type R = crate::R<SrEnable2Spec>;
#[doc = "Register `SR_ENABLE2` writer"]
pub type W = crate::W<SrEnable2Spec>;
#[doc = "Field `EN` reader - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
pub type EnR = crate::FieldReader<u32>;
#[doc = "Field `EN` writer - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
pub type EnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<SrEnable2Spec> {
        EnW::new(self, 0)
    }
}
#[doc = "Sub-Region Enable register for region 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr_enable2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr_enable2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrEnable2Spec;
impl crate::RegisterSpec for SrEnable2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_enable2::R`](R) reader structure"]
impl crate::Readable for SrEnable2Spec {}
#[doc = "`write(|w| ..)` method takes [`sr_enable2::W`](W) writer structure"]
impl crate::Writable for SrEnable2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR_ENABLE2 to value 0"]
impl crate::Resettable for SrEnable2Spec {
    const RESET_VALUE: u32 = 0;
}
