#[doc = "Register `KEY_BLOCK` writer"]
pub type W = crate::W<KeyBlockSpec>;
#[doc = "Field `KEY_BLOCK` writer - Write a value to block quiddikey/PUF all index."]
pub type KeyBlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write a value to block quiddikey/PUF all index."]
    #[inline(always)]
    pub fn key_block(&mut self) -> KeyBlockW<KeyBlockSpec> {
        KeyBlockW::new(self, 0)
    }
}
#[doc = "block quiddikey/PUF all index.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_block::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyBlockSpec;
impl crate::RegisterSpec for KeyBlockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`key_block::W`](W) writer structure"]
impl crate::Writable for KeyBlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_BLOCK to value 0x3cc3_5aa5"]
impl crate::Resettable for KeyBlockSpec {
    const RESET_VALUE: u32 = 0x3cc3_5aa5;
}
