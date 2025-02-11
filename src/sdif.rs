#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    pwren: Pwren,
    clkdiv: Clkdiv,
    _reserved3: [u8; 0x04],
    clkena: Clkena,
    tmout: Tmout,
    ctype: Ctype,
    blksiz: Blksiz,
    bytcnt: Bytcnt,
    intmask: Intmask,
    cmdarg: Cmdarg,
    cmd: Cmd,
    resp: [Resp; 4],
    mintsts: Mintsts,
    rintsts: Rintsts,
    status: Status,
    fifoth: Fifoth,
    cdetect: Cdetect,
    wrtprt: Wrtprt,
    _reserved18: [u8; 0x04],
    tcbcnt: Tcbcnt,
    tbbcnt: Tbbcnt,
    debnce: Debnce,
    _reserved21: [u8; 0x10],
    rst_n: RstN,
    _reserved22: [u8; 0x04],
    bmod: Bmod,
    pldmnd: Pldmnd,
    dbaddr: Dbaddr,
    idsts: Idsts,
    idinten: Idinten,
    dscaddr: Dscaddr,
    bufaddr: Bufaddr,
    _reserved29: [u8; 0x64],
    cardthrctl: Cardthrctl,
    backendpwr: Backendpwr,
    _reserved31: [u8; 0xf8],
    fifo: [Fifo; 64],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Power Enable register"]
    #[inline(always)]
    pub const fn pwren(&self) -> &Pwren {
        &self.pwren
    }
    #[doc = "0x08 - Clock Divider register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x10 - Clock Enable register"]
    #[inline(always)]
    pub const fn clkena(&self) -> &Clkena {
        &self.clkena
    }
    #[doc = "0x14 - Time-out register"]
    #[inline(always)]
    pub const fn tmout(&self) -> &Tmout {
        &self.tmout
    }
    #[doc = "0x18 - Card Type register"]
    #[inline(always)]
    pub const fn ctype(&self) -> &Ctype {
        &self.ctype
    }
    #[doc = "0x1c - Block Size register"]
    #[inline(always)]
    pub const fn blksiz(&self) -> &Blksiz {
        &self.blksiz
    }
    #[doc = "0x20 - Byte Count register"]
    #[inline(always)]
    pub const fn bytcnt(&self) -> &Bytcnt {
        &self.bytcnt
    }
    #[doc = "0x24 - Interrupt Mask register"]
    #[inline(always)]
    pub const fn intmask(&self) -> &Intmask {
        &self.intmask
    }
    #[doc = "0x28 - Command Argument register"]
    #[inline(always)]
    pub const fn cmdarg(&self) -> &Cmdarg {
        &self.cmdarg
    }
    #[doc = "0x2c - Command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x30..0x40 - Response register"]
    #[inline(always)]
    pub const fn resp(&self, n: usize) -> &Resp {
        &self.resp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Response register"]
    #[inline(always)]
    pub fn resp_iter(&self) -> impl Iterator<Item = &Resp> {
        self.resp.iter()
    }
    #[doc = "0x40 - Masked Interrupt Status register"]
    #[inline(always)]
    pub const fn mintsts(&self) -> &Mintsts {
        &self.mintsts
    }
    #[doc = "0x44 - Raw Interrupt Status register"]
    #[inline(always)]
    pub const fn rintsts(&self) -> &Rintsts {
        &self.rintsts
    }
    #[doc = "0x48 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x4c - FIFO Threshold Watermark register"]
    #[inline(always)]
    pub const fn fifoth(&self) -> &Fifoth {
        &self.fifoth
    }
    #[doc = "0x50 - Card Detect register"]
    #[inline(always)]
    pub const fn cdetect(&self) -> &Cdetect {
        &self.cdetect
    }
    #[doc = "0x54 - Write Protect register"]
    #[inline(always)]
    pub const fn wrtprt(&self) -> &Wrtprt {
        &self.wrtprt
    }
    #[doc = "0x5c - Transferred CIU Card Byte Count register"]
    #[inline(always)]
    pub const fn tcbcnt(&self) -> &Tcbcnt {
        &self.tcbcnt
    }
    #[doc = "0x60 - Transferred Host to BIU-FIFO Byte Count register"]
    #[inline(always)]
    pub const fn tbbcnt(&self) -> &Tbbcnt {
        &self.tbbcnt
    }
    #[doc = "0x64 - Debounce Count register"]
    #[inline(always)]
    pub const fn debnce(&self) -> &Debnce {
        &self.debnce
    }
    #[doc = "0x78 - Hardware Reset"]
    #[inline(always)]
    pub const fn rst_n(&self) -> &RstN {
        &self.rst_n
    }
    #[doc = "0x80 - Bus Mode register"]
    #[inline(always)]
    pub const fn bmod(&self) -> &Bmod {
        &self.bmod
    }
    #[doc = "0x84 - Poll Demand register"]
    #[inline(always)]
    pub const fn pldmnd(&self) -> &Pldmnd {
        &self.pldmnd
    }
    #[doc = "0x88 - Descriptor List Base Address register"]
    #[inline(always)]
    pub const fn dbaddr(&self) -> &Dbaddr {
        &self.dbaddr
    }
    #[doc = "0x8c - Internal DMAC Status register"]
    #[inline(always)]
    pub const fn idsts(&self) -> &Idsts {
        &self.idsts
    }
    #[doc = "0x90 - Internal DMAC Interrupt Enable register"]
    #[inline(always)]
    pub const fn idinten(&self) -> &Idinten {
        &self.idinten
    }
    #[doc = "0x94 - Current Host Descriptor Address register"]
    #[inline(always)]
    pub const fn dscaddr(&self) -> &Dscaddr {
        &self.dscaddr
    }
    #[doc = "0x98 - Current Buffer Descriptor Address register"]
    #[inline(always)]
    pub const fn bufaddr(&self) -> &Bufaddr {
        &self.bufaddr
    }
    #[doc = "0x100 - Card Threshold Control"]
    #[inline(always)]
    pub const fn cardthrctl(&self) -> &Cardthrctl {
        &self.cardthrctl
    }
    #[doc = "0x104 - Power control"]
    #[inline(always)]
    pub const fn backendpwr(&self) -> &Backendpwr {
        &self.backendpwr
    }
    #[doc = "0x200..0x300 - SDIF FIFO"]
    #[inline(always)]
    pub const fn fifo(&self, n: usize) -> &Fifo {
        &self.fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x300 - SDIF FIFO"]
    #[inline(always)]
    pub fn fifo_iter(&self) -> impl Iterator<Item = &Fifo> {
        self.fifo.iter()
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "PWREN (rw) register accessor: Power Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwren`]
module"]
#[doc(alias = "PWREN")]
pub type Pwren = crate::Reg<pwren::PwrenSpec>;
#[doc = "Power Enable register"]
pub mod pwren;
#[doc = "CLKDIV (rw) register accessor: Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`]
module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "CLKENA (rw) register accessor: Clock Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkena`]
module"]
#[doc(alias = "CLKENA")]
pub type Clkena = crate::Reg<clkena::ClkenaSpec>;
#[doc = "Clock Enable register"]
pub mod clkena;
#[doc = "TMOUT (rw) register accessor: Time-out register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmout`]
module"]
#[doc(alias = "TMOUT")]
pub type Tmout = crate::Reg<tmout::TmoutSpec>;
#[doc = "Time-out register"]
pub mod tmout;
#[doc = "CTYPE (rw) register accessor: Card Type register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctype::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctype::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctype`]
module"]
#[doc(alias = "CTYPE")]
pub type Ctype = crate::Reg<ctype::CtypeSpec>;
#[doc = "Card Type register"]
pub mod ctype;
#[doc = "BLKSIZ (rw) register accessor: Block Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`blksiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blksiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blksiz`]
module"]
#[doc(alias = "BLKSIZ")]
pub type Blksiz = crate::Reg<blksiz::BlksizSpec>;
#[doc = "Block Size register"]
pub mod blksiz;
#[doc = "BYTCNT (rw) register accessor: Byte Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`bytcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bytcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bytcnt`]
module"]
#[doc(alias = "BYTCNT")]
pub type Bytcnt = crate::Reg<bytcnt::BytcntSpec>;
#[doc = "Byte Count register"]
pub mod bytcnt;
#[doc = "INTMASK (rw) register accessor: Interrupt Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmask`]
module"]
#[doc(alias = "INTMASK")]
pub type Intmask = crate::Reg<intmask::IntmaskSpec>;
#[doc = "Interrupt Mask register"]
pub mod intmask;
#[doc = "CMDARG (rw) register accessor: Command Argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdarg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdarg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdarg`]
module"]
#[doc(alias = "CMDARG")]
pub type Cmdarg = crate::Reg<cmdarg::CmdargSpec>;
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "CMD (rw) register accessor: Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "RESP (rw) register accessor: Response register\n\nYou can [`read`](crate::Reg::read) this register and get [`resp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp`]
module"]
#[doc(alias = "RESP")]
pub type Resp = crate::Reg<resp::RespSpec>;
#[doc = "Response register"]
pub mod resp;
#[doc = "MINTSTS (rw) register accessor: Masked Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintsts`]
module"]
#[doc(alias = "MINTSTS")]
pub type Mintsts = crate::Reg<mintsts::MintstsSpec>;
#[doc = "Masked Interrupt Status register"]
pub mod mintsts;
#[doc = "RINTSTS (rw) register accessor: Raw Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rintsts`]
module"]
#[doc(alias = "RINTSTS")]
pub type Rintsts = crate::Reg<rintsts::RintstsSpec>;
#[doc = "Raw Interrupt Status register"]
pub mod rintsts;
#[doc = "STATUS (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "FIFOTH (rw) register accessor: FIFO Threshold Watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoth::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoth::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifoth`]
module"]
#[doc(alias = "FIFOTH")]
pub type Fifoth = crate::Reg<fifoth::FifothSpec>;
#[doc = "FIFO Threshold Watermark register"]
pub mod fifoth;
#[doc = "CDETECT (rw) register accessor: Card Detect register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdetect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdetect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdetect`]
module"]
#[doc(alias = "CDETECT")]
pub type Cdetect = crate::Reg<cdetect::CdetectSpec>;
#[doc = "Card Detect register"]
pub mod cdetect;
#[doc = "WRTPRT (rw) register accessor: Write Protect register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrtprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrtprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrtprt`]
module"]
#[doc(alias = "WRTPRT")]
pub type Wrtprt = crate::Reg<wrtprt::WrtprtSpec>;
#[doc = "Write Protect register"]
pub mod wrtprt;
#[doc = "TCBCNT (rw) register accessor: Transferred CIU Card Byte Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcbcnt`]
module"]
#[doc(alias = "TCBCNT")]
pub type Tcbcnt = crate::Reg<tcbcnt::TcbcntSpec>;
#[doc = "Transferred CIU Card Byte Count register"]
pub mod tcbcnt;
#[doc = "TBBCNT (rw) register accessor: Transferred Host to BIU-FIFO Byte Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbbcnt`]
module"]
#[doc(alias = "TBBCNT")]
pub type Tbbcnt = crate::Reg<tbbcnt::TbbcntSpec>;
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
pub mod tbbcnt;
#[doc = "DEBNCE (rw) register accessor: Debounce Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`debnce::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debnce::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debnce`]
module"]
#[doc(alias = "DEBNCE")]
pub type Debnce = crate::Reg<debnce::DebnceSpec>;
#[doc = "Debounce Count register"]
pub mod debnce;
#[doc = "RST_N (rw) register accessor: Hardware Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_n::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_n::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_n`]
module"]
#[doc(alias = "RST_N")]
pub type RstN = crate::Reg<rst_n::RstNSpec>;
#[doc = "Hardware Reset"]
pub mod rst_n;
#[doc = "BMOD (rw) register accessor: Bus Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmod`]
module"]
#[doc(alias = "BMOD")]
pub type Bmod = crate::Reg<bmod::BmodSpec>;
#[doc = "Bus Mode register"]
pub mod bmod;
#[doc = "PLDMND (rw) register accessor: Poll Demand register\n\nYou can [`read`](crate::Reg::read) this register and get [`pldmnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pldmnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pldmnd`]
module"]
#[doc(alias = "PLDMND")]
pub type Pldmnd = crate::Reg<pldmnd::PldmndSpec>;
#[doc = "Poll Demand register"]
pub mod pldmnd;
#[doc = "DBADDR (rw) register accessor: Descriptor List Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbaddr`]
module"]
#[doc(alias = "DBADDR")]
pub type Dbaddr = crate::Reg<dbaddr::DbaddrSpec>;
#[doc = "Descriptor List Base Address register"]
pub mod dbaddr;
#[doc = "IDSTS (rw) register accessor: Internal DMAC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`idsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idsts`]
module"]
#[doc(alias = "IDSTS")]
pub type Idsts = crate::Reg<idsts::IdstsSpec>;
#[doc = "Internal DMAC Status register"]
pub mod idsts;
#[doc = "IDINTEN (rw) register accessor: Internal DMAC Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`idinten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idinten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idinten`]
module"]
#[doc(alias = "IDINTEN")]
pub type Idinten = crate::Reg<idinten::IdintenSpec>;
#[doc = "Internal DMAC Interrupt Enable register"]
pub mod idinten;
#[doc = "DSCADDR (rw) register accessor: Current Host Descriptor Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscaddr`]
module"]
#[doc(alias = "DSCADDR")]
pub type Dscaddr = crate::Reg<dscaddr::DscaddrSpec>;
#[doc = "Current Host Descriptor Address register"]
pub mod dscaddr;
#[doc = "BUFADDR (rw) register accessor: Current Buffer Descriptor Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufaddr`]
module"]
#[doc(alias = "BUFADDR")]
pub type Bufaddr = crate::Reg<bufaddr::BufaddrSpec>;
#[doc = "Current Buffer Descriptor Address register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL (rw) register accessor: Card Threshold Control\n\nYou can [`read`](crate::Reg::read) this register and get [`cardthrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cardthrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cardthrctl`]
module"]
#[doc(alias = "CARDTHRCTL")]
pub type Cardthrctl = crate::Reg<cardthrctl::CardthrctlSpec>;
#[doc = "Card Threshold Control"]
pub mod cardthrctl;
#[doc = "BACKENDPWR (rw) register accessor: Power control\n\nYou can [`read`](crate::Reg::read) this register and get [`backendpwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backendpwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backendpwr`]
module"]
#[doc(alias = "BACKENDPWR")]
pub type Backendpwr = crate::Reg<backendpwr::BackendpwrSpec>;
#[doc = "Power control"]
pub mod backendpwr;
#[doc = "FIFO (rw) register accessor: SDIF FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "SDIF FIFO"]
pub mod fifo;
