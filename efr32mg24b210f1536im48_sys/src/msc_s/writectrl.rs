#[doc = "Register `WRITECTRL` reader"]
pub type R = crate::R<WritectrlSpec>;
#[doc = "Register `WRITECTRL` writer"]
pub type W = crate::W<WritectrlSpec>;
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WrenR = crate::BitReader;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IrqeraseabortR = crate::BitReader;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IrqeraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWRITE` reader - Low-Power Write"]
pub type LpwriteR = crate::BitReader;
#[doc = "Field `LPWRITE` writer - Low-Power Write"]
pub type LpwriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGECOUNT` reader - EraseRange Count"]
pub type RangecountR = crate::FieldReader<u16>;
#[doc = "Field `RANGECOUNT` writer - EraseRange Count"]
pub type RangecountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IrqeraseabortR {
        IrqeraseabortR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-Power Write"]
    #[inline(always)]
    pub fn lpwrite(&self) -> LpwriteR {
        LpwriteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:25 - EraseRange Count"]
    #[inline(always)]
    pub fn rangecount(&self) -> RangecountR {
        RangecountR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WrenW<WritectrlSpec> {
        WrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn irqeraseabort(&mut self) -> IrqeraseabortW<WritectrlSpec> {
        IrqeraseabortW::new(self, 1)
    }
    #[doc = "Bit 3 - Low-Power Write"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrite(&mut self) -> LpwriteW<WritectrlSpec> {
        LpwriteW::new(self, 3)
    }
    #[doc = "Bits 16:25 - EraseRange Count"]
    #[inline(always)]
    #[must_use]
    pub fn rangecount(&mut self) -> RangecountW<WritectrlSpec> {
        RangecountW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`writectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WritectrlSpec;
impl crate::RegisterSpec for WritectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writectrl::R`](R) reader structure"]
impl crate::Readable for WritectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`writectrl::W`](W) writer structure"]
impl crate::Writable for WritectrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WritectrlSpec {
    const RESET_VALUE: u32 = 0;
}
