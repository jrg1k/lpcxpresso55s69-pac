#[doc = "Register `MCLKIO` reader"]
pub type R = crate::R<MclkioSpec>;
#[doc = "Register `MCLKIO` writer"]
pub type W = crate::W<MclkioSpec>;
#[doc = "MCLK control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mclkio {
    #[doc = "0: input mode."]
    Input = 0,
    #[doc = "1: output mode."]
    Output = 1,
}
impl From<Mclkio> for bool {
    #[inline(always)]
    fn from(variant: Mclkio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLKIO` reader - MCLK control."]
pub type MclkioR = crate::BitReader<Mclkio>;
impl MclkioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mclkio {
        match self.bits {
            false => Mclkio::Input,
            true => Mclkio::Output,
        }
    }
    #[doc = "input mode."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Mclkio::Input
    }
    #[doc = "output mode."]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Mclkio::Output
    }
}
#[doc = "Field `MCLKIO` writer - MCLK control."]
pub type MclkioW<'a, REG> = crate::BitWriter<'a, REG, Mclkio>;
impl<'a, REG> MclkioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "input mode."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkio::Input)
    }
    #[doc = "output mode."]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Mclkio::Output)
    }
}
impl R {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&self) -> MclkioR {
        MclkioR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCLK control."]
    #[inline(always)]
    pub fn mclkio(&mut self) -> MclkioW<MclkioSpec> {
        MclkioW::new(self, 0)
    }
}
#[doc = "MCLK control\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MclkioSpec;
impl crate::RegisterSpec for MclkioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclkio::R`](R) reader structure"]
impl crate::Readable for MclkioSpec {}
#[doc = "`write(|w| ..)` method takes [`mclkio::W`](W) writer structure"]
impl crate::Writable for MclkioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCLKIO to value 0"]
impl crate::Resettable for MclkioSpec {
    const RESET_VALUE: u32 = 0;
}
