#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RUNSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RUNSTATUSWDT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTATUSWDTR {
    #[doc = "Watchdog not running"]
    NOTRUNNING,
    #[doc = "Watchdog is running"]
    RUNNING,
}
impl RUNSTATUSWDTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RUNSTATUSWDTR::NOTRUNNING => false,
            RUNSTATUSWDTR::RUNNING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUNSTATUSWDTR {
        match value {
            false => RUNSTATUSWDTR::NOTRUNNING,
            true => RUNSTATUSWDTR::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline]
    pub fn is_not_running(&self) -> bool {
        *self == RUNSTATUSWDTR::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline]
    pub fn is_running(&self) -> bool {
        *self == RUNSTATUSWDTR::RUNNING
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Indicates whether or not the watchdog is running"]
    #[inline]
    pub fn runstatuswdt(&self) -> RUNSTATUSWDTR {
        RUNSTATUSWDTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
