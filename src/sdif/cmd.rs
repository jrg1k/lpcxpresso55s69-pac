#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CMD_INDEX` reader - Command index."]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command index."]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESPONSE_EXPECT` reader - Response expect."]
pub type ResponseExpectR = crate::BitReader;
#[doc = "Field `RESPONSE_EXPECT` writer - Response expect."]
pub type ResponseExpectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPONSE_LENGTH` reader - Response length."]
pub type ResponseLengthR = crate::BitReader;
#[doc = "Field `RESPONSE_LENGTH` writer - Response length."]
pub type ResponseLengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_RESPONSE_CRC` reader - Check response CRC."]
pub type CheckResponseCrcR = crate::BitReader;
#[doc = "Field `CHECK_RESPONSE_CRC` writer - Check response CRC."]
pub type CheckResponseCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_EXPECTED` reader - Data expected."]
pub type DataExpectedR = crate::BitReader;
#[doc = "Field `DATA_EXPECTED` writer - Data expected."]
pub type DataExpectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_WRITE` reader - read/write."]
pub type ReadWriteR = crate::BitReader;
#[doc = "Field `READ_WRITE` writer - read/write."]
pub type ReadWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFER_MODE` reader - Transfer mode."]
pub type TransferModeR = crate::BitReader;
#[doc = "Field `TRANSFER_MODE` writer - Transfer mode."]
pub type TransferModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_AUTO_STOP` reader - Send auto stop."]
pub type SendAutoStopR = crate::BitReader;
#[doc = "Field `SEND_AUTO_STOP` writer - Send auto stop."]
pub type SendAutoStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - Wait prvdata complete."]
pub type WaitPrvdataCompleteR = crate::BitReader;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - Wait prvdata complete."]
pub type WaitPrvdataCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ABORT_CMD` reader - Stop abort command."]
pub type StopAbortCmdR = crate::BitReader;
#[doc = "Field `STOP_ABORT_CMD` writer - Stop abort command."]
pub type StopAbortCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_INITIALIZATION` reader - Send initialization."]
pub type SendInitializationR = crate::BitReader;
#[doc = "Field `SEND_INITIALIZATION` writer - Send initialization."]
pub type SendInitializationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Specifies the card number of SDCARD for which the current Command is being executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CardNumber {
    #[doc = "0: Command will be execute on SDCARD 0"]
    Card0 = 0,
    #[doc = "1: Command will be execute on SDCARD 1"]
    Card1 = 1,
}
impl From<CardNumber> for u8 {
    #[inline(always)]
    fn from(variant: CardNumber) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CardNumber {
    type Ux = u8;
}
impl crate::IsEnum for CardNumber {}
#[doc = "Field `CARD_NUMBER` reader - Specifies the card number of SDCARD for which the current Command is being executed"]
pub type CardNumberR = crate::FieldReader<CardNumber>;
impl CardNumberR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CardNumber> {
        match self.bits {
            0 => Some(CardNumber::Card0),
            1 => Some(CardNumber::Card1),
            _ => None,
        }
    }
    #[doc = "Command will be execute on SDCARD 0"]
    #[inline(always)]
    pub fn is_card0(&self) -> bool {
        *self == CardNumber::Card0
    }
    #[doc = "Command will be execute on SDCARD 1"]
    #[inline(always)]
    pub fn is_card1(&self) -> bool {
        *self == CardNumber::Card1
    }
}
#[doc = "Field `CARD_NUMBER` writer - Specifies the card number of SDCARD for which the current Command is being executed"]
pub type CardNumberW<'a, REG> = crate::FieldWriter<'a, REG, 5, CardNumber>;
impl<'a, REG> CardNumberW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Command will be execute on SDCARD 0"]
    #[inline(always)]
    pub fn card0(self) -> &'a mut crate::W<REG> {
        self.variant(CardNumber::Card0)
    }
    #[doc = "Command will be execute on SDCARD 1"]
    #[inline(always)]
    pub fn card1(self) -> &'a mut crate::W<REG> {
        self.variant(CardNumber::Card1)
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - Update clock registers only."]
pub type UpdateClockRegistersOnlyR = crate::BitReader;
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - Update clock registers only."]
pub type UpdateClockRegistersOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_CEATA_DEVICE` reader - Read ceata device."]
pub type ReadCeataDeviceR = crate::BitReader;
#[doc = "Field `READ_CEATA_DEVICE` writer - Read ceata device."]
pub type ReadCeataDeviceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCS_EXPECTED` reader - CCS expected."]
pub type CcsExpectedR = crate::BitReader;
#[doc = "Field `CCS_EXPECTED` writer - CCS expected."]
pub type CcsExpectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_BOOT` reader - Enable Boot - this bit should be set only for mandatory boot mode."]
pub type EnableBootR = crate::BitReader;
#[doc = "Field `ENABLE_BOOT` writer - Enable Boot - this bit should be set only for mandatory boot mode."]
pub type EnableBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXPECT_BOOT_ACK` reader - Expect Boot Acknowledge."]
pub type ExpectBootAckR = crate::BitReader;
#[doc = "Field `EXPECT_BOOT_ACK` writer - Expect Boot Acknowledge."]
pub type ExpectBootAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_BOOT` reader - Disable Boot."]
pub type DisableBootR = crate::BitReader;
#[doc = "Field `DISABLE_BOOT` writer - Disable Boot."]
pub type DisableBootW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_MODE` reader - Boot Mode."]
pub type BootModeR = crate::BitReader;
#[doc = "Field `BOOT_MODE` writer - Boot Mode."]
pub type BootModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VOLT_SWITCH` reader - Voltage switch bit."]
pub type VoltSwitchR = crate::BitReader;
#[doc = "Field `VOLT_SWITCH` writer - Voltage switch bit."]
pub type VoltSwitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_HOLD_REG` reader - Use Hold Register."]
pub type UseHoldRegR = crate::BitReader;
#[doc = "Field `USE_HOLD_REG` writer - Use Hold Register."]
pub type UseHoldRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_CMD` reader - Start command."]
pub type StartCmdR = crate::BitReader;
#[doc = "Field `START_CMD` writer - Start command."]
pub type StartCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&self) -> ResponseExpectR {
        ResponseExpectR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&self) -> ResponseLengthR {
        ResponseLengthR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CheckResponseCrcR {
        CheckResponseCrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DataExpectedR {
        DataExpectedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&self) -> ReadWriteR {
        ReadWriteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TransferModeR {
        TransferModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SendAutoStopR {
        SendAutoStopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WaitPrvdataCompleteR {
        WaitPrvdataCompleteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> StopAbortCmdR {
        StopAbortCmdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SendInitializationR {
        SendInitializationR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline(always)]
    pub fn card_number(&self) -> CardNumberR {
        CardNumberR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UpdateClockRegistersOnlyR {
        UpdateClockRegistersOnlyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> ReadCeataDeviceR {
        ReadCeataDeviceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CcsExpectedR {
        CcsExpectedR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&self) -> EnableBootR {
        EnableBootR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&self) -> ExpectBootAckR {
        ExpectBootAckR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&self) -> DisableBootR {
        DisableBootR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&self) -> VoltSwitchR {
        VoltSwitchR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&self) -> UseHoldRegR {
        UseHoldRegR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&self) -> StartCmdR {
        StartCmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&mut self) -> CmdIndexW<CmdSpec> {
        CmdIndexW::new(self, 0)
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&mut self) -> ResponseExpectW<CmdSpec> {
        ResponseExpectW::new(self, 6)
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&mut self) -> ResponseLengthW<CmdSpec> {
        ResponseLengthW::new(self, 7)
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&mut self) -> CheckResponseCrcW<CmdSpec> {
        CheckResponseCrcW::new(self, 8)
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&mut self) -> DataExpectedW<CmdSpec> {
        DataExpectedW::new(self, 9)
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&mut self) -> ReadWriteW<CmdSpec> {
        ReadWriteW::new(self, 10)
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&mut self) -> TransferModeW<CmdSpec> {
        TransferModeW::new(self, 11)
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&mut self) -> SendAutoStopW<CmdSpec> {
        SendAutoStopW::new(self, 12)
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&mut self) -> WaitPrvdataCompleteW<CmdSpec> {
        WaitPrvdataCompleteW::new(self, 13)
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&mut self) -> StopAbortCmdW<CmdSpec> {
        StopAbortCmdW::new(self, 14)
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&mut self) -> SendInitializationW<CmdSpec> {
        SendInitializationW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline(always)]
    pub fn card_number(&mut self) -> CardNumberW<CmdSpec> {
        CardNumberW::new(self, 16)
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&mut self) -> UpdateClockRegistersOnlyW<CmdSpec> {
        UpdateClockRegistersOnlyW::new(self, 21)
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&mut self) -> ReadCeataDeviceW<CmdSpec> {
        ReadCeataDeviceW::new(self, 22)
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&mut self) -> CcsExpectedW<CmdSpec> {
        CcsExpectedW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&mut self) -> EnableBootW<CmdSpec> {
        EnableBootW::new(self, 24)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&mut self) -> ExpectBootAckW<CmdSpec> {
        ExpectBootAckW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&mut self) -> DisableBootW<CmdSpec> {
        DisableBootW::new(self, 26)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BootModeW<CmdSpec> {
        BootModeW::new(self, 27)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&mut self) -> VoltSwitchW<CmdSpec> {
        VoltSwitchW::new(self, 28)
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&mut self) -> UseHoldRegW<CmdSpec> {
        UseHoldRegW::new(self, 29)
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&mut self) -> StartCmdW<CmdSpec> {
        StartCmdW::new(self, 31)
    }
}
#[doc = "Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
