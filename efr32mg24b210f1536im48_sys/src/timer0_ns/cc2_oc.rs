#[doc = "Register `CC2_OC` reader"]
pub type R = crate::R<Cc2OcSpec>;
#[doc = "Register `CC2_OC` writer"]
pub type W = crate::W<Cc2OcSpec>;
#[doc = "Field `OC` reader - Output Compare Value"]
pub type OcR = crate::FieldReader<u32>;
#[doc = "Field `OC` writer - Output Compare Value"]
pub type OcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn oc(&mut self) -> OcW<Cc2OcSpec> {
        OcW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_oc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_oc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2OcSpec;
impl crate::RegisterSpec for Cc2OcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_oc::R`](R) reader structure"]
impl crate::Readable for Cc2OcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc2_oc::W`](W) writer structure"]
impl crate::Writable for Cc2OcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_OC to value 0"]
impl crate::Resettable for Cc2OcSpec {
    const RESET_VALUE: u32 = 0;
}
