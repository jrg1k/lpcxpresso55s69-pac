#[doc = "Register `TCBCNT` reader"]
pub type R = crate::R<TcbcntSpec>;
#[doc = "Register `TCBCNT` writer"]
pub type W = crate::W<TcbcntSpec>;
#[doc = "Field `TRANS_CARD_BYTE_COUNT` reader - Number of bytes transferred by CIU unit to card."]
pub type TransCardByteCountR = crate::FieldReader<u32>;
#[doc = "Field `TRANS_CARD_BYTE_COUNT` writer - Number of bytes transferred by CIU unit to card."]
pub type TransCardByteCountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn trans_card_byte_count(&self) -> TransCardByteCountR {
        TransCardByteCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn trans_card_byte_count(&mut self) -> TransCardByteCountW<TcbcntSpec> {
        TransCardByteCountW::new(self, 0)
    }
}
#[doc = "Transferred CIU Card Byte Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcbcntSpec;
impl crate::RegisterSpec for TcbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcbcnt::R`](R) reader structure"]
impl crate::Readable for TcbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`tcbcnt::W`](W) writer structure"]
impl crate::Writable for TcbcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCBCNT to value 0"]
impl crate::Resettable for TcbcntSpec {
    const RESET_VALUE: u32 = 0;
}
