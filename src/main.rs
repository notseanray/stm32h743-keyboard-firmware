#![no_main]
#![no_std]

mod multiplexers;
mod layers;
mod layout;
mod config_structs;
mod aliases;
mod userconfig;

use core::mem::MaybeUninit;

// set the panic handler
use panic_halt as _;

use keyberon::debounce::Debouncer;
use keyberon::key_code::KbHidReport;
use keyberon::layout::Layout;
use rtic::app;
use stm32h7xx_hal::gpio::{self, EPin, Input, Output, PushPull};
use stm32h7xx_hal::prelude::*;
use stm32h7xx_hal::usb_hs::{UsbBus, USB1};
use usb_device::bus::UsbBusAllocator;

type UsbDevice = usb_device::device::UsbDevice<'static, UsbBus<USB1>>;

static mut EP_MEMORY: MaybeUninit<[u32; 1024]> = MaybeUninit::uninit();

#[rtic::app(device = stm32h7xx_hal::stm32, peripherals = true)]
mod app {
    use analog_multiplexer::{DummyPin, Multiplexer};
    use stm32h7xx_hal::{adc::{Adc, AdcSampleTime, Resolution}, delay::Delay, pac::{Peripherals, PWR, SYSCFG}, rcc::{rec::{AdcClkSel, UsbClkSel}, CoreClocks}, time::{Hertz, MegaHertz, MicroSeconds}, timer::{Event, Timer}, usb_hs::{Usb1BusType, UsbBus, USB1}};
    use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
    use usbd_serial::SerialPort;

    use self::{aliases::SelectPins, layers::LAYERS};

    use super::*;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_serial: SerialPort<'static, UsbBus<USB1>>
    }

    #[local]
    struct Local {
        debouncer: Debouncer<[[bool; 13]; 4]>,
        layout: Layout<16, 5, 7, ()>,
        //bus: Option<Usb1BusType>,
        //ep_mem: [u32; 1024],
        multiplexer: aliases::Multiplex,
    }

    // todo power check?
    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let keyboard_config = config_structs::KeyboardConfig {
            north_down: userconfig::NORTH_DOWN,
            actuation_threshold: userconfig::ACTUATION_THRESHOLD,
            release_threshold: userconfig::RELEASE_THRESHOLD,
            ignore_below: userconfig::IGNORE_BELOW,
            recalibration_rate: userconfig::RECALIBRATION_RATE,
            num_multiplexers: userconfig::NUM_MULTIPLEXERS,
            max_channels: userconfig::MAX_CHANNELS,
            usb_vid: userconfig::USB_VID,
            usb_pid: userconfig::USB_PID,
        };

        let dp = Peripherals::take().expect("Cannot take peripherals");
        //let pwr = ctx.device.PWR.constrain();
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.vos1().freeze();
        let rcc = ctx.device.RCC.constrain();
        let ccdr = rcc.sys_ck(Hertz::from_raw(480_000_000)).freeze(pwrcfg, &ctx.device.SYSCFG);
        let clocks = rcc.sysclk(Hertz::from_raw(480_000_000)).use_hse(Hertz::from_raw(25_000)).freeze(pwrcfg, &ctx.device.SYSCFG);
        let mut ccdr = rcc.sys_ck(80.MHz()).freeze(pwrcfg, &ctx.device.SYSCFG);

        // 2khz
        let mut timer3 = ctx.device.TIM3.timer(
            MicroSeconds::from_ticks(500).into_rate(),
            ccdr.peripheral.TIM3,
            &ccdr.clocks,
        );
        timer3.listen(Event::TimeOut);

        /*
        // Schedule a task that occasionally checks if the default mV values need to be adjusted
        ctx.schedule
            .update_defaults(ctx.start+ config.keyboard.recalibration_rate.cycles())
            .unwrap();

        ctx.schedule
            .tick_display(ctx.start + DISPLAY_TICK_RATE.cycles())
            .unwrap();
        */



        // internal USB voltage regulator in ON mode
        unsafe {
            let pwr = *PWR::ptr();
            pwr.cr3.modify(|_, w| w.usbregen().set_bit());
            while pwr.cr3.read().usb33rdy().bit_is_clear() {}
        }

        // 48MHz CLOCK
        let _ = ccdr.clocks.hsi48_ck().expect("HSI48 must run");
        ccdr.peripheral.kernel_usb_clk_mux(UsbClkSel::Hsi48);

        let gpioa = ctx.device.GPIOA.split(ccdr.peripheral.GPIOA);
        let gpiob = ctx.device.GPIOB.split(ccdr.peripheral.GPIOB);
        let gpioc = ctx.device.GPIOC.split(ccdr.peripheral.GPIOC);

        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();

        // rm0433
        let (pin_dm, pin_dp) = {
            let gpiob = ctx.device.GPIOB.split(ccdr.peripheral.GPIOB);
            (gpiob.pb14.into_alternate(), gpiob.pb15.into_alternate())
        };

        let usb = USB1::new(
            ctx.device.OTG1_HS_GLOBAL,
            ctx.device.OTG1_HS_DEVICE,
            ctx.device.OTG1_HS_PWRCLK,
            pin_dm,
            pin_dp,
            ccdr.peripheral.USB1OTG,
            &ccdr.clocks,
        );

        // Initialise EP_MEMORY to zero
        unsafe {
            let buf: &mut [MaybeUninit<u32>; 1024] =
                &mut *(core::ptr::addr_of_mut!(EP_MEMORY) as *mut _);
            for value in buf.iter_mut() {
                value.as_mut_ptr().write(0);
            }
        }
        let usb_bus = cortex_m::singleton!(
            : usb_device::class_prelude::UsbBusAllocator<Usb1BusType> =
                UsbBus::new(usb, unsafe { EP_MEMORY.assume_init_mut() })
        )
        .unwrap();
        let usb_keyboard = keyberon::new_class(usb_bus, ());
        let usb_dev = keyberon::new_device(usb_bus);
        let usb_serial = usbd_serial::SerialPort::new(usb_bus);
        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(userconfig::USB_VID, userconfig::USB_PID))
            .strings(&[usb_device::device::StringDescriptors::default()
                .manufacturer("SeanCo")
                .product("ShitBoardv1")
                .serial_number("0")])
            .unwrap()
            .device_class(usbd_serial::USB_CLASS_CDC)
            .build();

        let mut pa0 = gpioa.pa0.into_analog();
        let mut pa1 = gpioa.pa1.into_analog();
        let mut pa2 = gpioa.pa2.into_analog();
        let mut pa3 = gpioa.pa3.into_analog();
        let mut pa4 = gpioa.pa4.into_analog();
        let analog_pins = (pa0, pa1, pa2, pa3, pa4);

        let s0 = gpioc.pc13.into_push_pull_output();
        let s1 = gpiob.pb8.into_push_pull_output();
        let s2 = gpiob.pb12.into_push_pull_output();
        let s3 = gpioa.pa15.into_push_pull_output();
        let en = DummyPin; // Just run it to GND to keep always-enabled
        let select_pins = (s0, s1, s2, s3, en);
        let mut multiplexer = Multiplexer::new(select_pins);

        let mut ch_states0: multiplexers::ChannelStates = Default::default();
        let mut ch_states1: multiplexers::ChannelStates = Default::default();
        let mut ch_states2: multiplexers::ChannelStates = Default::default();
        let mut ch_states3: multiplexers::ChannelStates = Default::default();
        let mut ch_states4: multiplexers::ChannelStates = Default::default();

        ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::Pll2P);
        let cp = cortex_m::Peripherals::take().unwrap();
        let mut delay = Delay::new(cp.SYST, ccdr.clocks);

        // TODO double check clock settings
        let mut adc = Adc::adc1(ctx.device.ADC1, Hertz::from_raw(50_000_000), &mut delay, ccdr.peripheral.ADC12, &ccdr.clocks).enable();
        adc.set_resolution(Resolution::SixteenBit);
        let sample_time = AdcSampleTime::T_8;
        adc.set_sample_time(sample_time);

        // Read in the initial millivolt values for all analog channels so we have
        // a default/resting state to evaluate against.  We'll set new defaults later
        // after we've captured a few values (controlled by DEFAULT_WAIT_MS).
        for channel in 0..16 {
            // This sets the channel on all multiplexers simultaneously
            // (since they're all connected to the same S0,S1,S2,S3 pins).
            multiplexer.set_channel(channel);

            // Read the analog value of each channel/key and store it in our ChannelStates struct...
            for multi in 0..userconfig::NUM_MULTIPLEXERS {
                let millivolts = match multi {
                    0 => {
                        adc.start_conversion(&mut analog_pins.0);
                        let value = adc.read(&mut analog_pins.0).unwrap_or(0);
                        value / 4
                    },
                    1 => {
                        adc.start_conversion(&mut analog_pins.1);
                        adc.read(&mut analog_pins.1).unwrap_or(0) / 4
                    },
                    2 => {
                        adc.start_conversion(&mut analog_pins.2);
                        adc.read(&mut analog_pins.2).unwrap_or(0) / 4
                    },
                    3 => {
                        adc.start_conversion(&mut analog_pins.3);
                        adc.read(&mut analog_pins.3).unwrap_or(0) / 4
                    },
                    4 => {
                        adc.start_conversion(&mut analog_pins.4);
                        adc.read(&mut analog_pins.4).unwrap_or(0) / 4
                    },
                    _ => unreachable!(), // Riskeyboard 70 only has 5 multiplexers
                };
                match multi {
                    0 => ch_states0[channel as usize].update_default(millivolts),
                    1 => ch_states1[channel as usize].update_default(millivolts),
                    2 => ch_states2[channel as usize].update_default(millivolts),
                    3 => ch_states3[channel as usize].update_default(millivolts),
                    4 => ch_states4[channel as usize].update_default(millivolts),
                    _ => {}
                };
            }
        }

        (
            Shared { usb_dev, usb_serial },
            Local {
                debouncer: Debouncer::new([[false; 13]; 4], [[false; 13]; 4], 5),
                layout: Layout::new(&LAYERS),
                multiplexer,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = OTG_FS, priority = 2, shared = [usb_dev, usb_serial])]
    fn usb_tx(c: usb_tx::Context) {
        c.shared.usb_serial.lock(|usb_dev, usbd_serial| {
            if usb_dev.poll(&mut [usb_serial]) {
                usb_class.poll();
            }
        })
    }

    #[task(binds = TIM3, priority = 1, local = [debouncer, layout])]
    fn tick(mut ctx: tick::Context) {

        for event in c.local.debouncer.events(c.local.matrix.get().unwrap()) {
            ctx.local.layout.event(event);
        }
        match ctx.local.layout.tick() {
            keyberon::layout::CustomEvent::Release(()) => unsafe {
                cortex_m::asm::bootload(0x1FFF0000 as _)
            },
            _ => (),
        }

        let report: KbHidReport = c.local.layout.keycodes().collect();
        if ctx.shared
            .usb_class
            .lock(|k| k.device_mut().set_keyboard_report(report.clone()))
        {
            while let Ok(0) = ctx.shared.usb_serial.lock(|k| k.write(report.as_bytes())) {}
        }
    }
}
