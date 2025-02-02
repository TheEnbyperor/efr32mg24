#[doc = "Register `EN` reader"]
pub type R = crate::R<EnSpec>;
#[doc = "Register `EN` writer"]
pub type W = crate::W<EnSpec>;
#[doc = "Field `EN` reader - LDMA module enable and disable register"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - LDMA module enable and disable register"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LDMA module enable and disable register"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDMA module enable and disable register"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<EnSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnSpec;
impl crate::RegisterSpec for EnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`en::R`](R) reader structure"]
impl crate::Readable for EnSpec {}
#[doc = "`write(|w| ..)` method takes [`en::W`](W) writer structure"]
impl crate::Writable for EnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EnSpec {
    const RESET_VALUE: u32 = 0;
}
