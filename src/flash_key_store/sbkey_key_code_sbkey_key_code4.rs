#[doc = "Register `SBKEY_KEY_CODE4` reader"]
pub type R = crate::R<SbkeyKeyCodeSbkeyKeyCode4Spec>;
#[doc = "Register `SBKEY_KEY_CODE4` writer"]
pub type W = crate::W<SbkeyKeyCodeSbkeyKeyCode4Spec>;
#[doc = "Field `FIELD` reader - ."]
pub type FieldR = crate::FieldReader<u32>;
#[doc = "Field `FIELD` writer - ."]
pub type FieldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&self) -> FieldR {
        FieldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ."]
    #[inline(always)]
    pub fn field(&mut self) -> FieldW<SbkeyKeyCodeSbkeyKeyCode4Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`sbkey_key_code_sbkey_key_code4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbkey_key_code_sbkey_key_code4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SbkeyKeyCodeSbkeyKeyCode4Spec;
impl crate::RegisterSpec for SbkeyKeyCodeSbkeyKeyCode4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbkey_key_code_sbkey_key_code4::R`](R) reader structure"]
impl crate::Readable for SbkeyKeyCodeSbkeyKeyCode4Spec {}
#[doc = "`write(|w| ..)` method takes [`sbkey_key_code_sbkey_key_code4::W`](W) writer structure"]
impl crate::Writable for SbkeyKeyCodeSbkeyKeyCode4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBKEY_KEY_CODE4 to value 0"]
impl crate::Resettable for SbkeyKeyCodeSbkeyKeyCode4Spec {
    const RESET_VALUE: u32 = 0;
}
