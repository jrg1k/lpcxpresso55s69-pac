#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Latest IC Boot cause:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bootmode {
    #[doc = "0: Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    Powerup = 0,
    #[doc = "1: Latest IC boot was from DEEP SLEEP low power mode."]
    Deepsleep = 1,
    #[doc = "2: Latest IC boot was from POWER DOWN low power mode."]
    Powerdown = 2,
    #[doc = "3: Latest IC boot was from DEEP POWER DOWN low power mode."]
    Deeppowerdown = 3,
}
impl From<Bootmode> for u8 {
    #[inline(always)]
    fn from(variant: Bootmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bootmode {
    type Ux = u8;
}
impl crate::IsEnum for Bootmode {}
#[doc = "Field `BOOTMODE` reader - Latest IC Boot cause:."]
pub type BootmodeR = crate::FieldReader<Bootmode>;
impl BootmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootmode {
        match self.bits {
            0 => Bootmode::Powerup,
            1 => Bootmode::Deepsleep,
            2 => Bootmode::Powerdown,
            3 => Bootmode::Deeppowerdown,
            _ => unreachable!(),
        }
    }
    #[doc = "Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    #[inline(always)]
    pub fn is_powerup(&self) -> bool {
        *self == Bootmode::Powerup
    }
    #[doc = "Latest IC boot was from DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == Bootmode::Deepsleep
    }
    #[doc = "Latest IC boot was from POWER DOWN low power mode."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == Bootmode::Powerdown
    }
    #[doc = "Latest IC boot was from DEEP POWER DOWN low power mode."]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == Bootmode::Deeppowerdown
    }
}
impl R {
    #[doc = "Bits 18:19 - Latest IC Boot cause:."]
    #[inline(always)]
    pub fn bootmode(&self) -> BootmodeR {
        BootmodeR::new(((self.bits >> 18) & 3) as u8)
    }
}
#[doc = "Power Management Controller FSM (Finite State Machines) status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
