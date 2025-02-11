#[doc = "Register `KEYOUTINDEX` reader"]
pub type R = crate::R<KeyoutindexSpec>;
#[doc = "Register `KEYOUTINDEX` writer"]
pub type W = crate::W<KeyoutindexSpec>;
#[doc = "Field `KEYOUTIDX` reader - Key index for the key that is currently output via the Key Output register"]
pub type KeyoutidxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Key index for the key that is currently output via the Key Output register"]
    #[inline(always)]
    pub fn keyoutidx(&self) -> KeyoutidxR {
        KeyoutidxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "PUF Key Output Index register\n\nYou can [`read`](crate::Reg::read) this register and get [`keyoutindex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyoutindex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyoutindexSpec;
impl crate::RegisterSpec for KeyoutindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyoutindex::R`](R) reader structure"]
impl crate::Readable for KeyoutindexSpec {}
#[doc = "`write(|w| ..)` method takes [`keyoutindex::W`](W) writer structure"]
impl crate::Writable for KeyoutindexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYOUTINDEX to value 0"]
impl crate::Resettable for KeyoutindexSpec {
    const RESET_VALUE: u32 = 0;
}
