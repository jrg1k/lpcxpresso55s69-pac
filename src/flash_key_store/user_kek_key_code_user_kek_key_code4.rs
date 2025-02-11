#[doc = "Register `USER_KEK_KEY_CODE4` reader"]
pub type R = crate::R<UserKekKeyCodeUserKekKeyCode4Spec>;
#[doc = "Register `USER_KEK_KEY_CODE4` writer"]
pub type W = crate::W<UserKekKeyCodeUserKekKeyCode4Spec>;
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
    pub fn field(&mut self) -> FieldW<UserKekKeyCodeUserKekKeyCode4Spec> {
        FieldW::new(self, 0)
    }
}
#[doc = ".\n\nYou can [`read`](crate::Reg::read) this register and get [`user_kek_key_code_user_kek_key_code4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user_kek_key_code_user_kek_key_code4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserKekKeyCodeUserKekKeyCode4Spec;
impl crate::RegisterSpec for UserKekKeyCodeUserKekKeyCode4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_kek_key_code_user_kek_key_code4::R`](R) reader structure"]
impl crate::Readable for UserKekKeyCodeUserKekKeyCode4Spec {}
#[doc = "`write(|w| ..)` method takes [`user_kek_key_code_user_kek_key_code4::W`](W) writer structure"]
impl crate::Writable for UserKekKeyCodeUserKekKeyCode4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER_KEK_KEY_CODE4 to value 0"]
impl crate::Resettable for UserKekKeyCodeUserKekKeyCode4Spec {
    const RESET_VALUE: u32 = 0;
}
