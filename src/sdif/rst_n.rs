#[doc = "Register `RST_N` reader"]
pub type R = crate::R<RstNSpec>;
#[doc = "Register `RST_N` writer"]
pub type W = crate::W<RstNSpec>;
#[doc = "Field `CARD_RESET` reader - Hardware reset."]
pub type CardResetR = crate::BitReader;
#[doc = "Field `CARD_RESET` writer - Hardware reset."]
pub type CardResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    pub fn card_reset(&self) -> CardResetR {
        CardResetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware reset."]
    #[inline(always)]
    pub fn card_reset(&mut self) -> CardResetW<RstNSpec> {
        CardResetW::new(self, 0)
    }
}
#[doc = "Hardware Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstNSpec;
impl crate::RegisterSpec for RstNSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_n::R`](R) reader structure"]
impl crate::Readable for RstNSpec {}
#[doc = "`write(|w| ..)` method takes [`rst_n::W`](W) writer structure"]
impl crate::Writable for RstNSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST_N to value 0x01"]
impl crate::Resettable for RstNSpec {
    const RESET_VALUE: u32 = 0x01;
}
