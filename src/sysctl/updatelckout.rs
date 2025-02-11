#[doc = "Register `UPDATELCKOUT` reader"]
pub type R = crate::R<UpdatelckoutSpec>;
#[doc = "Register `UPDATELCKOUT` writer"]
pub type W = crate::W<UpdatelckoutSpec>;
#[doc = "All Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Updatelckout {
    #[doc = "0: Normal Mode. Can be written to."]
    NormalMode = 0,
    #[doc = "1: Protected Mode. Cannot be written to."]
    ProtectedMode = 1,
}
impl From<Updatelckout> for bool {
    #[inline(always)]
    fn from(variant: Updatelckout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATELCKOUT` reader - All Registers"]
pub type UpdatelckoutR = crate::BitReader<Updatelckout>;
impl UpdatelckoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Updatelckout {
        match self.bits {
            false => Updatelckout::NormalMode,
            true => Updatelckout::ProtectedMode,
        }
    }
    #[doc = "Normal Mode. Can be written to."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Updatelckout::NormalMode
    }
    #[doc = "Protected Mode. Cannot be written to."]
    #[inline(always)]
    pub fn is_protected_mode(&self) -> bool {
        *self == Updatelckout::ProtectedMode
    }
}
#[doc = "Field `UPDATELCKOUT` writer - All Registers"]
pub type UpdatelckoutW<'a, REG> = crate::BitWriter<'a, REG, Updatelckout>;
impl<'a, REG> UpdatelckoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode. Can be written to."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Updatelckout::NormalMode)
    }
    #[doc = "Protected Mode. Cannot be written to."]
    #[inline(always)]
    pub fn protected_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Updatelckout::ProtectedMode)
    }
}
impl R {
    #[doc = "Bit 0 - All Registers"]
    #[inline(always)]
    pub fn updatelckout(&self) -> UpdatelckoutR {
        UpdatelckoutR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - All Registers"]
    #[inline(always)]
    pub fn updatelckout(&mut self) -> UpdatelckoutW<UpdatelckoutSpec> {
        UpdatelckoutW::new(self, 0)
    }
}
#[doc = "update lock out control\n\nYou can [`read`](crate::Reg::read) this register and get [`updatelckout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`updatelckout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpdatelckoutSpec;
impl crate::RegisterSpec for UpdatelckoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`updatelckout::R`](R) reader structure"]
impl crate::Readable for UpdatelckoutSpec {}
#[doc = "`write(|w| ..)` method takes [`updatelckout::W`](W) writer structure"]
impl crate::Writable for UpdatelckoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UPDATELCKOUT to value 0"]
impl crate::Resettable for UpdatelckoutSpec {
    const RESET_VALUE: u32 = 0;
}
