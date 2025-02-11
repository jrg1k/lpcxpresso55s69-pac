#[doc = "Register `MUTEX` reader"]
pub type R = crate::R<MutexSpec>;
#[doc = "Register `MUTEX` writer"]
pub type W = crate::W<MutexSpec>;
#[doc = "Field `EX` reader - Cleared when read, set when written. See usage description above."]
pub type ExR = crate::BitReader;
#[doc = "Field `EX` writer - Cleared when read, set when written. See usage description above."]
pub type ExW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub fn ex(&self) -> ExR {
        ExR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub fn ex(&mut self) -> ExW<MutexSpec> {
        ExW::new(self, 0)
    }
}
#[doc = "Mutual exclusion register\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`mutex::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mutex::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MutexSpec;
impl crate::RegisterSpec for MutexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mutex::R`](R) reader structure"]
impl crate::Readable for MutexSpec {}
#[doc = "`write(|w| ..)` method takes [`mutex::W`](W) writer structure"]
impl crate::Writable for MutexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUTEX to value 0x01"]
impl crate::Resettable for MutexSpec {
    const RESET_VALUE: u32 = 0x01;
}
