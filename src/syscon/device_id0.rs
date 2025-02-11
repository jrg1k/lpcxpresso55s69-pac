#[doc = "Register `DEVICE_ID0` reader"]
pub type R = crate::R<DeviceId0Spec>;
#[doc = "Field `ROM_REV_MINOR` reader - ROM revision."]
pub type RomRevMinorR = crate::FieldReader;
impl R {
    #[doc = "Bits 20:23 - ROM revision."]
    #[inline(always)]
    pub fn rom_rev_minor(&self) -> RomRevMinorR {
        RomRevMinorR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Device ID\n\nYou can [`read`](crate::Reg::read) this register and get [`device_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceId0Spec;
impl crate::RegisterSpec for DeviceId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_id0::R`](R) reader structure"]
impl crate::Readable for DeviceId0Spec {}
#[doc = "`reset()` method sets DEVICE_ID0 to value 0"]
impl crate::Resettable for DeviceId0Spec {
    const RESET_VALUE: u32 = 0;
}
