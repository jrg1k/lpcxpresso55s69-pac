#[doc = "Register `CARDTHRCTL` reader"]
pub type R = crate::R<CardthrctlSpec>;
#[doc = "Register `CARDTHRCTL` writer"]
pub type W = crate::W<CardthrctlSpec>;
#[doc = "Field `CARDRDTHREN` reader - Card Read Threshold Enable."]
pub type CardrdthrenR = crate::BitReader;
#[doc = "Field `CARDRDTHREN` writer - Card Read Threshold Enable."]
pub type CardrdthrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSYCLRINTEN` reader - Busy Clear Interrupt Enable."]
pub type BsyclrintenR = crate::BitReader;
#[doc = "Field `BSYCLRINTEN` writer - Busy Clear Interrupt Enable."]
pub type BsyclrintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARDTHRESHOLD` reader - Card Threshold size."]
pub type CardthresholdR = crate::FieldReader;
#[doc = "Field `CARDTHRESHOLD` writer - Card Threshold size."]
pub type CardthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&self) -> CardrdthrenR {
        CardrdthrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&self) -> BsyclrintenR {
        BsyclrintenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&self) -> CardthresholdR {
        CardthresholdR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Card Read Threshold Enable."]
    #[inline(always)]
    pub fn cardrdthren(&mut self) -> CardrdthrenW<CardthrctlSpec> {
        CardrdthrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Enable."]
    #[inline(always)]
    pub fn bsyclrinten(&mut self) -> BsyclrintenW<CardthrctlSpec> {
        BsyclrintenW::new(self, 1)
    }
    #[doc = "Bits 16:23 - Card Threshold size."]
    #[inline(always)]
    pub fn cardthreshold(&mut self) -> CardthresholdW<CardthrctlSpec> {
        CardthresholdW::new(self, 16)
    }
}
#[doc = "Card Threshold Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cardthrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cardthrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CardthrctlSpec;
impl crate::RegisterSpec for CardthrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cardthrctl::R`](R) reader structure"]
impl crate::Readable for CardthrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cardthrctl::W`](W) writer structure"]
impl crate::Writable for CardthrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CARDTHRCTL to value 0"]
impl crate::Resettable for CardthrctlSpec {
    const RESET_VALUE: u32 = 0;
}
