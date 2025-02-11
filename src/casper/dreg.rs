#[doc = "Register `DREG` reader"]
pub type R = crate::R<DregSpec>;
#[doc = "Register `DREG` writer"]
pub type W = crate::W<DregSpec>;
#[doc = "Field `REG_VALUE` reader - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub type RegValueR = crate::FieldReader<u32>;
#[doc = "Field `REG_VALUE` writer - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
pub type RegValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&self) -> RegValueR {
        RegValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to be fed into Multiplier. Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&mut self) -> RegValueW<DregSpec> {
        RegValueW::new(self, 0)
    }
}
#[doc = "D register\n\nYou can [`read`](crate::Reg::read) this register and get [`dreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DregSpec;
impl crate::RegisterSpec for DregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dreg::R`](R) reader structure"]
impl crate::Readable for DregSpec {}
#[doc = "`write(|w| ..)` method takes [`dreg::W`](W) writer structure"]
impl crate::Writable for DregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DREG to value 0"]
impl crate::Resettable for DregSpec {
    const RESET_VALUE: u32 = 0;
}
