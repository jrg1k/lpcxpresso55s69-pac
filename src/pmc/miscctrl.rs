#[doc = "Register `MISCCTRL` reader"]
pub type R = crate::R<MiscctrlSpec>;
#[doc = "Register `MISCCTRL` writer"]
pub type W = crate::W<MiscctrlSpec>;
#[doc = "Select LDO Deep Sleep reference source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldodeepsleepref {
    #[doc = "0: LDO DEEP Sleep uses Flash buffer biasing as reference."]
    Flashbuffer = 0,
    #[doc = "1: LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    Bgp0p8v = 1,
}
impl From<Ldodeepsleepref> for bool {
    #[inline(always)]
    fn from(variant: Ldodeepsleepref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDODEEPSLEEPREF` reader - Select LDO Deep Sleep reference source."]
pub type LdodeepsleeprefR = crate::BitReader<Ldodeepsleepref>;
impl LdodeepsleeprefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldodeepsleepref {
        match self.bits {
            false => Ldodeepsleepref::Flashbuffer,
            true => Ldodeepsleepref::Bgp0p8v,
        }
    }
    #[doc = "LDO DEEP Sleep uses Flash buffer biasing as reference."]
    #[inline(always)]
    pub fn is_flashbuffer(&self) -> bool {
        *self == Ldodeepsleepref::Flashbuffer
    }
    #[doc = "LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    #[inline(always)]
    pub fn is_bgp0p8v(&self) -> bool {
        *self == Ldodeepsleepref::Bgp0p8v
    }
}
#[doc = "Field `LDODEEPSLEEPREF` writer - Select LDO Deep Sleep reference source."]
pub type LdodeepsleeprefW<'a, REG> = crate::BitWriter<'a, REG, Ldodeepsleepref>;
impl<'a, REG> LdodeepsleeprefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO DEEP Sleep uses Flash buffer biasing as reference."]
    #[inline(always)]
    pub fn flashbuffer(self) -> &'a mut crate::W<REG> {
        self.variant(Ldodeepsleepref::Flashbuffer)
    }
    #[doc = "LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    #[inline(always)]
    pub fn bgp0p8v(self) -> &'a mut crate::W<REG> {
        self.variant(Ldodeepsleepref::Bgp0p8v)
    }
}
#[doc = "Control the activation of LDO MEM High Z mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ldomemhighzmode {
    #[doc = "0: LDO MEM High Z mode is disabled."]
    Disable = 0,
    #[doc = "1: LDO MEM High Z mode is enabled."]
    Enable = 1,
}
impl From<Ldomemhighzmode> for bool {
    #[inline(always)]
    fn from(variant: Ldomemhighzmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOMEMHIGHZMODE` reader - Control the activation of LDO MEM High Z mode."]
pub type LdomemhighzmodeR = crate::BitReader<Ldomemhighzmode>;
impl LdomemhighzmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ldomemhighzmode {
        match self.bits {
            false => Ldomemhighzmode::Disable,
            true => Ldomemhighzmode::Enable,
        }
    }
    #[doc = "LDO MEM High Z mode is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ldomemhighzmode::Disable
    }
    #[doc = "LDO MEM High Z mode is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ldomemhighzmode::Enable
    }
}
#[doc = "Field `LDOMEMHIGHZMODE` writer - Control the activation of LDO MEM High Z mode."]
pub type LdomemhighzmodeW<'a, REG> = crate::BitWriter<'a, REG, Ldomemhighzmode>;
impl<'a, REG> LdomemhighzmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO MEM High Z mode is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ldomemhighzmode::Disable)
    }
    #[doc = "LDO MEM High Z mode is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ldomemhighzmode::Enable)
    }
}
#[doc = "Field `LOWPWR_FLASH_BUF` reader - no description available"]
pub type LowpwrFlashBufR = crate::BitReader;
#[doc = "Field `LOWPWR_FLASH_BUF` writer - no description available"]
pub type LowpwrFlashBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISCCTRL_3_8` reader - Reserved."]
pub type Miscctrl3_8R = crate::FieldReader;
#[doc = "Field `MISCCTRL_3_8` writer - Reserved."]
pub type Miscctrl3_8W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MODEWAKEUP0` reader - Configure wake up I/O 0 in Deep Power Down mode"]
pub type Modewakeup0R = crate::BitReader;
#[doc = "Field `MODEWAKEUP0` writer - Configure wake up I/O 0 in Deep Power Down mode"]
pub type Modewakeup0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEWAKEUP1` reader - Configure wake up I/O 1 in Deep Power Down mode"]
pub type Modewakeup1R = crate::BitReader;
#[doc = "Field `MODEWAKEUP1` writer - Configure wake up I/O 1 in Deep Power Down mode"]
pub type Modewakeup1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEWAKEUP2` reader - Configure wake up I/O 2 in Deep Power Down mode"]
pub type Modewakeup2R = crate::BitReader;
#[doc = "Field `MODEWAKEUP2` writer - Configure wake up I/O 2 in Deep Power Down mode"]
pub type Modewakeup2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEWAKEUP3` reader - Configure wake up I/O 3 in Deep Power Down mode"]
pub type Modewakeup3R = crate::BitReader;
#[doc = "Field `MODEWAKEUP3` writer - Configure wake up I/O 3 in Deep Power Down mode"]
pub type Modewakeup3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableBleed {
    #[doc = "0: LDO_MEM bleed current is enabled."]
    BleedEnable = 0,
    #[doc = "1: LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    BleedDisable = 1,
}
impl From<DisableBleed> for bool {
    #[inline(always)]
    fn from(variant: DisableBleed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISABLE_BLEED` reader - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
pub type DisableBleedR = crate::BitReader<DisableBleed>;
impl DisableBleedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableBleed {
        match self.bits {
            false => DisableBleed::BleedEnable,
            true => DisableBleed::BleedDisable,
        }
    }
    #[doc = "LDO_MEM bleed current is enabled."]
    #[inline(always)]
    pub fn is_bleed_enable(&self) -> bool {
        *self == DisableBleed::BleedEnable
    }
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    #[inline(always)]
    pub fn is_bleed_disable(&self) -> bool {
        *self == DisableBleed::BleedDisable
    }
}
#[doc = "Field `DISABLE_BLEED` writer - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
pub type DisableBleedW<'a, REG> = crate::BitWriter<'a, REG, DisableBleed>;
impl<'a, REG> DisableBleedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LDO_MEM bleed current is enabled."]
    #[inline(always)]
    pub fn bleed_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DisableBleed::BleedEnable)
    }
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    #[inline(always)]
    pub fn bleed_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DisableBleed::BleedDisable)
    }
}
#[doc = "Field `MISCCTRL_13_14` reader - Reserved."]
pub type Miscctrl13_14R = crate::FieldReader;
#[doc = "Field `MISCCTRL_13_14` writer - Reserved."]
pub type Miscctrl13_14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "WAKEUP IO event detector reset control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakupioRst {
    #[doc = "0: Wakeup IO is not reset."]
    Released = 0,
    #[doc = "1: Wakeup IO is reset."]
    Asserted = 1,
}
impl From<WakupioRst> for bool {
    #[inline(always)]
    fn from(variant: WakupioRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKUPIO_RST` reader - WAKEUP IO event detector reset control."]
pub type WakupioRstR = crate::BitReader<WakupioRst>;
impl WakupioRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakupioRst {
        match self.bits {
            false => WakupioRst::Released,
            true => WakupioRst::Asserted,
        }
    }
    #[doc = "Wakeup IO is not reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == WakupioRst::Released
    }
    #[doc = "Wakeup IO is reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == WakupioRst::Asserted
    }
}
#[doc = "Field `WAKUPIO_RST` writer - WAKEUP IO event detector reset control."]
pub type WakupioRstW<'a, REG> = crate::BitWriter<'a, REG, WakupioRst>;
impl<'a, REG> WakupioRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup IO is not reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(WakupioRst::Released)
    }
    #[doc = "Wakeup IO is reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(WakupioRst::Asserted)
    }
}
impl R {
    #[doc = "Bit 0 - Select LDO Deep Sleep reference source."]
    #[inline(always)]
    pub fn ldodeepsleepref(&self) -> LdodeepsleeprefR {
        LdodeepsleeprefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub fn ldomemhighzmode(&self) -> LdomemhighzmodeR {
        LdomemhighzmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn lowpwr_flash_buf(&self) -> LowpwrFlashBufR {
        LowpwrFlashBufR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_3_8(&self) -> Miscctrl3_8R {
        Miscctrl3_8R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Configure wake up I/O 0 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup0(&self) -> Modewakeup0R {
        Modewakeup0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configure wake up I/O 1 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup1(&self) -> Modewakeup1R {
        Modewakeup1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configure wake up I/O 2 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup2(&self) -> Modewakeup2R {
        Modewakeup2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Configure wake up I/O 3 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup3(&self) -> Modewakeup3R {
        Modewakeup3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn disable_bleed(&self) -> DisableBleedR {
        DisableBleedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_13_14(&self) -> Miscctrl13_14R {
        Miscctrl13_14R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub fn wakupio_rst(&self) -> WakupioRstR {
        WakupioRstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select LDO Deep Sleep reference source."]
    #[inline(always)]
    pub fn ldodeepsleepref(&mut self) -> LdodeepsleeprefW<MiscctrlSpec> {
        LdodeepsleeprefW::new(self, 0)
    }
    #[doc = "Bit 1 - Control the activation of LDO MEM High Z mode."]
    #[inline(always)]
    pub fn ldomemhighzmode(&mut self) -> LdomemhighzmodeW<MiscctrlSpec> {
        LdomemhighzmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn lowpwr_flash_buf(&mut self) -> LowpwrFlashBufW<MiscctrlSpec> {
        LowpwrFlashBufW::new(self, 2)
    }
    #[doc = "Bits 3:7 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_3_8(&mut self) -> Miscctrl3_8W<MiscctrlSpec> {
        Miscctrl3_8W::new(self, 3)
    }
    #[doc = "Bit 8 - Configure wake up I/O 0 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup0(&mut self) -> Modewakeup0W<MiscctrlSpec> {
        Modewakeup0W::new(self, 8)
    }
    #[doc = "Bit 9 - Configure wake up I/O 1 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup1(&mut self) -> Modewakeup1W<MiscctrlSpec> {
        Modewakeup1W::new(self, 9)
    }
    #[doc = "Bit 10 - Configure wake up I/O 2 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup2(&mut self) -> Modewakeup2W<MiscctrlSpec> {
        Modewakeup2W::new(self, 10)
    }
    #[doc = "Bit 11 - Configure wake up I/O 3 in Deep Power Down mode"]
    #[inline(always)]
    pub fn modewakeup3(&mut self) -> Modewakeup3W<MiscctrlSpec> {
        Modewakeup3W::new(self, 11)
    }
    #[doc = "Bit 12 - Controls LDO MEM bleed current. This field is expected to be controlled by the Low Power Software only in DEEP SLEEP low power mode."]
    #[inline(always)]
    pub fn disable_bleed(&mut self) -> DisableBleedW<MiscctrlSpec> {
        DisableBleedW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Reserved."]
    #[inline(always)]
    pub fn miscctrl_13_14(&mut self) -> Miscctrl13_14W<MiscctrlSpec> {
        Miscctrl13_14W::new(self, 13)
    }
    #[doc = "Bit 15 - WAKEUP IO event detector reset control."]
    #[inline(always)]
    pub fn wakupio_rst(&mut self) -> WakupioRstW<MiscctrlSpec> {
        WakupioRstW::new(self, 15)
    }
}
#[doc = "Dummy Control bus to PMU \\[Reset by: PoR, Pin Reset, Brown Out Detectors Reset, Deep Power Down Reset, Software Reset\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`miscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`miscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscctrlSpec;
impl crate::RegisterSpec for MiscctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`miscctrl::R`](R) reader structure"]
impl crate::Readable for MiscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`miscctrl::W`](W) writer structure"]
impl crate::Writable for MiscctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISCCTRL to value 0"]
impl crate::Resettable for MiscctrlSpec {
    const RESET_VALUE: u32 = 0;
}
