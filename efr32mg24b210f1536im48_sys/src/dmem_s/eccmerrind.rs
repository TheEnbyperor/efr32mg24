#[doc = "Register `ECCMERRIND` reader"]
pub type R = crate::R<EccmerrindSpec>;
#[doc = "Field `P0` reader - Multiple ECC errors on AHB port 0"]
pub type P0R = crate::BitReader;
#[doc = "Field `P1` reader - Multiple ECC errors on AHB port 1"]
pub type P1R = crate::BitReader;
#[doc = "Field `P2` reader - Multiple ECC errors on AHB port 2"]
pub type P2R = crate::BitReader;
#[doc = "Field `P3` reader - Multiple ECC errors on AHB port 2"]
pub type P3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Multiple ECC errors on AHB port 0"]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multiple ECC errors on AHB port 1"]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple ECC errors on AHB port 2"]
    #[inline(always)]
    pub fn p2(&self) -> P2R {
        P2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multiple ECC errors on AHB port 2"]
    #[inline(always)]
    pub fn p3(&self) -> P3R {
        P3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eccmerrind::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccmerrindSpec;
impl crate::RegisterSpec for EccmerrindSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccmerrind::R`](R) reader structure"]
impl crate::Readable for EccmerrindSpec {}
#[doc = "`reset()` method sets ECCMERRIND to value 0"]
impl crate::Resettable for EccmerrindSpec {
    const RESET_VALUE: u32 = 0;
}
