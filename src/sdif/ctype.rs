#[doc = "Register `CTYPE` reader"]
pub type R = crate::R<CtypeSpec>;
#[doc = "Register `CTYPE` writer"]
pub type W = crate::W<CtypeSpec>;
#[doc = "Field `CARD0_WIDTH0` reader - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
pub type Card0Width0R = crate::BitReader;
#[doc = "Field `CARD0_WIDTH0` writer - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
pub type Card0Width0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD1_WIDTH0` reader - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
pub type Card1Width0R = crate::BitReader;
#[doc = "Field `CARD1_WIDTH0` writer - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
pub type Card1Width0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD0_WIDTH1` reader - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
pub type Card0Width1R = crate::BitReader;
#[doc = "Field `CARD0_WIDTH1` writer - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
pub type Card0Width1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD1_WIDTH1` reader - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
pub type Card1Width1R = crate::BitReader;
#[doc = "Field `CARD1_WIDTH1` writer - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
pub type Card1Width1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card0_width0(&self) -> Card0Width0R {
        Card0Width0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card1_width0(&self) -> Card1Width0R {
        Card1Width0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card0_width1(&self) -> Card0Width1R {
        Card0Width1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card1_width1(&self) -> Card1Width1R {
        Card1Width1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if card 0 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD0_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card0_width0(&mut self) -> Card0Width0W<CtypeSpec> {
        Card0Width0W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates if card 1 is 1-bit or 4-bit: 0 - 1-bit mode 1 - 4-bit mode 1 and 4-bit modes only work when 8-bit mode in CARD1_WIDTH1 is not enabled (bit 16 in this register is set to 0)."]
    #[inline(always)]
    pub fn card1_width0(&mut self) -> Card1Width0W<CtypeSpec> {
        Card1Width0W::new(self, 1)
    }
    #[doc = "Bit 16 - Indicates if card 0 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card0_width1(&mut self) -> Card0Width1W<CtypeSpec> {
        Card0Width1W::new(self, 16)
    }
    #[doc = "Bit 17 - Indicates if card 1 is 8-bit: 0 - Non 8-bit mode 1 - 8-bit mode."]
    #[inline(always)]
    pub fn card1_width1(&mut self) -> Card1Width1W<CtypeSpec> {
        Card1Width1W::new(self, 17)
    }
}
#[doc = "Card Type register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtypeSpec;
impl crate::RegisterSpec for CtypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctype::R`](R) reader structure"]
impl crate::Readable for CtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`ctype::W`](W) writer structure"]
impl crate::Writable for CtypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTYPE to value 0"]
impl crate::Resettable for CtypeSpec {
    const RESET_VALUE: u32 = 0;
}
