#[doc = "Register `PWREN` reader"]
pub type R = crate::R<PwrenSpec>;
#[doc = "Register `PWREN` writer"]
pub type W = crate::W<PwrenSpec>;
#[doc = "Field `POWER_ENABLE0` reader - Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
pub type PowerEnable0R = crate::BitReader;
#[doc = "Field `POWER_ENABLE0` writer - Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
pub type PowerEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POWER_ENABLE1` reader - Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
pub type PowerEnable1R = crate::BitReader;
#[doc = "Field `POWER_ENABLE1` writer - Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
pub type PowerEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[inline(always)]
    pub fn power_enable0(&self) -> PowerEnable0R {
        PowerEnable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[inline(always)]
    pub fn power_enable1(&self) -> PowerEnable1R {
        PowerEnable1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on/off switch for card 0; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 0."]
    #[inline(always)]
    pub fn power_enable0(&mut self) -> PowerEnable0W<PwrenSpec> {
        PowerEnable0W::new(self, 0)
    }
    #[doc = "Bit 1 - Power on/off switch for card 1; once power is turned on, software should wait for regulator/switch ramp-up time before trying to initialize card 1."]
    #[inline(always)]
    pub fn power_enable1(&mut self) -> PowerEnable1W<PwrenSpec> {
        PowerEnable1W::new(self, 1)
    }
}
#[doc = "Power Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrenSpec;
impl crate::RegisterSpec for PwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwren::R`](R) reader structure"]
impl crate::Readable for PwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwren::W`](W) writer structure"]
impl crate::Writable for PwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWREN to value 0"]
impl crate::Resettable for PwrenSpec {
    const RESET_VALUE: u32 = 0;
}
