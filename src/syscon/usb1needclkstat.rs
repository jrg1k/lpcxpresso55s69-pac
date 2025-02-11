#[doc = "Register `USB1NEEDCLKSTAT` reader"]
pub type R = crate::R<Usb1needclkstatSpec>;
#[doc = "Register `USB1NEEDCLKSTAT` writer"]
pub type W = crate::W<Usb1needclkstatSpec>;
#[doc = "USB1 Device need_clock signal status:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevNeedclk {
    #[doc = "0: DEV_NEEDCLK is low."]
    Low = 0,
    #[doc = "1: DEV_NEEDCLK is high."]
    High = 1,
}
impl From<DevNeedclk> for bool {
    #[inline(always)]
    fn from(variant: DevNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_NEEDCLK` reader - USB1 Device need_clock signal status:."]
pub type DevNeedclkR = crate::BitReader<DevNeedclk>;
impl DevNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevNeedclk {
        match self.bits {
            false => DevNeedclk::Low,
            true => DevNeedclk::High,
        }
    }
    #[doc = "DEV_NEEDCLK is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DevNeedclk::Low
    }
    #[doc = "DEV_NEEDCLK is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DevNeedclk::High
    }
}
#[doc = "USB1 Host need_clock signal status:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostNeedclk {
    #[doc = "0: HOST_NEEDCLK is low."]
    Low = 0,
    #[doc = "1: HOST_NEEDCLK is high."]
    High = 1,
}
impl From<HostNeedclk> for bool {
    #[inline(always)]
    fn from(variant: HostNeedclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_NEEDCLK` reader - USB1 Host need_clock signal status:."]
pub type HostNeedclkR = crate::BitReader<HostNeedclk>;
impl HostNeedclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostNeedclk {
        match self.bits {
            false => HostNeedclk::Low,
            true => HostNeedclk::High,
        }
    }
    #[doc = "HOST_NEEDCLK is low."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HostNeedclk::Low
    }
    #[doc = "HOST_NEEDCLK is high."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HostNeedclk::High
    }
}
impl R {
    #[doc = "Bit 0 - USB1 Device need_clock signal status:."]
    #[inline(always)]
    pub fn dev_needclk(&self) -> DevNeedclkR {
        DevNeedclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB1 Host need_clock signal status:."]
    #[inline(always)]
    pub fn host_needclk(&self) -> HostNeedclkR {
        HostNeedclkR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {}
#[doc = "USB1 need clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1needclkstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1needclkstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb1needclkstatSpec;
impl crate::RegisterSpec for Usb1needclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb1needclkstat::R`](R) reader structure"]
impl crate::Readable for Usb1needclkstatSpec {}
#[doc = "`write(|w| ..)` method takes [`usb1needclkstat::W`](W) writer structure"]
impl crate::Writable for Usb1needclkstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB1NEEDCLKSTAT to value 0"]
impl crate::Resettable for Usb1needclkstatSpec {
    const RESET_VALUE: u32 = 0;
}
