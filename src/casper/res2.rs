#[doc = "Register `RES2` reader"]
pub type R = crate::R<Res2Spec>;
#[doc = "Register `RES2` writer"]
pub type W = crate::W<Res2Spec>;
#[doc = "Field `REG_VALUE` reader - Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
pub type RegValueR = crate::FieldReader<u32>;
#[doc = "Field `REG_VALUE` writer - Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
pub type RegValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&self) -> RegValueR {
        RegValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register to hold working result (from multiplier, adder/xor, etc). Is not normally written or read by application, but is available when accelerator not busy."]
    #[inline(always)]
    pub fn reg_value(&mut self) -> RegValueW<Res2Spec> {
        RegValueW::new(self, 0)
    }
}
#[doc = "Result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res2Spec;
impl crate::RegisterSpec for Res2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res2::R`](R) reader structure"]
impl crate::Readable for Res2Spec {}
#[doc = "`write(|w| ..)` method takes [`res2::W`](W) writer structure"]
impl crate::Writable for Res2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES2 to value 0"]
impl crate::Resettable for Res2Spec {
    const RESET_VALUE: u32 = 0;
}
