#[doc = "Register `CC2_OCB` reader"]
pub type R = crate::R<Cc2OcbSpec>;
#[doc = "Register `CC2_OCB` writer"]
pub type W = crate::W<Cc2OcbSpec>;
#[doc = "Field `OCB` reader - Output Compare Value Buffer"]
pub type OcbR = crate::FieldReader<u16>;
#[doc = "Field `OCB` writer - Output Compare Value Buffer"]
pub type OcbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output Compare Value Buffer"]
    #[inline(always)]
    pub fn ocb(&self) -> OcbR {
        OcbR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Compare Value Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ocb(&mut self) -> OcbW<Cc2OcbSpec> {
        OcbW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ocb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ocb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2OcbSpec;
impl crate::RegisterSpec for Cc2OcbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ocb::R`](R) reader structure"]
impl crate::Readable for Cc2OcbSpec {}
#[doc = "`write(|w| ..)` method takes [`cc2_ocb::W`](W) writer structure"]
impl crate::Writable for Cc2OcbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_OCB to value 0"]
impl crate::Resettable for Cc2OcbSpec {
    const RESET_VALUE: u32 = 0;
}
