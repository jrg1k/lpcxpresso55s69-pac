#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    memoryremap: Memoryremap,
    _reserved1: [u8; 0x0c],
    ahbmatprio: Ahbmatprio,
    _reserved2: [u8; 0x24],
    cpu0stckcal: Cpu0stckcal,
    cpu0nstckcal: Cpu0nstckcal,
    cpu1stckcal: Cpu1stckcal,
    _reserved5: [u8; 0x04],
    nmisrc: Nmisrc,
    _reserved6: [u8; 0xb4],
    _reserved_6_presetctrl: [u8; 0x04],
    _reserved_7_presetctrl: [u8; 0x04],
    _reserved_8_presetctrl: [u8; 0x04],
    _reserved9: [u8; 0x14],
    presetctrlset: [Presetctrlset; 3],
    _reserved10: [u8; 0x14],
    presetctrlclr: [Presetctrlclr; 3],
    _reserved11: [u8; 0x14],
    swr_reset: SwrReset,
    _reserved12: [u8; 0x9c],
    _reserved_12_ahbclkctrl: [u8; 0x04],
    _reserved_13_ahbclkctrl: [u8; 0x04],
    _reserved_14_ahbclkctrl: [u8; 0x04],
    _reserved15: [u8; 0x14],
    ahbclkctrlset: [Ahbclkctrlset; 3],
    _reserved16: [u8; 0x14],
    ahbclkctrlclr: [Ahbclkctrlclr; 3],
    _reserved17: [u8; 0x14],
    _reserved_17_systickclksel: [u8; 0x04],
    _reserved_18_systickclksel: [u8; 0x04],
    traceclksel: Traceclksel,
    _reserved_20_ctimerclksel: [u8; 0x04],
    _reserved_21_ctimerclksel: [u8; 0x04],
    _reserved_22_ctimerclksel: [u8; 0x04],
    _reserved_23_ctimerclksel: [u8; 0x04],
    _reserved_24_ctimerclksel: [u8; 0x04],
    mainclksela: Mainclksela,
    mainclkselb: Mainclkselb,
    clkoutsel: Clkoutsel,
    _reserved28: [u8; 0x04],
    pll0clksel: Pll0clksel,
    pll1clksel: Pll1clksel,
    _reserved30: [u8; 0x0c],
    adcclksel: Adcclksel,
    usb0clksel: Usb0clksel,
    _reserved32: [u8; 0x04],
    _reserved_32_fcclksel: [u8; 0x04],
    _reserved_33_fcclksel: [u8; 0x04],
    _reserved_34_fcclksel: [u8; 0x04],
    _reserved_35_fcclksel: [u8; 0x04],
    _reserved_36_fcclksel: [u8; 0x04],
    _reserved_37_fcclksel: [u8; 0x04],
    _reserved_38_fcclksel: [u8; 0x04],
    _reserved_39_fcclksel: [u8; 0x04],
    hslspiclksel: Hslspiclksel,
    _reserved41: [u8; 0x0c],
    mclkclksel: Mclkclksel,
    _reserved42: [u8; 0x0c],
    sctclksel: Sctclksel,
    _reserved43: [u8; 0x04],
    sdioclksel: Sdioclksel,
    _reserved44: [u8; 0x04],
    systickclkdiv0: Systickclkdiv0,
    systickclkdiv1: Systickclkdiv1,
    traceclkdiv: Traceclkdiv,
    _reserved47: [u8; 0x14],
    _reserved_47_flexfrgctrl: [u8; 0x04],
    _reserved_48_flexfrgctrl: [u8; 0x04],
    _reserved_49_flexfrgctrl: [u8; 0x04],
    _reserved_50_flexfrgctrl: [u8; 0x04],
    _reserved_51_flexfrgctrl: [u8; 0x04],
    _reserved_52_flexfrgctrl: [u8; 0x04],
    _reserved_53_flexfrgctrl: [u8; 0x04],
    _reserved_54_flexfrgctrl: [u8; 0x04],
    _reserved55: [u8; 0x40],
    ahbclkdiv: Ahbclkdiv,
    clkoutdiv: Clkoutdiv,
    frohfdiv: Frohfdiv,
    wdtclkdiv: Wdtclkdiv,
    _reserved59: [u8; 0x04],
    adcclkdiv: Adcclkdiv,
    usb0clkdiv: Usb0clkdiv,
    _reserved61: [u8; 0x10],
    mclkdiv: Mclkdiv,
    _reserved62: [u8; 0x04],
    sctclkdiv: Sctclkdiv,
    _reserved63: [u8; 0x04],
    sdioclkdiv: Sdioclkdiv,
    _reserved64: [u8; 0x04],
    pll0clkdiv: Pll0clkdiv,
    _reserved65: [u8; 0x34],
    clockgenupdatelockout: Clockgenupdatelockout,
    fmccr: Fmccr,
    _reserved67: [u8; 0x08],
    usb0needclkctrl: Usb0needclkctrl,
    usb0needclkstat: Usb0needclkstat,
    _reserved69: [u8; 0x08],
    fmcflush: Fmcflush,
    mclkio: Mclkio,
    usb1needclkctrl: Usb1needclkctrl,
    usb1needclkstat: Usb1needclkstat,
    _reserved73: [u8; 0x34],
    sdioclkctrl: Sdioclkctrl,
    _reserved74: [u8; 0xfc],
    pll1ctrl: Pll1ctrl,
    pll1stat: Pll1stat,
    pll1ndec: Pll1ndec,
    pll1mdec: Pll1mdec,
    pll1pdec: Pll1pdec,
    _reserved79: [u8; 0x0c],
    pll0ctrl: Pll0ctrl,
    pll0stat: Pll0stat,
    pll0ndec: Pll0ndec,
    pll0pdec: Pll0pdec,
    pll0sscg0: Pll0sscg0,
    pll0sscg1: Pll0sscg1,
    _reserved85: [u8; 0x016c],
    funcretentionctrl: Funcretentionctrl,
    _reserved86: [u8; 0xf8],
    cpuctrl: Cpuctrl,
    cpboot: Cpboot,
    _reserved88: [u8; 0x04],
    cpstat: Cpstat,
    _reserved89: [u8; 0x0208],
    clock_ctrl: ClockCtrl,
    _reserved90: [u8; 0xf4],
    comp_int_ctrl: CompIntCtrl,
    comp_int_status: CompIntStatus,
    _reserved92: [u8; 0x02ec],
    autoclkgateoverride: Autoclkgateoverride,
    gpiopsync: Gpiopsync,
    _reserved94: [u8; 0x0194],
    debug_lock_en: DebugLockEn,
    debug_features: DebugFeatures,
    debug_features_dp: DebugFeaturesDp,
    _reserved97: [u8; 0x10],
    key_block: KeyBlock,
    debug_auth_beacon: DebugAuthBeacon,
    _reserved99: [u8; 0x10],
    cpucfg: Cpucfg,
    _reserved100: [u8; 0x20],
    device_id0: DeviceId0,
    dieid: Dieid,
}
impl RegisterBlock {
    #[doc = "0x00 - Memory Remap control register"]
    #[inline(always)]
    pub const fn memoryremap(&self) -> &Memoryremap {
        &self.memoryremap
    }
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    #[inline(always)]
    pub const fn ahbmatprio(&self) -> &Ahbmatprio {
        &self.ahbmatprio
    }
    #[doc = "0x38 - System tick calibration for secure part of CPU0"]
    #[inline(always)]
    pub const fn cpu0stckcal(&self) -> &Cpu0stckcal {
        &self.cpu0stckcal
    }
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0"]
    #[inline(always)]
    pub const fn cpu0nstckcal(&self) -> &Cpu0nstckcal {
        &self.cpu0nstckcal
    }
    #[doc = "0x40 - System tick calibration for CPU1"]
    #[inline(always)]
    pub const fn cpu1stckcal(&self) -> &Cpu1stckcal {
        &self.cpu1stckcal
    }
    #[doc = "0x48 - NMI Source Select"]
    #[inline(always)]
    pub const fn nmisrc(&self) -> &Nmisrc {
        &self.nmisrc
    }
    #[doc = "0x100 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn presetctrl_presetctrlx0(&self) -> &PresetctrlPresetctrlx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - Peripheral reset control 0"]
    #[inline(always)]
    pub const fn presetctrl_presetctrl0(&self) -> &PresetctrlPresetctrl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x104 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn presetctrl_presetctrlx1(&self) -> &PresetctrlPresetctrlx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - Peripheral reset control 1"]
    #[inline(always)]
    pub const fn presetctrl_presetctrl1(&self) -> &PresetctrlPresetctrl1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn presetctrl_presetctrlx2(&self) -> &PresetctrlPresetctrlx2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - Peripheral reset control 2"]
    #[inline(always)]
    pub const fn presetctrl_presetctrl2(&self) -> &PresetctrlPresetctrl2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x120..0x12c - Peripheral reset control set register"]
    #[inline(always)]
    pub const fn presetctrlset(&self, n: usize) -> &Presetctrlset {
        &self.presetctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x12c - Peripheral reset control set register"]
    #[inline(always)]
    pub fn presetctrlset_iter(&self) -> impl Iterator<Item = &Presetctrlset> {
        self.presetctrlset.iter()
    }
    #[doc = "0x140..0x14c - Peripheral reset control clear register"]
    #[inline(always)]
    pub const fn presetctrlclr(&self, n: usize) -> &Presetctrlclr {
        &self.presetctrlclr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x14c - Peripheral reset control clear register"]
    #[inline(always)]
    pub fn presetctrlclr_iter(&self) -> impl Iterator<Item = &Presetctrlclr> {
        self.presetctrlclr.iter()
    }
    #[doc = "0x160 - generate a software_reset"]
    #[inline(always)]
    pub const fn swr_reset(&self) -> &SwrReset {
        &self.swr_reset
    }
    #[doc = "0x200 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrlx0(&self) -> &AhbclkctrlAhbclkctrlx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - AHB Clock control 0"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrl0(&self) -> &AhbclkctrlAhbclkctrl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x204 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrlx1(&self) -> &AhbclkctrlAhbclkctrlx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - AHB Clock control 1"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrl1(&self) -> &AhbclkctrlAhbclkctrl1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x208 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrlx2(&self) -> &AhbclkctrlAhbclkctrlx2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - AHB Clock control 2"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrl2(&self) -> &AhbclkctrlAhbclkctrl2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x220..0x22c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrlset(&self, n: usize) -> &Ahbclkctrlset {
        &self.ahbclkctrlset[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x220..0x22c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ahbclkctrlset_iter(&self) -> impl Iterator<Item = &Ahbclkctrlset> {
        self.ahbclkctrlset.iter()
    }
    #[doc = "0x240..0x24c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrlclr(&self, n: usize) -> &Ahbclkctrlclr {
        &self.ahbclkctrlclr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x24c - Peripheral reset control register"]
    #[inline(always)]
    pub fn ahbclkctrlclr_iter(&self) -> impl Iterator<Item = &Ahbclkctrlclr> {
        self.ahbclkctrlclr.iter()
    }
    #[doc = "0x260 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclksel_systickclkselx0(&self) -> &SystickclkselSystickclkselx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub const fn systickclksel_systickclksel0(&self) -> &SystickclkselSystickclksel0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(608).cast() }
    }
    #[doc = "0x264 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclksel_systickclkselx1(&self) -> &SystickclkselSystickclkselx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x264 - System Tick Timer for CPU1 source select"]
    #[inline(always)]
    pub const fn systickclksel_systickclksel1(&self) -> &SystickclkselSystickclksel1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(612).cast() }
    }
    #[doc = "0x268 - Trace clock source select"]
    #[inline(always)]
    pub const fn traceclksel(&self) -> &Traceclksel {
        &self.traceclksel
    }
    #[doc = "0x26c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx0(&self) -> &CtimerclkselCtimerclkselx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x26c - CTimer 0 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel0(&self) -> &CtimerclkselCtimerclksel0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(620).cast() }
    }
    #[doc = "0x270 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx1(&self) -> &CtimerclkselCtimerclkselx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x270 - CTimer 1 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel1(&self) -> &CtimerclkselCtimerclksel1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(624).cast() }
    }
    #[doc = "0x274 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx2(&self) -> &CtimerclkselCtimerclkselx2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x274 - CTimer 2 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel2(&self) -> &CtimerclkselCtimerclksel2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(628).cast() }
    }
    #[doc = "0x278 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx3(&self) -> &CtimerclkselCtimerclkselx3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x278 - CTimer 3 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel3(&self) -> &CtimerclkselCtimerclksel3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(632).cast() }
    }
    #[doc = "0x27c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx4(&self) -> &CtimerclkselCtimerclkselx4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(636).cast() }
    }
    #[doc = "0x27c - CTimer 4 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel4(&self) -> &CtimerclkselCtimerclksel4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(636).cast() }
    }
    #[doc = "0x280 - Main clock A source select"]
    #[inline(always)]
    pub const fn mainclksela(&self) -> &Mainclksela {
        &self.mainclksela
    }
    #[doc = "0x284 - Main clock source select"]
    #[inline(always)]
    pub const fn mainclkselb(&self) -> &Mainclkselb {
        &self.mainclkselb
    }
    #[doc = "0x288 - CLKOUT clock source select"]
    #[inline(always)]
    pub const fn clkoutsel(&self) -> &Clkoutsel {
        &self.clkoutsel
    }
    #[doc = "0x290 - PLL0 clock source select"]
    #[inline(always)]
    pub const fn pll0clksel(&self) -> &Pll0clksel {
        &self.pll0clksel
    }
    #[doc = "0x294 - PLL1 clock source select"]
    #[inline(always)]
    pub const fn pll1clksel(&self) -> &Pll1clksel {
        &self.pll1clksel
    }
    #[doc = "0x2a4 - ADC clock source select"]
    #[inline(always)]
    pub const fn adcclksel(&self) -> &Adcclksel {
        &self.adcclksel
    }
    #[doc = "0x2a8 - FS USB clock source select"]
    #[inline(always)]
    pub const fn usb0clksel(&self) -> &Usb0clksel {
        &self.usb0clksel
    }
    #[doc = "0x2b0 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx0(&self) -> &FcclkselFcclkselx0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel0(&self) -> &FcclkselFcclksel0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b4 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx1(&self) -> &FcclkselFcclkselx1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel1(&self) -> &FcclkselFcclksel1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2b8 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx2(&self) -> &FcclkselFcclkselx2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(696).cast() }
    }
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel2(&self) -> &FcclkselFcclksel2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(696).cast() }
    }
    #[doc = "0x2bc - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx3(&self) -> &FcclkselFcclkselx3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(700).cast() }
    }
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel3(&self) -> &FcclkselFcclksel3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(700).cast() }
    }
    #[doc = "0x2c0 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx4(&self) -> &FcclkselFcclkselx4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(704).cast() }
    }
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel4(&self) -> &FcclkselFcclksel4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(704).cast() }
    }
    #[doc = "0x2c4 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx5(&self) -> &FcclkselFcclkselx5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(708).cast() }
    }
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel5(&self) -> &FcclkselFcclksel5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(708).cast() }
    }
    #[doc = "0x2c8 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx6(&self) -> &FcclkselFcclkselx6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(712).cast() }
    }
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel6(&self) -> &FcclkselFcclksel6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(712).cast() }
    }
    #[doc = "0x2cc - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx7(&self) -> &FcclkselFcclkselx7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(716).cast() }
    }
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel7(&self) -> &FcclkselFcclksel7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(716).cast() }
    }
    #[doc = "0x2d0 - HS LSPI clock source select"]
    #[inline(always)]
    pub const fn hslspiclksel(&self) -> &Hslspiclksel {
        &self.hslspiclksel
    }
    #[doc = "0x2e0 - MCLK clock source select"]
    #[inline(always)]
    pub const fn mclkclksel(&self) -> &Mclkclksel {
        &self.mclkclksel
    }
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    #[inline(always)]
    pub const fn sctclksel(&self) -> &Sctclksel {
        &self.sctclksel
    }
    #[doc = "0x2f8 - SDIO clock source select"]
    #[inline(always)]
    pub const fn sdioclksel(&self) -> &Sdioclksel {
        &self.sdioclksel
    }
    #[doc = "0x300 - System Tick Timer divider for CPU0"]
    #[inline(always)]
    pub const fn systickclkdiv0(&self) -> &Systickclkdiv0 {
        &self.systickclkdiv0
    }
    #[doc = "0x304 - System Tick Timer divider for CPU1"]
    #[inline(always)]
    pub const fn systickclkdiv1(&self) -> &Systickclkdiv1 {
        &self.systickclkdiv1
    }
    #[doc = "0x308 - TRACE clock divider"]
    #[inline(always)]
    pub const fn traceclkdiv(&self) -> &Traceclkdiv {
        &self.traceclkdiv
    }
    #[doc = "0x320 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl0(&self) -> &FlexfrgctrlFlexfrgxctrl0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(800).cast() }
    }
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg0ctrl(&self) -> &FlexfrgctrlFlexfrg0ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(800).cast() }
    }
    #[doc = "0x324 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl1(&self) -> &FlexfrgctrlFlexfrgxctrl1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(804).cast() }
    }
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg1ctrl(&self) -> &FlexfrgctrlFlexfrg1ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(804).cast() }
    }
    #[doc = "0x328 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl2(&self) -> &FlexfrgctrlFlexfrgxctrl2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(808).cast() }
    }
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg2ctrl(&self) -> &FlexfrgctrlFlexfrg2ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(808).cast() }
    }
    #[doc = "0x32c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl3(&self) -> &FlexfrgctrlFlexfrgxctrl3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(812).cast() }
    }
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg3ctrl(&self) -> &FlexfrgctrlFlexfrg3ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(812).cast() }
    }
    #[doc = "0x330 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl4(&self) -> &FlexfrgctrlFlexfrgxctrl4 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(816).cast() }
    }
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg4ctrl(&self) -> &FlexfrgctrlFlexfrg4ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(816).cast() }
    }
    #[doc = "0x334 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl5(&self) -> &FlexfrgctrlFlexfrgxctrl5 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(820).cast() }
    }
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg5ctrl(&self) -> &FlexfrgctrlFlexfrg5ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(820).cast() }
    }
    #[doc = "0x338 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl6(&self) -> &FlexfrgctrlFlexfrgxctrl6 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(824).cast() }
    }
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg6ctrl(&self) -> &FlexfrgctrlFlexfrg6ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(824).cast() }
    }
    #[doc = "0x33c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl7(&self) -> &FlexfrgctrlFlexfrgxctrl7 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(828).cast() }
    }
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg7ctrl(&self) -> &FlexfrgctrlFlexfrg7ctrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(828).cast() }
    }
    #[doc = "0x380 - System clock divider"]
    #[inline(always)]
    pub const fn ahbclkdiv(&self) -> &Ahbclkdiv {
        &self.ahbclkdiv
    }
    #[doc = "0x384 - CLKOUT clock divider"]
    #[inline(always)]
    pub const fn clkoutdiv(&self) -> &Clkoutdiv {
        &self.clkoutdiv
    }
    #[doc = "0x388 - FRO_HF (96MHz) clock divider"]
    #[inline(always)]
    pub const fn frohfdiv(&self) -> &Frohfdiv {
        &self.frohfdiv
    }
    #[doc = "0x38c - WDT clock divider"]
    #[inline(always)]
    pub const fn wdtclkdiv(&self) -> &Wdtclkdiv {
        &self.wdtclkdiv
    }
    #[doc = "0x394 - ADC clock divider"]
    #[inline(always)]
    pub const fn adcclkdiv(&self) -> &Adcclkdiv {
        &self.adcclkdiv
    }
    #[doc = "0x398 - USB0 Clock divider"]
    #[inline(always)]
    pub const fn usb0clkdiv(&self) -> &Usb0clkdiv {
        &self.usb0clkdiv
    }
    #[doc = "0x3ac - I2S MCLK clock divider"]
    #[inline(always)]
    pub const fn mclkdiv(&self) -> &Mclkdiv {
        &self.mclkdiv
    }
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    #[inline(always)]
    pub const fn sctclkdiv(&self) -> &Sctclkdiv {
        &self.sctclkdiv
    }
    #[doc = "0x3bc - SDIO clock divider"]
    #[inline(always)]
    pub const fn sdioclkdiv(&self) -> &Sdioclkdiv {
        &self.sdioclkdiv
    }
    #[doc = "0x3c4 - PLL0 clock divider"]
    #[inline(always)]
    pub const fn pll0clkdiv(&self) -> &Pll0clkdiv {
        &self.pll0clkdiv
    }
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    #[inline(always)]
    pub const fn clockgenupdatelockout(&self) -> &Clockgenupdatelockout {
        &self.clockgenupdatelockout
    }
    #[doc = "0x400 - FMC configuration register"]
    #[inline(always)]
    pub const fn fmccr(&self) -> &Fmccr {
        &self.fmccr
    }
    #[doc = "0x40c - USB0 need clock control"]
    #[inline(always)]
    pub const fn usb0needclkctrl(&self) -> &Usb0needclkctrl {
        &self.usb0needclkctrl
    }
    #[doc = "0x410 - USB0 need clock status"]
    #[inline(always)]
    pub const fn usb0needclkstat(&self) -> &Usb0needclkstat {
        &self.usb0needclkstat
    }
    #[doc = "0x41c - FMCflush control"]
    #[inline(always)]
    pub const fn fmcflush(&self) -> &Fmcflush {
        &self.fmcflush
    }
    #[doc = "0x420 - MCLK control"]
    #[inline(always)]
    pub const fn mclkio(&self) -> &Mclkio {
        &self.mclkio
    }
    #[doc = "0x424 - USB1 need clock control"]
    #[inline(always)]
    pub const fn usb1needclkctrl(&self) -> &Usb1needclkctrl {
        &self.usb1needclkctrl
    }
    #[doc = "0x428 - USB1 need clock status"]
    #[inline(always)]
    pub const fn usb1needclkstat(&self) -> &Usb1needclkstat {
        &self.usb1needclkstat
    }
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    #[inline(always)]
    pub const fn sdioclkctrl(&self) -> &Sdioclkctrl {
        &self.sdioclkctrl
    }
    #[doc = "0x560 - PLL1 550m control"]
    #[inline(always)]
    pub const fn pll1ctrl(&self) -> &Pll1ctrl {
        &self.pll1ctrl
    }
    #[doc = "0x564 - PLL1 550m status"]
    #[inline(always)]
    pub const fn pll1stat(&self) -> &Pll1stat {
        &self.pll1stat
    }
    #[doc = "0x568 - PLL1 550m N divider"]
    #[inline(always)]
    pub const fn pll1ndec(&self) -> &Pll1ndec {
        &self.pll1ndec
    }
    #[doc = "0x56c - PLL1 550m M divider"]
    #[inline(always)]
    pub const fn pll1mdec(&self) -> &Pll1mdec {
        &self.pll1mdec
    }
    #[doc = "0x570 - PLL1 550m P divider"]
    #[inline(always)]
    pub const fn pll1pdec(&self) -> &Pll1pdec {
        &self.pll1pdec
    }
    #[doc = "0x580 - PLL0 550m control"]
    #[inline(always)]
    pub const fn pll0ctrl(&self) -> &Pll0ctrl {
        &self.pll0ctrl
    }
    #[doc = "0x584 - PLL0 550m status"]
    #[inline(always)]
    pub const fn pll0stat(&self) -> &Pll0stat {
        &self.pll0stat
    }
    #[doc = "0x588 - PLL0 550m N divider"]
    #[inline(always)]
    pub const fn pll0ndec(&self) -> &Pll0ndec {
        &self.pll0ndec
    }
    #[doc = "0x58c - PLL0 550m P divider"]
    #[inline(always)]
    pub const fn pll0pdec(&self) -> &Pll0pdec {
        &self.pll0pdec
    }
    #[doc = "0x590 - PLL0 Spread Spectrum Wrapper control register 0"]
    #[inline(always)]
    pub const fn pll0sscg0(&self) -> &Pll0sscg0 {
        &self.pll0sscg0
    }
    #[doc = "0x594 - PLL0 Spread Spectrum Wrapper control register 1"]
    #[inline(always)]
    pub const fn pll0sscg1(&self) -> &Pll0sscg1 {
        &self.pll0sscg1
    }
    #[doc = "0x704 - Functional retention control register"]
    #[inline(always)]
    pub const fn funcretentionctrl(&self) -> &Funcretentionctrl {
        &self.funcretentionctrl
    }
    #[doc = "0x800 - CPU Control for multiple processors"]
    #[inline(always)]
    pub const fn cpuctrl(&self) -> &Cpuctrl {
        &self.cpuctrl
    }
    #[doc = "0x804 - Coprocessor Boot Address"]
    #[inline(always)]
    pub const fn cpboot(&self) -> &Cpboot {
        &self.cpboot
    }
    #[doc = "0x80c - CPU Status"]
    #[inline(always)]
    pub const fn cpstat(&self) -> &Cpstat {
        &self.cpstat
    }
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &ClockCtrl {
        &self.clock_ctrl
    }
    #[doc = "0xb10 - Comparator Interrupt control"]
    #[inline(always)]
    pub const fn comp_int_ctrl(&self) -> &CompIntCtrl {
        &self.comp_int_ctrl
    }
    #[doc = "0xb14 - Comparator Interrupt status"]
    #[inline(always)]
    pub const fn comp_int_status(&self) -> &CompIntStatus {
        &self.comp_int_status
    }
    #[doc = "0xe04 - Control automatic clock gating"]
    #[inline(always)]
    pub const fn autoclkgateoverride(&self) -> &Autoclkgateoverride {
        &self.autoclkgateoverride
    }
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    #[inline(always)]
    pub const fn gpiopsync(&self) -> &Gpiopsync {
        &self.gpiopsync
    }
    #[doc = "0xfa0 - Control write access to security registers."]
    #[inline(always)]
    pub const fn debug_lock_en(&self) -> &DebugLockEn {
        &self.debug_lock_en
    }
    #[doc = "0xfa4 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
    #[inline(always)]
    pub const fn debug_features(&self) -> &DebugFeatures {
        &self.debug_features
    }
    #[doc = "0xfa8 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
    #[inline(always)]
    pub const fn debug_features_dp(&self) -> &DebugFeaturesDp {
        &self.debug_features_dp
    }
    #[doc = "0xfbc - block quiddikey/PUF all index."]
    #[inline(always)]
    pub const fn key_block(&self) -> &KeyBlock {
        &self.key_block
    }
    #[doc = "0xfc0 - Debug authentication BEACON register"]
    #[inline(always)]
    pub const fn debug_auth_beacon(&self) -> &DebugAuthBeacon {
        &self.debug_auth_beacon
    }
    #[doc = "0xfd4 - CPUs configuration register"]
    #[inline(always)]
    pub const fn cpucfg(&self) -> &Cpucfg {
        &self.cpucfg
    }
    #[doc = "0xff8 - Device ID"]
    #[inline(always)]
    pub const fn device_id0(&self) -> &DeviceId0 {
        &self.device_id0
    }
    #[doc = "0xffc - Chip revision ID and Number"]
    #[inline(always)]
    pub const fn dieid(&self) -> &Dieid {
        &self.dieid
    }
}
#[doc = "MEMORYREMAP (rw) register accessor: Memory Remap control register\n\nYou can [`read`](crate::Reg::read) this register and get [`memoryremap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memoryremap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memoryremap`]
module"]
#[doc(alias = "MEMORYREMAP")]
pub type Memoryremap = crate::Reg<memoryremap::MemoryremapSpec>;
#[doc = "Memory Remap control register"]
pub mod memoryremap;
#[doc = "AHBMATPRIO (rw) register accessor: AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatprio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatprio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbmatprio`]
module"]
#[doc(alias = "AHBMATPRIO")]
pub type Ahbmatprio = crate::Reg<ahbmatprio::AhbmatprioSpec>;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub mod ahbmatprio;
#[doc = "CPU0STCKCAL (rw) register accessor: System tick calibration for secure part of CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0stckcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0stckcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0stckcal`]
module"]
#[doc(alias = "CPU0STCKCAL")]
pub type Cpu0stckcal = crate::Reg<cpu0stckcal::Cpu0stckcalSpec>;
#[doc = "System tick calibration for secure part of CPU0"]
pub mod cpu0stckcal;
#[doc = "CPU0NSTCKCAL (rw) register accessor: System tick calibration for non-secure part of CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu0nstckcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu0nstckcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu0nstckcal`]
module"]
#[doc(alias = "CPU0NSTCKCAL")]
pub type Cpu0nstckcal = crate::Reg<cpu0nstckcal::Cpu0nstckcalSpec>;
#[doc = "System tick calibration for non-secure part of CPU0"]
pub mod cpu0nstckcal;
#[doc = "CPU1STCKCAL (rw) register accessor: System tick calibration for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1stckcal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1stckcal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu1stckcal`]
module"]
#[doc(alias = "CPU1STCKCAL")]
pub type Cpu1stckcal = crate::Reg<cpu1stckcal::Cpu1stckcalSpec>;
#[doc = "System tick calibration for CPU1"]
pub mod cpu1stckcal;
#[doc = "NMISRC (rw) register accessor: NMI Source Select\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmisrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisrc`]
module"]
#[doc(alias = "NMISRC")]
pub type Nmisrc = crate::Reg<nmisrc::NmisrcSpec>;
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "PRESETCTRL_PRESETCTRL0 (rw) register accessor: Peripheral reset control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl_presetctrl0`]
module"]
#[doc(alias = "PRESETCTRL_PRESETCTRL0")]
pub type PresetctrlPresetctrl0 = crate::Reg<presetctrl_presetctrl0::PresetctrlPresetctrl0Spec>;
#[doc = "Peripheral reset control 0"]
pub mod presetctrl_presetctrl0;
#[doc = "PRESETCTRL_PRESETCTRLX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrlx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrlx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl_presetctrlx0`]
module"]
#[doc(alias = "PRESETCTRL_PRESETCTRLX0")]
pub type PresetctrlPresetctrlx0 = crate::Reg<presetctrl_presetctrlx0::PresetctrlPresetctrlx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl_presetctrlx0;
#[doc = "PRESETCTRL_PRESETCTRL1 (rw) register accessor: Peripheral reset control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl_presetctrl1`]
module"]
#[doc(alias = "PRESETCTRL_PRESETCTRL1")]
pub type PresetctrlPresetctrl1 = crate::Reg<presetctrl_presetctrl1::PresetctrlPresetctrl1Spec>;
#[doc = "Peripheral reset control 1"]
pub mod presetctrl_presetctrl1;
#[doc = "PRESETCTRL_PRESETCTRLX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrlx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrlx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl_presetctrlx1`]
module"]
#[doc(alias = "PRESETCTRL_PRESETCTRLX1")]
pub type PresetctrlPresetctrlx1 = crate::Reg<presetctrl_presetctrlx1::PresetctrlPresetctrlx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl_presetctrlx1;
#[doc = "PRESETCTRL_PRESETCTRL2 (rw) register accessor: Peripheral reset control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl_presetctrl2`]
module"]
#[doc(alias = "PRESETCTRL_PRESETCTRL2")]
pub type PresetctrlPresetctrl2 = crate::Reg<presetctrl_presetctrl2::PresetctrlPresetctrl2Spec>;
#[doc = "Peripheral reset control 2"]
pub mod presetctrl_presetctrl2;
#[doc = "PRESETCTRL_PRESETCTRLX2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrl_presetctrlx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrl_presetctrlx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrl_presetctrlx2`]
module"]
#[doc(alias = "PRESETCTRL_PRESETCTRLX2")]
pub type PresetctrlPresetctrlx2 = crate::Reg<presetctrl_presetctrlx2::PresetctrlPresetctrlx2Spec>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl_presetctrlx2;
#[doc = "PRESETCTRLSET (rw) register accessor: Peripheral reset control set register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrlset`]
module"]
#[doc(alias = "PRESETCTRLSET")]
pub type Presetctrlset = crate::Reg<presetctrlset::PresetctrlsetSpec>;
#[doc = "Peripheral reset control set register"]
pub mod presetctrlset;
#[doc = "PRESETCTRLCLR (rw) register accessor: Peripheral reset control clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`presetctrlclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`presetctrlclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@presetctrlclr`]
module"]
#[doc(alias = "PRESETCTRLCLR")]
pub type Presetctrlclr = crate::Reg<presetctrlclr::PresetctrlclrSpec>;
#[doc = "Peripheral reset control clear register"]
pub mod presetctrlclr;
#[doc = "SWR_RESET (w) register accessor: generate a software_reset\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swr_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swr_reset`]
module"]
#[doc(alias = "SWR_RESET")]
pub type SwrReset = crate::Reg<swr_reset::SwrResetSpec>;
#[doc = "generate a software_reset"]
pub mod swr_reset;
#[doc = "AHBCLKCTRL_AHBCLKCTRL0 (rw) register accessor: AHB Clock control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl_ahbclkctrl0`]
module"]
#[doc(alias = "AHBCLKCTRL_AHBCLKCTRL0")]
pub type AhbclkctrlAhbclkctrl0 = crate::Reg<ahbclkctrl_ahbclkctrl0::AhbclkctrlAhbclkctrl0Spec>;
#[doc = "AHB Clock control 0"]
pub mod ahbclkctrl_ahbclkctrl0;
#[doc = "AHBCLKCTRL_AHBCLKCTRLX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrlx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrlx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl_ahbclkctrlx0`]
module"]
#[doc(alias = "AHBCLKCTRL_AHBCLKCTRLX0")]
pub type AhbclkctrlAhbclkctrlx0 = crate::Reg<ahbclkctrl_ahbclkctrlx0::AhbclkctrlAhbclkctrlx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrl_ahbclkctrlx0;
#[doc = "AHBCLKCTRL_AHBCLKCTRL1 (rw) register accessor: AHB Clock control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl_ahbclkctrl1`]
module"]
#[doc(alias = "AHBCLKCTRL_AHBCLKCTRL1")]
pub type AhbclkctrlAhbclkctrl1 = crate::Reg<ahbclkctrl_ahbclkctrl1::AhbclkctrlAhbclkctrl1Spec>;
#[doc = "AHB Clock control 1"]
pub mod ahbclkctrl_ahbclkctrl1;
#[doc = "AHBCLKCTRL_AHBCLKCTRLX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrlx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrlx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl_ahbclkctrlx1`]
module"]
#[doc(alias = "AHBCLKCTRL_AHBCLKCTRLX1")]
pub type AhbclkctrlAhbclkctrlx1 = crate::Reg<ahbclkctrl_ahbclkctrlx1::AhbclkctrlAhbclkctrlx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrl_ahbclkctrlx1;
#[doc = "AHBCLKCTRL_AHBCLKCTRL2 (rw) register accessor: AHB Clock control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl_ahbclkctrl2`]
module"]
#[doc(alias = "AHBCLKCTRL_AHBCLKCTRL2")]
pub type AhbclkctrlAhbclkctrl2 = crate::Reg<ahbclkctrl_ahbclkctrl2::AhbclkctrlAhbclkctrl2Spec>;
#[doc = "AHB Clock control 2"]
pub mod ahbclkctrl_ahbclkctrl2;
#[doc = "AHBCLKCTRL_AHBCLKCTRLX2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrl_ahbclkctrlx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrl_ahbclkctrlx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrl_ahbclkctrlx2`]
module"]
#[doc(alias = "AHBCLKCTRL_AHBCLKCTRLX2")]
pub type AhbclkctrlAhbclkctrlx2 = crate::Reg<ahbclkctrl_ahbclkctrlx2::AhbclkctrlAhbclkctrlx2Spec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrl_ahbclkctrlx2;
#[doc = "AHBCLKCTRLSET (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrlset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrlset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrlset`]
module"]
#[doc(alias = "AHBCLKCTRLSET")]
pub type Ahbclkctrlset = crate::Reg<ahbclkctrlset::AhbclkctrlsetSpec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlset;
#[doc = "AHBCLKCTRLCLR (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkctrlclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkctrlclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkctrlclr`]
module"]
#[doc(alias = "AHBCLKCTRLCLR")]
pub type Ahbclkctrlclr = crate::Reg<ahbclkctrlclr::AhbclkctrlclrSpec>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlclr;
#[doc = "SYSTICKCLKSEL_SYSTICKCLKSEL0 (rw) register accessor: System Tick Timer for CPU0 source select\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel_systickclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel_systickclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclksel_systickclksel0`]
module"]
#[doc(alias = "SYSTICKCLKSEL_SYSTICKCLKSEL0")]
pub type SystickclkselSystickclksel0 =
    crate::Reg<systickclksel_systickclksel0::SystickclkselSystickclksel0Spec>;
#[doc = "System Tick Timer for CPU0 source select"]
pub mod systickclksel_systickclksel0;
#[doc = "SYSTICKCLKSEL_SYSTICKCLKSELX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel_systickclkselx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel_systickclkselx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclksel_systickclkselx0`]
module"]
#[doc(alias = "SYSTICKCLKSEL_SYSTICKCLKSELX0")]
pub type SystickclkselSystickclkselx0 =
    crate::Reg<systickclksel_systickclkselx0::SystickclkselSystickclkselx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod systickclksel_systickclkselx0;
#[doc = "SYSTICKCLKSEL_SYSTICKCLKSEL1 (rw) register accessor: System Tick Timer for CPU1 source select\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel_systickclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel_systickclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclksel_systickclksel1`]
module"]
#[doc(alias = "SYSTICKCLKSEL_SYSTICKCLKSEL1")]
pub type SystickclkselSystickclksel1 =
    crate::Reg<systickclksel_systickclksel1::SystickclkselSystickclksel1Spec>;
#[doc = "System Tick Timer for CPU1 source select"]
pub mod systickclksel_systickclksel1;
#[doc = "SYSTICKCLKSEL_SYSTICKCLKSELX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclksel_systickclkselx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclksel_systickclkselx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclksel_systickclkselx1`]
module"]
#[doc(alias = "SYSTICKCLKSEL_SYSTICKCLKSELX1")]
pub type SystickclkselSystickclkselx1 =
    crate::Reg<systickclksel_systickclkselx1::SystickclkselSystickclkselx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod systickclksel_systickclkselx1;
#[doc = "TRACECLKSEL (rw) register accessor: Trace clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclksel`]
module"]
#[doc(alias = "TRACECLKSEL")]
pub type Traceclksel = crate::Reg<traceclksel::TraceclkselSpec>;
#[doc = "Trace clock source select"]
pub mod traceclksel;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL0 (rw) register accessor: CTimer 0 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclksel0`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSEL0")]
pub type CtimerclkselCtimerclksel0 =
    crate::Reg<ctimerclksel_ctimerclksel0::CtimerclkselCtimerclksel0Spec>;
#[doc = "CTimer 0 clock source select"]
pub mod ctimerclksel_ctimerclksel0;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclkselx0`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSELX0")]
pub type CtimerclkselCtimerclkselx0 =
    crate::Reg<ctimerclksel_ctimerclkselx0::CtimerclkselCtimerclkselx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx0;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL1 (rw) register accessor: CTimer 1 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclksel1`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSEL1")]
pub type CtimerclkselCtimerclksel1 =
    crate::Reg<ctimerclksel_ctimerclksel1::CtimerclkselCtimerclksel1Spec>;
#[doc = "CTimer 1 clock source select"]
pub mod ctimerclksel_ctimerclksel1;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclkselx1`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSELX1")]
pub type CtimerclkselCtimerclkselx1 =
    crate::Reg<ctimerclksel_ctimerclkselx1::CtimerclkselCtimerclkselx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx1;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL2 (rw) register accessor: CTimer 2 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclksel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclksel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclksel2`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSEL2")]
pub type CtimerclkselCtimerclksel2 =
    crate::Reg<ctimerclksel_ctimerclksel2::CtimerclkselCtimerclksel2Spec>;
#[doc = "CTimer 2 clock source select"]
pub mod ctimerclksel_ctimerclksel2;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclkselx2`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSELX2")]
pub type CtimerclkselCtimerclkselx2 =
    crate::Reg<ctimerclksel_ctimerclkselx2::CtimerclkselCtimerclkselx2Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx2;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL3 (rw) register accessor: CTimer 3 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclksel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclksel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclksel3`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSEL3")]
pub type CtimerclkselCtimerclksel3 =
    crate::Reg<ctimerclksel_ctimerclksel3::CtimerclkselCtimerclksel3Spec>;
#[doc = "CTimer 3 clock source select"]
pub mod ctimerclksel_ctimerclksel3;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX3 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclkselx3`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSELX3")]
pub type CtimerclkselCtimerclkselx3 =
    crate::Reg<ctimerclksel_ctimerclkselx3::CtimerclkselCtimerclkselx3Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx3;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL4 (rw) register accessor: CTimer 4 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclksel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclksel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclksel4`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSEL4")]
pub type CtimerclkselCtimerclksel4 =
    crate::Reg<ctimerclksel_ctimerclksel4::CtimerclkselCtimerclksel4Spec>;
#[doc = "CTimer 4 clock source select"]
pub mod ctimerclksel_ctimerclksel4;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX4 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctimerclksel_ctimerclkselx4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctimerclksel_ctimerclkselx4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctimerclksel_ctimerclkselx4`]
module"]
#[doc(alias = "CTIMERCLKSEL_CTIMERCLKSELX4")]
pub type CtimerclkselCtimerclkselx4 =
    crate::Reg<ctimerclksel_ctimerclkselx4::CtimerclkselCtimerclkselx4Spec>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx4;
#[doc = "MAINCLKSELA (rw) register accessor: Main clock A source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclksela::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclksela::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclksela`]
module"]
#[doc(alias = "MAINCLKSELA")]
pub type Mainclksela = crate::Reg<mainclksela::MainclkselaSpec>;
#[doc = "Main clock A source select"]
pub mod mainclksela;
#[doc = "MAINCLKSELB (rw) register accessor: Main clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mainclkselb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mainclkselb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mainclkselb`]
module"]
#[doc(alias = "MAINCLKSELB")]
pub type Mainclkselb = crate::Reg<mainclkselb::MainclkselbSpec>;
#[doc = "Main clock source select"]
pub mod mainclkselb;
#[doc = "CLKOUTSEL (rw) register accessor: CLKOUT clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutsel`]
module"]
#[doc(alias = "CLKOUTSEL")]
pub type Clkoutsel = crate::Reg<clkoutsel::ClkoutselSpec>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "PLL0CLKSEL (rw) register accessor: PLL0 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0clksel`]
module"]
#[doc(alias = "PLL0CLKSEL")]
pub type Pll0clksel = crate::Reg<pll0clksel::Pll0clkselSpec>;
#[doc = "PLL0 clock source select"]
pub mod pll0clksel;
#[doc = "PLL1CLKSEL (rw) register accessor: PLL1 clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1clksel`]
module"]
#[doc(alias = "PLL1CLKSEL")]
pub type Pll1clksel = crate::Reg<pll1clksel::Pll1clkselSpec>;
#[doc = "PLL1 clock source select"]
pub mod pll1clksel;
#[doc = "ADCCLKSEL (rw) register accessor: ADC clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`adcclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclksel`]
module"]
#[doc(alias = "ADCCLKSEL")]
pub type Adcclksel = crate::Reg<adcclksel::AdcclkselSpec>;
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "USB0CLKSEL (rw) register accessor: FS USB clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0clksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0clksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0clksel`]
module"]
#[doc(alias = "USB0CLKSEL")]
pub type Usb0clksel = crate::Reg<usb0clksel::Usb0clkselSpec>;
#[doc = "FS USB clock source select"]
pub mod usb0clksel;
#[doc = "FCCLKSEL_FCCLKSEL0 (rw) register accessor: Flexcomm Interface 0 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel0`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL0")]
pub type FcclkselFcclksel0 = crate::Reg<fcclksel_fcclksel0::FcclkselFcclksel0Spec>;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel0;
#[doc = "FCCLKSEL_FCCLKSELX0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx0`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX0")]
pub type FcclkselFcclkselx0 = crate::Reg<fcclksel_fcclkselx0::FcclkselFcclkselx0Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx0;
#[doc = "FCCLKSEL_FCCLKSEL1 (rw) register accessor: Flexcomm Interface 1 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel1`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL1")]
pub type FcclkselFcclksel1 = crate::Reg<fcclksel_fcclksel1::FcclkselFcclksel1Spec>;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel1;
#[doc = "FCCLKSEL_FCCLKSELX1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx1`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX1")]
pub type FcclkselFcclkselx1 = crate::Reg<fcclksel_fcclkselx1::FcclkselFcclkselx1Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx1;
#[doc = "FCCLKSEL_FCCLKSEL2 (rw) register accessor: Flexcomm Interface 2 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel2`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL2")]
pub type FcclkselFcclksel2 = crate::Reg<fcclksel_fcclksel2::FcclkselFcclksel2Spec>;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel2;
#[doc = "FCCLKSEL_FCCLKSELX2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx2`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX2")]
pub type FcclkselFcclkselx2 = crate::Reg<fcclksel_fcclkselx2::FcclkselFcclkselx2Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx2;
#[doc = "FCCLKSEL_FCCLKSEL3 (rw) register accessor: Flexcomm Interface 3 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel3`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL3")]
pub type FcclkselFcclksel3 = crate::Reg<fcclksel_fcclksel3::FcclkselFcclksel3Spec>;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel3;
#[doc = "FCCLKSEL_FCCLKSELX3 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx3`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX3")]
pub type FcclkselFcclkselx3 = crate::Reg<fcclksel_fcclkselx3::FcclkselFcclkselx3Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx3;
#[doc = "FCCLKSEL_FCCLKSEL4 (rw) register accessor: Flexcomm Interface 4 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel4`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL4")]
pub type FcclkselFcclksel4 = crate::Reg<fcclksel_fcclksel4::FcclkselFcclksel4Spec>;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel4;
#[doc = "FCCLKSEL_FCCLKSELX4 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx4`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX4")]
pub type FcclkselFcclkselx4 = crate::Reg<fcclksel_fcclkselx4::FcclkselFcclkselx4Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx4;
#[doc = "FCCLKSEL_FCCLKSEL5 (rw) register accessor: Flexcomm Interface 5 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel5`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL5")]
pub type FcclkselFcclksel5 = crate::Reg<fcclksel_fcclksel5::FcclkselFcclksel5Spec>;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel5;
#[doc = "FCCLKSEL_FCCLKSELX5 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx5`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX5")]
pub type FcclkselFcclkselx5 = crate::Reg<fcclksel_fcclkselx5::FcclkselFcclkselx5Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx5;
#[doc = "FCCLKSEL_FCCLKSEL6 (rw) register accessor: Flexcomm Interface 6 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel6`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL6")]
pub type FcclkselFcclksel6 = crate::Reg<fcclksel_fcclksel6::FcclkselFcclksel6Spec>;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel6;
#[doc = "FCCLKSEL_FCCLKSELX6 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx6`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX6")]
pub type FcclkselFcclkselx6 = crate::Reg<fcclksel_fcclkselx6::FcclkselFcclkselx6Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx6;
#[doc = "FCCLKSEL_FCCLKSEL7 (rw) register accessor: Flexcomm Interface 7 clock source select for Fractional Rate Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclksel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclksel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclksel7`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSEL7")]
pub type FcclkselFcclksel7 = crate::Reg<fcclksel_fcclksel7::FcclkselFcclksel7Spec>;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel7;
#[doc = "FCCLKSEL_FCCLKSELX7 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcclksel_fcclkselx7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcclksel_fcclkselx7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcclksel_fcclkselx7`]
module"]
#[doc(alias = "FCCLKSEL_FCCLKSELX7")]
pub type FcclkselFcclkselx7 = crate::Reg<fcclksel_fcclkselx7::FcclkselFcclkselx7Spec>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx7;
#[doc = "HSLSPICLKSEL (rw) register accessor: HS LSPI clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`hslspiclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hslspiclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hslspiclksel`]
module"]
#[doc(alias = "HSLSPICLKSEL")]
pub type Hslspiclksel = crate::Reg<hslspiclksel::HslspiclkselSpec>;
#[doc = "HS LSPI clock source select"]
pub mod hslspiclksel;
#[doc = "MCLKCLKSEL (rw) register accessor: MCLK clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkclksel`]
module"]
#[doc(alias = "MCLKCLKSEL")]
pub type Mclkclksel = crate::Reg<mclkclksel::MclkclkselSpec>;
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "SCTCLKSEL (rw) register accessor: SCTimer/PWM clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`sctclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctclksel`]
module"]
#[doc(alias = "SCTCLKSEL")]
pub type Sctclksel = crate::Reg<sctclksel::SctclkselSpec>;
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "SDIOCLKSEL (rw) register accessor: SDIO clock source select\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioclksel`]
module"]
#[doc(alias = "SDIOCLKSEL")]
pub type Sdioclksel = crate::Reg<sdioclksel::SdioclkselSpec>;
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "SYSTICKCLKDIV0 (rw) register accessor: System Tick Timer divider for CPU0\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclkdiv0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclkdiv0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclkdiv0`]
module"]
#[doc(alias = "SYSTICKCLKDIV0")]
pub type Systickclkdiv0 = crate::Reg<systickclkdiv0::Systickclkdiv0Spec>;
#[doc = "System Tick Timer divider for CPU0"]
pub mod systickclkdiv0;
#[doc = "SYSTICKCLKDIV1 (rw) register accessor: System Tick Timer divider for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`systickclkdiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systickclkdiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systickclkdiv1`]
module"]
#[doc(alias = "SYSTICKCLKDIV1")]
pub type Systickclkdiv1 = crate::Reg<systickclkdiv1::Systickclkdiv1Spec>;
#[doc = "System Tick Timer divider for CPU1"]
pub mod systickclkdiv1;
#[doc = "TRACECLKDIV (rw) register accessor: TRACE clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclkdiv`]
module"]
#[doc(alias = "TRACECLKDIV")]
pub type Traceclkdiv = crate::Reg<traceclkdiv::TraceclkdivSpec>;
#[doc = "TRACE clock divider"]
pub mod traceclkdiv;
#[doc = "FLEXFRGCTRL_FLEXFRG0CTRL (rw) register accessor: Fractional rate divider for flexcomm 0\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg0ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG0CTRL")]
pub type FlexfrgctrlFlexfrg0ctrl =
    crate::Reg<flexfrgctrl_flexfrg0ctrl::FlexfrgctrlFlexfrg0ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 0"]
pub mod flexfrgctrl_flexfrg0ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL0 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl0`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL0")]
pub type FlexfrgctrlFlexfrgxctrl0 =
    crate::Reg<flexfrgctrl_flexfrgxctrl0::FlexfrgctrlFlexfrgxctrl0Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl0;
#[doc = "FLEXFRGCTRL_FLEXFRG1CTRL (rw) register accessor: Fractional rate divider for flexcomm 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg1ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG1CTRL")]
pub type FlexfrgctrlFlexfrg1ctrl =
    crate::Reg<flexfrgctrl_flexfrg1ctrl::FlexfrgctrlFlexfrg1ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 1"]
pub mod flexfrgctrl_flexfrg1ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL1 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl1`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL1")]
pub type FlexfrgctrlFlexfrgxctrl1 =
    crate::Reg<flexfrgctrl_flexfrgxctrl1::FlexfrgctrlFlexfrgxctrl1Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl1;
#[doc = "FLEXFRGCTRL_FLEXFRG2CTRL (rw) register accessor: Fractional rate divider for flexcomm 2\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg2ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG2CTRL")]
pub type FlexfrgctrlFlexfrg2ctrl =
    crate::Reg<flexfrgctrl_flexfrg2ctrl::FlexfrgctrlFlexfrg2ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 2"]
pub mod flexfrgctrl_flexfrg2ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL2 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl2`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL2")]
pub type FlexfrgctrlFlexfrgxctrl2 =
    crate::Reg<flexfrgctrl_flexfrgxctrl2::FlexfrgctrlFlexfrgxctrl2Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl2;
#[doc = "FLEXFRGCTRL_FLEXFRG3CTRL (rw) register accessor: Fractional rate divider for flexcomm 3\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg3ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG3CTRL")]
pub type FlexfrgctrlFlexfrg3ctrl =
    crate::Reg<flexfrgctrl_flexfrg3ctrl::FlexfrgctrlFlexfrg3ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 3"]
pub mod flexfrgctrl_flexfrg3ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL3 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl3`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL3")]
pub type FlexfrgctrlFlexfrgxctrl3 =
    crate::Reg<flexfrgctrl_flexfrgxctrl3::FlexfrgctrlFlexfrgxctrl3Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl3;
#[doc = "FLEXFRGCTRL_FLEXFRG4CTRL (rw) register accessor: Fractional rate divider for flexcomm 4\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg4ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG4CTRL")]
pub type FlexfrgctrlFlexfrg4ctrl =
    crate::Reg<flexfrgctrl_flexfrg4ctrl::FlexfrgctrlFlexfrg4ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 4"]
pub mod flexfrgctrl_flexfrg4ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL4 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl4`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL4")]
pub type FlexfrgctrlFlexfrgxctrl4 =
    crate::Reg<flexfrgctrl_flexfrgxctrl4::FlexfrgctrlFlexfrgxctrl4Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl4;
#[doc = "FLEXFRGCTRL_FLEXFRG5CTRL (rw) register accessor: Fractional rate divider for flexcomm 5\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg5ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg5ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg5ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG5CTRL")]
pub type FlexfrgctrlFlexfrg5ctrl =
    crate::Reg<flexfrgctrl_flexfrg5ctrl::FlexfrgctrlFlexfrg5ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 5"]
pub mod flexfrgctrl_flexfrg5ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL5 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl5`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL5")]
pub type FlexfrgctrlFlexfrgxctrl5 =
    crate::Reg<flexfrgctrl_flexfrgxctrl5::FlexfrgctrlFlexfrgxctrl5Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl5;
#[doc = "FLEXFRGCTRL_FLEXFRG6CTRL (rw) register accessor: Fractional rate divider for flexcomm 6\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg6ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg6ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg6ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG6CTRL")]
pub type FlexfrgctrlFlexfrg6ctrl =
    crate::Reg<flexfrgctrl_flexfrg6ctrl::FlexfrgctrlFlexfrg6ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 6"]
pub mod flexfrgctrl_flexfrg6ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL6 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl6`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL6")]
pub type FlexfrgctrlFlexfrgxctrl6 =
    crate::Reg<flexfrgctrl_flexfrgxctrl6::FlexfrgctrlFlexfrgxctrl6Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl6;
#[doc = "FLEXFRGCTRL_FLEXFRG7CTRL (rw) register accessor: Fractional rate divider for flexcomm 7\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrg7ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrg7ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrg7ctrl`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRG7CTRL")]
pub type FlexfrgctrlFlexfrg7ctrl =
    crate::Reg<flexfrgctrl_flexfrg7ctrl::FlexfrgctrlFlexfrg7ctrlSpec>;
#[doc = "Fractional rate divider for flexcomm 7"]
pub mod flexfrgctrl_flexfrg7ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL7 (rw) register accessor: Peripheral reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`flexfrgctrl_flexfrgxctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexfrgctrl_flexfrgxctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flexfrgctrl_flexfrgxctrl7`]
module"]
#[doc(alias = "FLEXFRGCTRL_FLEXFRGXCTRL7")]
pub type FlexfrgctrlFlexfrgxctrl7 =
    crate::Reg<flexfrgctrl_flexfrgxctrl7::FlexfrgctrlFlexfrgxctrl7Spec>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl7;
#[doc = "AHBCLKDIV (rw) register accessor: System clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbclkdiv`]
module"]
#[doc(alias = "AHBCLKDIV")]
pub type Ahbclkdiv = crate::Reg<ahbclkdiv::AhbclkdivSpec>;
#[doc = "System clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUTDIV (rw) register accessor: CLKOUT clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutdiv`]
module"]
#[doc(alias = "CLKOUTDIV")]
pub type Clkoutdiv = crate::Reg<clkoutdiv::ClkoutdivSpec>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FROHFDIV (rw) register accessor: FRO_HF (96MHz) clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`frohfdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frohfdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frohfdiv`]
module"]
#[doc(alias = "FROHFDIV")]
pub type Frohfdiv = crate::Reg<frohfdiv::FrohfdivSpec>;
#[doc = "FRO_HF (96MHz) clock divider"]
pub mod frohfdiv;
#[doc = "WDTCLKDIV (rw) register accessor: WDT clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtclkdiv`]
module"]
#[doc(alias = "WDTCLKDIV")]
pub type Wdtclkdiv = crate::Reg<wdtclkdiv::WdtclkdivSpec>;
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "ADCCLKDIV (rw) register accessor: ADC clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`adcclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclkdiv`]
module"]
#[doc(alias = "ADCCLKDIV")]
pub type Adcclkdiv = crate::Reg<adcclkdiv::AdcclkdivSpec>;
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0CLKDIV (rw) register accessor: USB0 Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0clkdiv`]
module"]
#[doc(alias = "USB0CLKDIV")]
pub type Usb0clkdiv = crate::Reg<usb0clkdiv::Usb0clkdivSpec>;
#[doc = "USB0 Clock divider"]
pub mod usb0clkdiv;
#[doc = "MCLKDIV (rw) register accessor: I2S MCLK clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkdiv`]
module"]
#[doc(alias = "MCLKDIV")]
pub type Mclkdiv = crate::Reg<mclkdiv::MclkdivSpec>;
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "SCTCLKDIV (rw) register accessor: SCT/PWM clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sctclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sctclkdiv`]
module"]
#[doc(alias = "SCTCLKDIV")]
pub type Sctclkdiv = crate::Reg<sctclkdiv::SctclkdivSpec>;
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "SDIOCLKDIV (rw) register accessor: SDIO clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioclkdiv`]
module"]
#[doc(alias = "SDIOCLKDIV")]
pub type Sdioclkdiv = crate::Reg<sdioclkdiv::SdioclkdivSpec>;
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "PLL0CLKDIV (rw) register accessor: PLL0 clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0clkdiv`]
module"]
#[doc(alias = "PLL0CLKDIV")]
pub type Pll0clkdiv = crate::Reg<pll0clkdiv::Pll0clkdivSpec>;
#[doc = "PLL0 clock divider"]
pub mod pll0clkdiv;
#[doc = "CLOCKGENUPDATELOCKOUT (rw) register accessor: Control clock configuration registers access (like xxxDIV, xxxSEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`clockgenupdatelockout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockgenupdatelockout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clockgenupdatelockout`]
module"]
#[doc(alias = "CLOCKGENUPDATELOCKOUT")]
pub type Clockgenupdatelockout = crate::Reg<clockgenupdatelockout::ClockgenupdatelockoutSpec>;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMCCR (rw) register accessor: FMC configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmccr`]
module"]
#[doc(alias = "FMCCR")]
pub type Fmccr = crate::Reg<fmccr::FmccrSpec>;
#[doc = "FMC configuration register"]
pub mod fmccr;
#[doc = "USB0NEEDCLKCTRL (rw) register accessor: USB0 need clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0needclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0needclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0needclkctrl`]
module"]
#[doc(alias = "USB0NEEDCLKCTRL")]
pub type Usb0needclkctrl = crate::Reg<usb0needclkctrl::Usb0needclkctrlSpec>;
#[doc = "USB0 need clock control"]
pub mod usb0needclkctrl;
#[doc = "USB0NEEDCLKSTAT (rw) register accessor: USB0 need clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usb0needclkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb0needclkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb0needclkstat`]
module"]
#[doc(alias = "USB0NEEDCLKSTAT")]
pub type Usb0needclkstat = crate::Reg<usb0needclkstat::Usb0needclkstatSpec>;
#[doc = "USB0 need clock status"]
pub mod usb0needclkstat;
#[doc = "FMCFLUSH (w) register accessor: FMCflush control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcflush::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcflush`]
module"]
#[doc(alias = "FMCFLUSH")]
pub type Fmcflush = crate::Reg<fmcflush::FmcflushSpec>;
#[doc = "FMCflush control"]
pub mod fmcflush;
#[doc = "MCLKIO (rw) register accessor: MCLK control\n\nYou can [`read`](crate::Reg::read) this register and get [`mclkio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mclkio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mclkio`]
module"]
#[doc(alias = "MCLKIO")]
pub type Mclkio = crate::Reg<mclkio::MclkioSpec>;
#[doc = "MCLK control"]
pub mod mclkio;
#[doc = "USB1NEEDCLKCTRL (rw) register accessor: USB1 need clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1needclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1needclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1needclkctrl`]
module"]
#[doc(alias = "USB1NEEDCLKCTRL")]
pub type Usb1needclkctrl = crate::Reg<usb1needclkctrl::Usb1needclkctrlSpec>;
#[doc = "USB1 need clock control"]
pub mod usb1needclkctrl;
#[doc = "USB1NEEDCLKSTAT (rw) register accessor: USB1 need clock status\n\nYou can [`read`](crate::Reg::read) this register and get [`usb1needclkstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb1needclkstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb1needclkstat`]
module"]
#[doc(alias = "USB1NEEDCLKSTAT")]
pub type Usb1needclkstat = crate::Reg<usb1needclkstat::Usb1needclkstatSpec>;
#[doc = "USB1 need clock status"]
pub mod usb1needclkstat;
#[doc = "SDIOCLKCTRL (rw) register accessor: SDIO CCLKIN phase and delay control\n\nYou can [`read`](crate::Reg::read) this register and get [`sdioclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdioclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdioclkctrl`]
module"]
#[doc(alias = "SDIOCLKCTRL")]
pub type Sdioclkctrl = crate::Reg<sdioclkctrl::SdioclkctrlSpec>;
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "PLL1CTRL (rw) register accessor: PLL1 550m control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1ctrl`]
module"]
#[doc(alias = "PLL1CTRL")]
pub type Pll1ctrl = crate::Reg<pll1ctrl::Pll1ctrlSpec>;
#[doc = "PLL1 550m control"]
pub mod pll1ctrl;
#[doc = "PLL1STAT (rw) register accessor: PLL1 550m status\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1stat`]
module"]
#[doc(alias = "PLL1STAT")]
pub type Pll1stat = crate::Reg<pll1stat::Pll1statSpec>;
#[doc = "PLL1 550m status"]
pub mod pll1stat;
#[doc = "PLL1NDEC (rw) register accessor: PLL1 550m N divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1ndec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1ndec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1ndec`]
module"]
#[doc(alias = "PLL1NDEC")]
pub type Pll1ndec = crate::Reg<pll1ndec::Pll1ndecSpec>;
#[doc = "PLL1 550m N divider"]
pub mod pll1ndec;
#[doc = "PLL1MDEC (rw) register accessor: PLL1 550m M divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1mdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1mdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1mdec`]
module"]
#[doc(alias = "PLL1MDEC")]
pub type Pll1mdec = crate::Reg<pll1mdec::Pll1mdecSpec>;
#[doc = "PLL1 550m M divider"]
pub mod pll1mdec;
#[doc = "PLL1PDEC (rw) register accessor: PLL1 550m P divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1pdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1pdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1pdec`]
module"]
#[doc(alias = "PLL1PDEC")]
pub type Pll1pdec = crate::Reg<pll1pdec::Pll1pdecSpec>;
#[doc = "PLL1 550m P divider"]
pub mod pll1pdec;
#[doc = "PLL0CTRL (rw) register accessor: PLL0 550m control\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0ctrl`]
module"]
#[doc(alias = "PLL0CTRL")]
pub type Pll0ctrl = crate::Reg<pll0ctrl::Pll0ctrlSpec>;
#[doc = "PLL0 550m control"]
pub mod pll0ctrl;
#[doc = "PLL0STAT (rw) register accessor: PLL0 550m status\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0stat`]
module"]
#[doc(alias = "PLL0STAT")]
pub type Pll0stat = crate::Reg<pll0stat::Pll0statSpec>;
#[doc = "PLL0 550m status"]
pub mod pll0stat;
#[doc = "PLL0NDEC (rw) register accessor: PLL0 550m N divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0ndec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0ndec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0ndec`]
module"]
#[doc(alias = "PLL0NDEC")]
pub type Pll0ndec = crate::Reg<pll0ndec::Pll0ndecSpec>;
#[doc = "PLL0 550m N divider"]
pub mod pll0ndec;
#[doc = "PLL0PDEC (rw) register accessor: PLL0 550m P divider\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0pdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0pdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0pdec`]
module"]
#[doc(alias = "PLL0PDEC")]
pub type Pll0pdec = crate::Reg<pll0pdec::Pll0pdecSpec>;
#[doc = "PLL0 550m P divider"]
pub mod pll0pdec;
#[doc = "PLL0SSCG0 (rw) register accessor: PLL0 Spread Spectrum Wrapper control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0sscg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0sscg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0sscg0`]
module"]
#[doc(alias = "PLL0SSCG0")]
pub type Pll0sscg0 = crate::Reg<pll0sscg0::Pll0sscg0Spec>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub mod pll0sscg0;
#[doc = "PLL0SSCG1 (rw) register accessor: PLL0 Spread Spectrum Wrapper control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pll0sscg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll0sscg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll0sscg1`]
module"]
#[doc(alias = "PLL0SSCG1")]
pub type Pll0sscg1 = crate::Reg<pll0sscg1::Pll0sscg1Spec>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub mod pll0sscg1;
#[doc = "FUNCRETENTIONCTRL (rw) register accessor: Functional retention control register\n\nYou can [`read`](crate::Reg::read) this register and get [`funcretentionctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`funcretentionctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@funcretentionctrl`]
module"]
#[doc(alias = "FUNCRETENTIONCTRL")]
pub type Funcretentionctrl = crate::Reg<funcretentionctrl::FuncretentionctrlSpec>;
#[doc = "Functional retention control register"]
pub mod funcretentionctrl;
#[doc = "CPUCTRL (rw) register accessor: CPU Control for multiple processors\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuctrl`]
module"]
#[doc(alias = "CPUCTRL")]
pub type Cpuctrl = crate::Reg<cpuctrl::CpuctrlSpec>;
#[doc = "CPU Control for multiple processors"]
pub mod cpuctrl;
#[doc = "CPBOOT (rw) register accessor: Coprocessor Boot Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cpboot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpboot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpboot`]
module"]
#[doc(alias = "CPBOOT")]
pub type Cpboot = crate::Reg<cpboot::CpbootSpec>;
#[doc = "Coprocessor Boot Address"]
pub mod cpboot;
#[doc = "CPSTAT (rw) register accessor: CPU Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpstat`]
module"]
#[doc(alias = "CPSTAT")]
pub type Cpstat = crate::Reg<cpstat::CpstatSpec>;
#[doc = "CPU Status"]
pub mod cpstat;
#[doc = "CLOCK_CTRL (rw) register accessor: Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
#[doc(alias = "CLOCK_CTRL")]
pub type ClockCtrl = crate::Reg<clock_ctrl::ClockCtrlSpec>;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub mod clock_ctrl;
#[doc = "COMP_INT_CTRL (rw) register accessor: Comparator Interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_int_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_int_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_int_ctrl`]
module"]
#[doc(alias = "COMP_INT_CTRL")]
pub type CompIntCtrl = crate::Reg<comp_int_ctrl::CompIntCtrlSpec>;
#[doc = "Comparator Interrupt control"]
pub mod comp_int_ctrl;
#[doc = "COMP_INT_STATUS (rw) register accessor: Comparator Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`comp_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_int_status`]
module"]
#[doc(alias = "COMP_INT_STATUS")]
pub type CompIntStatus = crate::Reg<comp_int_status::CompIntStatusSpec>;
#[doc = "Comparator Interrupt status"]
pub mod comp_int_status;
#[doc = "AUTOCLKGATEOVERRIDE (rw) register accessor: Control automatic clock gating\n\nYou can [`read`](crate::Reg::read) this register and get [`autoclkgateoverride::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autoclkgateoverride::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autoclkgateoverride`]
module"]
#[doc(alias = "AUTOCLKGATEOVERRIDE")]
pub type Autoclkgateoverride = crate::Reg<autoclkgateoverride::AutoclkgateoverrideSpec>;
#[doc = "Control automatic clock gating"]
pub mod autoclkgateoverride;
#[doc = "GPIOPSYNC (rw) register accessor: Enable bypass of the first stage of synchonization inside GPIO_INT module\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiopsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiopsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpiopsync`]
module"]
#[doc(alias = "GPIOPSYNC")]
pub type Gpiopsync = crate::Reg<gpiopsync::GpiopsyncSpec>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub mod gpiopsync;
#[doc = "DEBUG_LOCK_EN (rw) register accessor: Control write access to security registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_lock_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_lock_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_lock_en`]
module"]
#[doc(alias = "DEBUG_LOCK_EN")]
pub type DebugLockEn = crate::Reg<debug_lock_en::DebugLockEnSpec>;
#[doc = "Control write access to security registers."]
pub mod debug_lock_en;
#[doc = "DEBUG_FEATURES (rw) register accessor: Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_features::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_features::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_features`]
module"]
#[doc(alias = "DEBUG_FEATURES")]
pub type DebugFeatures = crate::Reg<debug_features::DebugFeaturesSpec>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
pub mod debug_features;
#[doc = "DEBUG_FEATURES_DP (rw) register accessor: Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_features_dp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_features_dp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_features_dp`]
module"]
#[doc(alias = "DEBUG_FEATURES_DP")]
pub type DebugFeaturesDp = crate::Reg<debug_features_dp::DebugFeaturesDpSpec>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
pub mod debug_features_dp;
#[doc = "KEY_BLOCK (w) register accessor: block quiddikey/PUF all index.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_block::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_block`]
module"]
#[doc(alias = "KEY_BLOCK")]
pub type KeyBlock = crate::Reg<key_block::KeyBlockSpec>;
#[doc = "block quiddikey/PUF all index."]
pub mod key_block;
#[doc = "DEBUG_AUTH_BEACON (rw) register accessor: Debug authentication BEACON register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_auth_beacon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_auth_beacon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_auth_beacon`]
module"]
#[doc(alias = "DEBUG_AUTH_BEACON")]
pub type DebugAuthBeacon = crate::Reg<debug_auth_beacon::DebugAuthBeaconSpec>;
#[doc = "Debug authentication BEACON register"]
pub mod debug_auth_beacon;
#[doc = "CPUCFG (rw) register accessor: CPUs configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucfg`]
module"]
#[doc(alias = "CPUCFG")]
pub type Cpucfg = crate::Reg<cpucfg::CpucfgSpec>;
#[doc = "CPUs configuration register"]
pub mod cpucfg;
#[doc = "DEVICE_ID0 (r) register accessor: Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@device_id0`]
module"]
#[doc(alias = "DEVICE_ID0")]
pub type DeviceId0 = crate::Reg<device_id0::DeviceId0Spec>;
#[doc = "Device ID"]
pub mod device_id0;
#[doc = "DIEID (r) register accessor: Chip revision ID and Number\n\nYou can [`read`](crate::Reg::read) this register and get [`dieid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieid`]
module"]
#[doc(alias = "DIEID")]
pub type Dieid = crate::Reg<dieid::DieidSpec>;
#[doc = "Chip revision ID and Number"]
pub mod dieid;
