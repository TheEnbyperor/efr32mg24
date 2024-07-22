#![no_std]
#![no_main]

extern crate panic_semihosting;

mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

static SLEEP_MODES: [&'static str; 5] = [
    "run", "sleep", "deep sleep", "stop", "shutoff"
];

#[cortex_m_rt::entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let _p = efr32mg24b210f1536im48_sys::Peripherals::take().unwrap();

    if unsafe { sys::sl_sleeptimer_init() } != 0 {
        panic!("Failed to initialise sleep timer");
    }
    if unsafe { sys::sl_power_manager_init() } != 0 {
        panic!("Failed to initialise power manager");
    }

    let mut pm_event_h: core::mem::MaybeUninit<sys::sl_power_manager_em_transition_event_handle_t> = core::mem::MaybeUninit::uninit();
    let pm_event_info = sys::sl_power_manager_em_transition_event_info_t {
        event_mask: sys::SL_POWER_MANAGER_EVENT_TRANSITION_ENTERING_EM0 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_LEAVING_EM0 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_ENTERING_EM1 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_LEAVING_EM1 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_ENTERING_EM2 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_LEAVING_EM2 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_ENTERING_EM3 |
            sys::SL_POWER_MANAGER_EVENT_TRANSITION_LEAVING_EM3,
        on_event: Some(pm_event_cb),
    };
    unsafe { sys::sl_power_manager_subscribe_em_transition_event(pm_event_h.as_mut_ptr(), &pm_event_info) };

    let mut timer: core::mem::MaybeUninit<sys::sl_sleeptimer_timer_handle_t> = core::mem::MaybeUninit::uninit();
    unsafe { sys::sl_sleeptimer_start_periodic_timer_ms(
        timer.as_mut_ptr(), 1000, Some(timer_callback), core::ptr::null_mut(), 0, 0
    ) };

    loop {
        unsafe { sys::sl_power_manager_sleep() };
    }
}

unsafe extern "C" fn timer_callback(_timer: *mut sys::sl_sleeptimer_timer_handle, _data: *mut core::ffi::c_void) {
    let tick = unsafe { sys::sl_sleeptimer_get_tick_count64() };
    let mut time: u64 = 0;
    if unsafe { sys::sl_sleeptimer_tick64_to_ms(tick, &mut time) } != 0 {
        panic!("Unable to convert time");
    }
    cortex_m_semihosting::hprintln!("Time: {}ms", time);
}

unsafe extern "C" fn pm_event_cb(from: sys::sl_power_manager_em_t, to: sys::sl_power_manager_em_t) {
    cortex_m_semihosting::hprintln!("Power management event: from {} to {}", SLEEP_MODES[from as usize], SLEEP_MODES[to as usize]);
}

#[cortex_m_rt::exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    use core::fmt::Write;

    if let Ok(mut hstdout) = cortex_m_semihosting::hio::hstdout() {
        writeln!(hstdout, "HardFault: {:#?}", ef).ok();
    }

    loop {}
}

#[cortex_m_rt::exception]
unsafe fn DefaultHandler(irq_n: i16) {
    match irq_n {
        4 => sys::TIMER0_IRQHandler(),
        n => cortex_m_semihosting::hprintln!("Unhandled IRQ: {}", n),
    }
}