#[doc = "Register `LOOP3STATE` reader"]
pub type R = crate::R<Loop3stateSpec>;
#[doc = "Register `LOOP3STATE` writer"]
pub type W = crate::W<Loop3stateSpec>;
#[doc = "Field `CNT` reader - Loop Counter"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Loop Counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ACTIVE` reader - Loop Active"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Loop Active"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCBEGIN` reader - Loop Start"]
pub type PcbeginR = crate::FieldReader;
#[doc = "Field `PCBEGIN` writer - Loop Start"]
pub type PcbeginW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Loop Counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - Loop Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Loop Start"]
    #[inline(always)]
    pub fn pcbegin(&self) -> PcbeginR {
        PcbeginR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Loop Counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<Loop3stateSpec> {
        CntW::new(self, 0)
    }
    #[doc = "Bit 12 - Loop Active"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<Loop3stateSpec> {
        ActiveW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Loop Start"]
    #[inline(always)]
    #[must_use]
    pub fn pcbegin(&mut self) -> PcbeginW<Loop3stateSpec> {
        PcbeginW::new(self, 16)
    }
}
#[doc = "Loop N State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`loop3state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop3state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Loop3stateSpec;
impl crate::RegisterSpec for Loop3stateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`loop3state::R`](R) reader structure"]
impl crate::Readable for Loop3stateSpec {}
#[doc = "`write(|w| ..)` method takes [`loop3state::W`](W) writer structure"]
impl crate::Writable for Loop3stateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOOP3STATE to value 0"]
impl crate::Resettable for Loop3stateSpec {
    const RESET_VALUE: u32 = 0;
}
