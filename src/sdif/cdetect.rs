#[doc = "Register `CDETECT` reader"]
pub type R = crate::R<CdetectSpec>;
#[doc = "Register `CDETECT` writer"]
pub type W = crate::W<CdetectSpec>;
#[doc = "Field `CARD0_DETECT` reader - Card 0 detect"]
pub type Card0DetectR = crate::BitReader;
#[doc = "Field `CARD0_DETECT` writer - Card 0 detect"]
pub type Card0DetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD1_DETECT` reader - Card 1 detect"]
pub type Card1DetectR = crate::BitReader;
#[doc = "Field `CARD1_DETECT` writer - Card 1 detect"]
pub type Card1DetectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Card 0 detect"]
    #[inline(always)]
    pub fn card0_detect(&self) -> Card0DetectR {
        Card0DetectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Card 1 detect"]
    #[inline(always)]
    pub fn card1_detect(&self) -> Card1DetectR {
        Card1DetectR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card 0 detect"]
    #[inline(always)]
    pub fn card0_detect(&mut self) -> Card0DetectW<CdetectSpec> {
        Card0DetectW::new(self, 0)
    }
    #[doc = "Bit 1 - Card 1 detect"]
    #[inline(always)]
    pub fn card1_detect(&mut self) -> Card1DetectW<CdetectSpec> {
        Card1DetectW::new(self, 1)
    }
}
#[doc = "Card Detect register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdetect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdetect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdetectSpec;
impl crate::RegisterSpec for CdetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdetect::R`](R) reader structure"]
impl crate::Readable for CdetectSpec {}
#[doc = "`write(|w| ..)` method takes [`cdetect::W`](W) writer structure"]
impl crate::Writable for CdetectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CDETECT to value 0"]
impl crate::Resettable for CdetectSpec {
    const RESET_VALUE: u32 = 0;
}
