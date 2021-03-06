#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 20usize],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved3: [u8; 12usize],
    #[doc = "0x30 - Prepare the TWI slave to respond to a write command"]
    pub tasks_preparerx: TASKS_PREPARERX,
    #[doc = "0x34 - Prepare the TWI slave to respond to a read command"]
    pub tasks_preparetx: TASKS_PREPARETX,
    _reserved5: [u8; 92usize],
    #[doc = "0x94 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved6: [u8; 4usize],
    #[doc = "0x9c - Subscribe configuration for task SUSPEND"]
    pub subscribe_suspend: SUBSCRIBE_SUSPEND,
    #[doc = "0xa0 - Subscribe configuration for task RESUME"]
    pub subscribe_resume: SUBSCRIBE_RESUME,
    _reserved8: [u8; 12usize],
    #[doc = "0xb0 - Subscribe configuration for task PREPARERX"]
    pub subscribe_preparerx: SUBSCRIBE_PREPARERX,
    #[doc = "0xb4 - Subscribe configuration for task PREPARETX"]
    pub subscribe_preparetx: SUBSCRIBE_PREPARETX,
    _reserved10: [u8; 76usize],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved11: [u8; 28usize],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved12: [u8; 36usize],
    #[doc = "0x14c - Receive sequence started"]
    pub events_rxstarted: EVENTS_RXSTARTED,
    #[doc = "0x150 - Transmit sequence started"]
    pub events_txstarted: EVENTS_TXSTARTED,
    _reserved14: [u8; 16usize],
    #[doc = "0x164 - Write command received"]
    pub events_write: EVENTS_WRITE,
    #[doc = "0x168 - Read command received"]
    pub events_read: EVENTS_READ,
    _reserved16: [u8; 24usize],
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    _reserved17: [u8; 28usize],
    #[doc = "0x1a4 - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved18: [u8; 36usize],
    #[doc = "0x1cc - Publish configuration for event RXSTARTED"]
    pub publish_rxstarted: PUBLISH_RXSTARTED,
    #[doc = "0x1d0 - Publish configuration for event TXSTARTED"]
    pub publish_txstarted: PUBLISH_TXSTARTED,
    _reserved20: [u8; 16usize],
    #[doc = "0x1e4 - Publish configuration for event WRITE"]
    pub publish_write: PUBLISH_WRITE,
    #[doc = "0x1e8 - Publish configuration for event READ"]
    pub publish_read: PUBLISH_READ,
    _reserved22: [u8; 20usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved23: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved26: [u8; 452usize],
    #[doc = "0x4d0 - Error source"]
    pub errorsrc: ERRORSRC,
    #[doc = "0x4d4 - Status register indicating which address had a match"]
    pub match_: MATCH,
    _reserved28: [u8; 40usize],
    #[doc = "0x500 - Enable TWIS"]
    pub enable: ENABLE,
    _reserved29: [u8; 4usize],
    #[doc = "0x508 - Unspecified"]
    pub psel: PSEL,
    _reserved30: [u8; 36usize],
    #[doc = "0x534 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544 - TXD EasyDMA channel"]
    pub txd: TXD,
    _reserved32: [u8; 52usize],
    #[doc = "0x588 - Description collection: TWI slave address n"]
    pub address: [ADDRESS; 2],
    _reserved33: [u8; 4usize],
    #[doc = "0x594 - Configuration register for the address match mechanism"]
    pub config: CONFIG,
    _reserved34: [u8; 40usize],
    #[doc = "0x5c0 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    pub orc: ORC,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL signal"]
    pub scl: self::psel::SCL,
    #[doc = "0x04 - Pin select for SDA signal"]
    pub sda: self::psel::SDA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD Data pointer"]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of bytes in RXD buffer"]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last RXD transaction"]
    pub amount: self::rxd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::rxd::LIST,
}
#[doc = r"Register block"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD Data pointer"]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of bytes in TXD buffer"]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last TXD transaction"]
    pub amount: self::txd::AMOUNT,
    #[doc = "0x0c - EasyDMA list type"]
    pub list: self::txd::LIST,
}
#[doc = r"Register block"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "Stop TWI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop TWI transaction"]
pub mod tasks_stop;
#[doc = "Suspend TWI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_suspend](tasks_suspend) module"]
pub type TASKS_SUSPEND = crate::Reg<u32, _TASKS_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SUSPEND;
#[doc = "`write(|w| ..)` method takes [tasks_suspend::W](tasks_suspend::W) writer structure"]
impl crate::Writable for TASKS_SUSPEND {}
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "Resume TWI transaction\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_resume](tasks_resume) module"]
pub type TASKS_RESUME = crate::Reg<u32, _TASKS_RESUME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RESUME;
#[doc = "`write(|w| ..)` method takes [tasks_resume::W](tasks_resume::W) writer structure"]
impl crate::Writable for TASKS_RESUME {}
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "Prepare the TWI slave to respond to a write command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_preparerx](tasks_preparerx) module"]
pub type TASKS_PREPARERX = crate::Reg<u32, _TASKS_PREPARERX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_PREPARERX;
#[doc = "`write(|w| ..)` method takes [tasks_preparerx::W](tasks_preparerx::W) writer structure"]
impl crate::Writable for TASKS_PREPARERX {}
#[doc = "Prepare the TWI slave to respond to a write command"]
pub mod tasks_preparerx;
#[doc = "Prepare the TWI slave to respond to a read command\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_preparetx](tasks_preparetx) module"]
pub type TASKS_PREPARETX = crate::Reg<u32, _TASKS_PREPARETX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_PREPARETX;
#[doc = "`write(|w| ..)` method takes [tasks_preparetx::W](tasks_preparetx::W) writer structure"]
impl crate::Writable for TASKS_PREPARETX {}
#[doc = "Prepare the TWI slave to respond to a read command"]
pub mod tasks_preparetx;
#[doc = "Subscribe configuration for task STOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_stop](subscribe_stop) module"]
pub type SUBSCRIBE_STOP = crate::Reg<u32, _SUBSCRIBE_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOP;
#[doc = "`read()` method returns [subscribe_stop::R](subscribe_stop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_stop::W](subscribe_stop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOP {}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Subscribe configuration for task SUSPEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_suspend](subscribe_suspend) module"]
pub type SUBSCRIBE_SUSPEND = crate::Reg<u32, _SUBSCRIBE_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_SUSPEND;
#[doc = "`read()` method returns [subscribe_suspend::R](subscribe_suspend::R) reader structure"]
impl crate::Readable for SUBSCRIBE_SUSPEND {}
#[doc = "`write(|w| ..)` method takes [subscribe_suspend::W](subscribe_suspend::W) writer structure"]
impl crate::Writable for SUBSCRIBE_SUSPEND {}
#[doc = "Subscribe configuration for task SUSPEND"]
pub mod subscribe_suspend;
#[doc = "Subscribe configuration for task RESUME\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_resume](subscribe_resume) module"]
pub type SUBSCRIBE_RESUME = crate::Reg<u32, _SUBSCRIBE_RESUME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_RESUME;
#[doc = "`read()` method returns [subscribe_resume::R](subscribe_resume::R) reader structure"]
impl crate::Readable for SUBSCRIBE_RESUME {}
#[doc = "`write(|w| ..)` method takes [subscribe_resume::W](subscribe_resume::W) writer structure"]
impl crate::Writable for SUBSCRIBE_RESUME {}
#[doc = "Subscribe configuration for task RESUME"]
pub mod subscribe_resume;
#[doc = "Subscribe configuration for task PREPARERX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_preparerx](subscribe_preparerx) module"]
pub type SUBSCRIBE_PREPARERX = crate::Reg<u32, _SUBSCRIBE_PREPARERX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_PREPARERX;
#[doc = "`read()` method returns [subscribe_preparerx::R](subscribe_preparerx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_PREPARERX {}
#[doc = "`write(|w| ..)` method takes [subscribe_preparerx::W](subscribe_preparerx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_PREPARERX {}
#[doc = "Subscribe configuration for task PREPARERX"]
pub mod subscribe_preparerx;
#[doc = "Subscribe configuration for task PREPARETX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subscribe_preparetx](subscribe_preparetx) module"]
pub type SUBSCRIBE_PREPARETX = crate::Reg<u32, _SUBSCRIBE_PREPARETX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_PREPARETX;
#[doc = "`read()` method returns [subscribe_preparetx::R](subscribe_preparetx::R) reader structure"]
impl crate::Readable for SUBSCRIBE_PREPARETX {}
#[doc = "`write(|w| ..)` method takes [subscribe_preparetx::W](subscribe_preparetx::W) writer structure"]
impl crate::Writable for SUBSCRIBE_PREPARETX {}
#[doc = "Subscribe configuration for task PREPARETX"]
pub mod subscribe_preparetx;
#[doc = "TWI stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "TWI error\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "TWI error"]
pub mod events_error;
#[doc = "Receive sequence started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxstarted](events_rxstarted) module"]
pub type EVENTS_RXSTARTED = crate::Reg<u32, _EVENTS_RXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXSTARTED;
#[doc = "`read()` method returns [events_rxstarted::R](events_rxstarted::R) reader structure"]
impl crate::Readable for EVENTS_RXSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_rxstarted::W](events_rxstarted::W) writer structure"]
impl crate::Writable for EVENTS_RXSTARTED {}
#[doc = "Receive sequence started"]
pub mod events_rxstarted;
#[doc = "Transmit sequence started\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txstarted](events_txstarted) module"]
pub type EVENTS_TXSTARTED = crate::Reg<u32, _EVENTS_TXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXSTARTED;
#[doc = "`read()` method returns [events_txstarted::R](events_txstarted::R) reader structure"]
impl crate::Readable for EVENTS_TXSTARTED {}
#[doc = "`write(|w| ..)` method takes [events_txstarted::W](events_txstarted::W) writer structure"]
impl crate::Writable for EVENTS_TXSTARTED {}
#[doc = "Transmit sequence started"]
pub mod events_txstarted;
#[doc = "Write command received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_write](events_write) module"]
pub type EVENTS_WRITE = crate::Reg<u32, _EVENTS_WRITE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_WRITE;
#[doc = "`read()` method returns [events_write::R](events_write::R) reader structure"]
impl crate::Readable for EVENTS_WRITE {}
#[doc = "`write(|w| ..)` method takes [events_write::W](events_write::W) writer structure"]
impl crate::Writable for EVENTS_WRITE {}
#[doc = "Write command received"]
pub mod events_write;
#[doc = "Read command received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_read](events_read) module"]
pub type EVENTS_READ = crate::Reg<u32, _EVENTS_READ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READ;
#[doc = "`read()` method returns [events_read::R](events_read::R) reader structure"]
impl crate::Readable for EVENTS_READ {}
#[doc = "`write(|w| ..)` method takes [events_read::W](events_read::W) writer structure"]
impl crate::Writable for EVENTS_READ {}
#[doc = "Read command received"]
pub mod events_read;
#[doc = "Publish configuration for event STOPPED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_stopped](publish_stopped) module"]
pub type PUBLISH_STOPPED = crate::Reg<u32, _PUBLISH_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_STOPPED;
#[doc = "`read()` method returns [publish_stopped::R](publish_stopped::R) reader structure"]
impl crate::Readable for PUBLISH_STOPPED {}
#[doc = "`write(|w| ..)` method takes [publish_stopped::W](publish_stopped::W) writer structure"]
impl crate::Writable for PUBLISH_STOPPED {}
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "Publish configuration for event ERROR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_error](publish_error) module"]
pub type PUBLISH_ERROR = crate::Reg<u32, _PUBLISH_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ERROR;
#[doc = "`read()` method returns [publish_error::R](publish_error::R) reader structure"]
impl crate::Readable for PUBLISH_ERROR {}
#[doc = "`write(|w| ..)` method takes [publish_error::W](publish_error::W) writer structure"]
impl crate::Writable for PUBLISH_ERROR {}
#[doc = "Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "Publish configuration for event RXSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_rxstarted](publish_rxstarted) module"]
pub type PUBLISH_RXSTARTED = crate::Reg<u32, _PUBLISH_RXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_RXSTARTED;
#[doc = "`read()` method returns [publish_rxstarted::R](publish_rxstarted::R) reader structure"]
impl crate::Readable for PUBLISH_RXSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_rxstarted::W](publish_rxstarted::W) writer structure"]
impl crate::Writable for PUBLISH_RXSTARTED {}
#[doc = "Publish configuration for event RXSTARTED"]
pub mod publish_rxstarted;
#[doc = "Publish configuration for event TXSTARTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_txstarted](publish_txstarted) module"]
pub type PUBLISH_TXSTARTED = crate::Reg<u32, _PUBLISH_TXSTARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_TXSTARTED;
#[doc = "`read()` method returns [publish_txstarted::R](publish_txstarted::R) reader structure"]
impl crate::Readable for PUBLISH_TXSTARTED {}
#[doc = "`write(|w| ..)` method takes [publish_txstarted::W](publish_txstarted::W) writer structure"]
impl crate::Writable for PUBLISH_TXSTARTED {}
#[doc = "Publish configuration for event TXSTARTED"]
pub mod publish_txstarted;
#[doc = "Publish configuration for event WRITE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_write](publish_write) module"]
pub type PUBLISH_WRITE = crate::Reg<u32, _PUBLISH_WRITE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_WRITE;
#[doc = "`read()` method returns [publish_write::R](publish_write::R) reader structure"]
impl crate::Readable for PUBLISH_WRITE {}
#[doc = "`write(|w| ..)` method takes [publish_write::W](publish_write::W) writer structure"]
impl crate::Writable for PUBLISH_WRITE {}
#[doc = "Publish configuration for event WRITE"]
pub mod publish_write;
#[doc = "Publish configuration for event READ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [publish_read](publish_read) module"]
pub type PUBLISH_READ = crate::Reg<u32, _PUBLISH_READ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_READ;
#[doc = "`read()` method returns [publish_read::R](publish_read::R) reader structure"]
impl crate::Readable for PUBLISH_READ {}
#[doc = "`write(|w| ..)` method takes [publish_read::W](publish_read::W) writer structure"]
impl crate::Writable for PUBLISH_READ {}
#[doc = "Publish configuration for event READ"]
pub mod publish_read;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Error source\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](errorsrc) module"]
pub type ERRORSRC = crate::Reg<u32, _ERRORSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORSRC;
#[doc = "`read()` method returns [errorsrc::R](errorsrc::R) reader structure"]
impl crate::Readable for ERRORSRC {}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](errorsrc::W) writer structure"]
impl crate::Writable for ERRORSRC {}
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "Status register indicating which address had a match\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](match_) module"]
pub type MATCH = crate::Reg<u32, _MATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATCH;
#[doc = "`read()` method returns [match_::R](match_::R) reader structure"]
impl crate::Readable for MATCH {}
#[doc = "Status register indicating which address had a match"]
pub mod match_;
#[doc = "Enable TWIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable TWIS"]
pub mod enable;
#[doc = "Description collection: TWI slave address n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address](address) module"]
pub type ADDRESS = crate::Reg<u32, _ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRESS;
#[doc = "`read()` method returns [address::R](address::R) reader structure"]
impl crate::Readable for ADDRESS {}
#[doc = "`write(|w| ..)` method takes [address::W](address::W) writer structure"]
impl crate::Writable for ADDRESS {}
#[doc = "Description collection: TWI slave address n"]
pub mod address;
#[doc = "Configuration register for the address match mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register for the address match mechanism"]
pub mod config;
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](orc) module"]
pub type ORC = crate::Reg<u32, _ORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORC;
#[doc = "`read()` method returns [orc::R](orc::R) reader structure"]
impl crate::Readable for ORC {}
#[doc = "`write(|w| ..)` method takes [orc::W](orc::W) writer structure"]
impl crate::Writable for ORC {}
#[doc = "Over-read character. Character sent out in case of an over-read of the transmit buffer."]
pub mod orc;
