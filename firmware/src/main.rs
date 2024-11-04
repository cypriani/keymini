#![no_main]
#![no_std]

// set the panic handler
use panic_halt as _;

use core::convert::Infallible;
use core::mem::MaybeUninit;
use cortex_m::peripheral::SCB;
use hal::gpio::{Input, Output, Pin, PullUp, PushPull};
use hal::prelude::*;
use hal::usb;
use hal::{stm32, timers};
use keyberon::debounce::Debouncer;
use keyberon::key_code::KbHidReport;
use keyberon::layout::{CustomEvent, Layout};
use keyberon::matrix::Matrix;
use rtic::app;
use stm32f0xx_hal as hal;
use usb_device::bus::UsbBusAllocator;
use usb_device::class::UsbClass as _;
use usb_device::device::UsbDeviceState;

mod layout;

type UsbClass = keyberon::Class<'static, usb::UsbBusType, ()>;
type UsbDevice = usb_device::device::UsbDevice<'static, usb::UsbBusType>;

#[link_section = ".uninit.GOTO_BOOTLOADER"]
static mut GOTO_BOOTLOADER: MaybeUninit<u8> = MaybeUninit::uninit();
const GO: u8 = 60;

#[cortex_m_rt::pre_init]
unsafe fn maybe_jump_bootloader() {
    let software_reset = (*hal::pac::RCC::ptr()).csr.read().sftrstf().bit_is_set();
    let jump_bootloader = software_reset && GOTO_BOOTLOADER.assume_init() == GO;
    GOTO_BOOTLOADER.write(0);
    if jump_bootloader {
        cortex_m::asm::bootload(0x1FFFC800 as _);
    }
}

pub fn bootloader() -> ! {
    unsafe { GOTO_BOOTLOADER.write(GO) };
    SCB::sys_reset()
}

trait ResultExt<T> {
    fn get(self) -> T;
}
impl<T> ResultExt<T> for Result<T, Infallible> {
    fn get(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => match e {},
        }
    }
}

#[app(device = crate::hal::pac, peripherals = true, dispatchers = [CEC_CAN])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
    }

    #[local]
    struct Local {
        matrix: Matrix<Pin<Input<PullUp>>, Pin<Output<PushPull>>, 10, 4>,
        debouncer: Debouncer<[[bool; 10]; 4]>,
        layout: Layout<10, 4, 4, ()>,
        timer: timers::Timer<stm32::TIM3>,
    }

    #[init(local = [bus: Option<UsbBusAllocator<usb::UsbBusType>> = None])]
    fn init(mut c: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = c
            .device
            .RCC
            .configure()
            .hsi48()
            .enable_crs(c.device.CRS)
            .sysclk(48.mhz())
            .pclk(24.mhz())
            .freeze(&mut c.device.FLASH);

        let gpioa = c.device.GPIOA.split(&mut rcc);
        let gpiob = c.device.GPIOB.split(&mut rcc);

        let usb = usb::Peripheral {
            usb: c.device.USB,
            pin_dm: gpioa.pa11,
            pin_dp: gpioa.pa12,
        };
        *c.local.bus = Some(usb::UsbBusType::new(usb));
        let usb_bus = c.local.bus.as_ref().unwrap();

        let usb_class = keyberon::new_class(usb_bus, ());
        let usb_dev = keyberon::new_device(usb_bus);

        let mut timer = timers::Timer::tim3(c.device.TIM3, 1.khz(), &mut rcc);
        timer.listen(timers::Event::TimeOut);

        let pa3 = gpioa.pa3;
        let pa4 = gpioa.pa4;
        let pa5 = gpioa.pa5;
        let pa6 = gpioa.pa6;
        let pa7 = gpioa.pa7;
        let Ok(matrix) = cortex_m::interrupt::free(move |cs| {
            Matrix::new(
                [
                    pa3.into_pull_up_input(cs).downgrade(),
                    pa4.into_pull_up_input(cs).downgrade(),
                    pa5.into_pull_up_input(cs).downgrade(),
                    pa6.into_pull_up_input(cs).downgrade(),
                    pa7.into_pull_up_input(cs).downgrade(),
                    gpiob.pb0.into_pull_up_input(cs).downgrade(),
                    gpiob.pb1.into_pull_up_input(cs).downgrade(),
                    gpiob.pb2.into_pull_up_input(cs).downgrade(),
                    gpiob.pb10.into_pull_up_input(cs).downgrade(),
                    gpiob.pb11.into_pull_up_input(cs).downgrade(),
                ],
                [
                    gpiob.pb3.into_push_pull_output(cs).downgrade(),
                    gpiob.pb4.into_push_pull_output(cs).downgrade(),
                    gpiob.pb5.into_push_pull_output(cs).downgrade(),
                    gpiob.pb6.into_push_pull_output(cs).downgrade(),
                ],
            )
        });

        (
            Shared { usb_dev, usb_class },
            Local {
                timer,
                debouncer: Debouncer::new([[false; 10]; 4], [[false; 10]; 4], 5),
                matrix: matrix,
                layout: Layout::new(&crate::layout::LAYERS),
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USB, priority = 2, shared = [usb_dev, usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        (c.shared.usb_dev, c.shared.usb_class).lock(|usb_dev, usb_class| {
            if usb_dev.poll(&mut [usb_class]) {
                usb_class.poll();
            }
        });
    }

    #[task(
        binds = TIM3,
        priority = 1,
        local = [matrix, debouncer, timer, layout],
        shared = [usb_dev, usb_class],
    )]
    fn tick(mut c: tick::Context) {
        let _ = c.local.timer.wait();

        for event in c.local.debouncer.events(c.local.matrix.get().get()) {
            c.local.layout.event(event)
        }

        match c.local.layout.tick() {
            CustomEvent::Release(()) => bootloader(),
            _ => (),
        }
        if c.shared.usb_dev.lock(|d| d.state()) != UsbDeviceState::Configured {
            return;
        }
        let report: KbHidReport = c.local.layout.keycodes().collect();
        if !c
            .shared
            .usb_class
            .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
        {
            return;
        }
        while let Ok(0) = c.shared.usb_class.lock(|k| k.write(report.as_bytes())) {}
    }
}
