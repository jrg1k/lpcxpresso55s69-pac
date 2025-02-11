#[doc = "Register `INT_SET_ENABLE` writer"]
pub type W = crate::W<IntSetEnableSpec>;
#[doc = "Field `FAIL` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR` writer - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
pub type EccErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn fail(&mut self) -> FailW<IntSetEnableSpec> {
        FailW::new(self, 0)
    }
    #[doc = "Bit 1 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<IntSetEnableSpec> {
        ErrW::new(self, 1)
    }
    #[doc = "Bit 2 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<IntSetEnableSpec> {
        DoneW::new(self, 2)
    }
    #[doc = "Bit 3 - When a SET_ENABLE bit is written to 1, the corresponding INT_ENABLE bit is set."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> EccErrW<IntSetEnableSpec> {
        EccErrW::new(self, 3)
    }
}
#[doc = "Set interrupt enable bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_set_enable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSetEnableSpec;
impl crate::RegisterSpec for IntSetEnableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_set_enable::W`](W) writer structure"]
impl crate::Writable for IntSetEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_SET_ENABLE to value 0"]
impl crate::Resettable for IntSetEnableSpec {
    const RESET_VALUE: u32 = 0;
}
