#[doc = "Register `RESP[%s]` reader"]
pub type R = crate::R<RespSpec>;
#[doc = "Register `RESP[%s]` writer"]
pub type W = crate::W<RespSpec>;
#[doc = "Field `RESPONSE` reader - Bits of response."]
pub type ResponseR = crate::FieldReader<u32>;
#[doc = "Field `RESPONSE` writer - Bits of response."]
pub type ResponseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&self) -> ResponseR {
        ResponseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits of response."]
    #[inline(always)]
    pub fn response(&mut self) -> ResponseW<RespSpec> {
        ResponseW::new(self, 0)
    }
}
#[doc = "Response register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespSpec;
impl crate::RegisterSpec for RespSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp::R`](R) reader structure"]
impl crate::Readable for RespSpec {}
#[doc = "`write(|w| ..)` method takes [`resp::W`](W) writer structure"]
impl crate::Writable for RespSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESP[%s]
to value 0"]
impl crate::Resettable for RespSpec {
    const RESET_VALUE: u32 = 0;
}
