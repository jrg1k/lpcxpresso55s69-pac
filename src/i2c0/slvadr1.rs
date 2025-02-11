#[doc = "Register `SLVADR1` reader"]
pub type R = crate::R<Slvadr1Spec>;
#[doc = "Register `SLVADR1` writer"]
pub type W = crate::W<Slvadr1Spec>;
#[doc = "Slave Address n Disable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sadisable {
    #[doc = "0: Enabled. Slave Address n is enabled."]
    Enabled = 0,
    #[doc = "1: Ignored Slave Address n is ignored."]
    Disabled = 1,
}
impl From<Sadisable> for bool {
    #[inline(always)]
    fn from(variant: Sadisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SADISABLE` reader - Slave Address n Disable."]
pub type SadisableR = crate::BitReader<Sadisable>;
impl SadisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sadisable {
        match self.bits {
            false => Sadisable::Enabled,
            true => Sadisable::Disabled,
        }
    }
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sadisable::Enabled
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sadisable::Disabled
    }
}
#[doc = "Field `SADISABLE` writer - Slave Address n Disable."]
pub type SadisableW<'a, REG> = crate::BitWriter<'a, REG, Sadisable>;
impl<'a, REG> SadisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled. Slave Address n is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sadisable::Enabled)
    }
    #[doc = "Ignored Slave Address n is ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sadisable::Disabled)
    }
}
#[doc = "Field `SLVADR` reader - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SlvadrR = crate::FieldReader;
#[doc = "Field `SLVADR` writer - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
pub type SlvadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&self) -> SadisableR {
        SadisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&self) -> SlvadrR {
        SlvadrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Address n Disable."]
    #[inline(always)]
    pub fn sadisable(&mut self) -> SadisableW<Slvadr1Spec> {
        SadisableW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn slvadr(&mut self) -> SlvadrW<Slvadr1Spec> {
        SlvadrW::new(self, 1)
    }
}
#[doc = "Slave address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`slvadr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvadr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slvadr1Spec;
impl crate::RegisterSpec for Slvadr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvadr1::R`](R) reader structure"]
impl crate::Readable for Slvadr1Spec {}
#[doc = "`write(|w| ..)` method takes [`slvadr1::W`](W) writer structure"]
impl crate::Writable for Slvadr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLVADR1 to value 0x01"]
impl crate::Resettable for Slvadr1Spec {
    const RESET_VALUE: u32 = 0x01;
}
