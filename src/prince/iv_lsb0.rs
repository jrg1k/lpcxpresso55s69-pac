#[doc = "Register `IV_LSB0` writer"]
pub type W = crate::W<IvLsb0Spec>;
#[doc = "Field `IVVAL` writer - Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
pub type IvvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub fn ivval(&mut self) -> IvvalW<IvLsb0Spec> {
        IvvalW::new(self, 0)
    }
}
#[doc = "Initial Vector register for region 0, Least Significant Bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_lsb0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvLsb0Spec;
impl crate::RegisterSpec for IvLsb0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iv_lsb0::W`](W) writer structure"]
impl crate::Writable for IvLsb0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IV_LSB0 to value 0"]
impl crate::Resettable for IvLsb0Spec {
    const RESET_VALUE: u32 = 0;
}
