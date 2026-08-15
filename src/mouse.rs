#![allow(unused)]
//! Contains code for mouse emulation

#[cfg(feature = "evdev-rust")]
use evdev::{AttributeSet, EventType, InputEvent, KeyCode, KeyEvent, RelativeAxisCode, uinput::VirtualDevice};

#[cfg(feature = "evdev-native")]
use evdev_rs::{Device, DeviceWrapper, InputEvent, ReadFlag, UInputDevice, UninitDevice, enums::{BusType, EventCode, EventType, EV_KEY, EV_REL, EV_SYN}};

use std::time::Duration;
use anyhow::Result;

const PRESS_RELEASE_DELAY_MIN: Duration = Duration::from_millis(5);

#[derive(Debug, Clone, Copy)]
pub enum MouseKey {
    Left,
    Right,
    Middle
}


#[cfg(feature = "evdev-rust")]
impl MouseKey {
    /// Convert to `KeyCode` id
    pub fn keycode(self) -> u16 {
        Into::<KeyCode>::into(self).0
    }
}

#[cfg(feature = "evdev-rust")]
impl Into<KeyCode> for MouseKey {
    fn into(self) -> KeyCode {
        match self {
            Self::Left => KeyCode::BTN_LEFT,
            Self::Right => KeyCode::BTN_RIGHT,
            Self::Middle => KeyCode::BTN_MIDDLE,
        }
    }
}

#[cfg(feature = "evdev-native")]
impl Into<EV_KEY> for MouseKey {
    fn into(self) -> EV_KEY {
        match self {
            Self::Left => EV_KEY::BTN_LEFT,
            Self::Right => EV_KEY::BTN_RIGHT,
            Self::Middle => EV_KEY::BTN_MIDDLE,
        }
    }
}

pub struct FakeMouse {
    #[cfg(feature = "evdev-rust")]
    device: VirtualDevice,

    #[cfg(feature = "evdev-native")]
    device: UInputDevice,
}

// TODO scroll
// TODO make proper error messages for errors
impl FakeMouse {
    /// It takes a while for kernel to create the virtual mouse so do not use it imidiately
    pub fn new() -> Result<Self> {
        #[cfg(feature = "evdev-rust")]
        {
            let device = VirtualDevice::builder()?
                .name("miceless")
                .with_relative_axes(&AttributeSet::from_iter([
                        RelativeAxisCode::REL_X,
                        RelativeAxisCode::REL_Y,
                        RelativeAxisCode::REL_WHEEL,
                        RelativeAxisCode::REL_HWHEEL,
                ]))?
                .with_keys(&AttributeSet::from_iter([
                        KeyCode::BTN_LEFT,
                        KeyCode::BTN_RIGHT,
                        KeyCode::BTN_MIDDLE,
                ]))?
                .build()?;

            Ok(Self {
                device,
            })
        }

        #[cfg(feature = "evdev-native")]
        {
            let u = UninitDevice::new().unwrap();

            u.set_name("Virtual Mouse");
            u.set_bustype(BusType::BUS_USB as u16);
            u.set_vendor_id(0xabcd);
            u.set_product_id(0xefef);

            // Note mouse keys have to be enabled for this to be detected
            // as a usable device, see: https://stackoverflow.com/a/64559658/6074942
            u.enable(EventCode::EV_KEY(EV_KEY::BTN_LEFT))?;
            u.enable(EventCode::EV_KEY(EV_KEY::BTN_RIGHT))?;
            u.enable(EventCode::EV_KEY(EV_KEY::BTN_MIDDLE))?;

            u.enable(EventCode::EV_REL(EV_REL::REL_X))?;
            u.enable(EventCode::EV_REL(EV_REL::REL_Y))?;

            u.enable(EventCode::EV_SYN(EV_SYN::SYN_REPORT))?;

            // Attempt to create UInputDevice from UninitDevice
            let device = UInputDevice::create_from_device(&u)?;

            Ok(Self {
                device,
            })
        }
    }

    /// Zero the mouse position (hacky avoid if possible)
    pub fn reset_position(&mut self) -> Result<()> {
        // NOTE reset position by moving to top left corner (HACK)
        // https://github.com/ReimuNotMoe/ydotool/blob/708e96ff27e381a8c549418a9d34cdde12305317/Client/tool_mousemove.c#L154
        self.rel_move(i32::MIN, i32::MIN)
    }

    pub fn rel_move(&mut self, x: i32, y: i32) -> Result<()> {
        #[cfg(feature = "evdev-rust")]
        {
            self.device.emit(&[
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, x),
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, y)
            ])?;
        }

        #[cfg(feature = "evdev-native")]
        {
            // TODO hopefully the kernel fills the time

            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_REL(EV_REL::REL_X),
                value: x,
            })?;

            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_REL(EV_REL::REL_Y),
                value: y,
            })?;

            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                value: 0,
            })?;
        }

        Ok(())
    }

    pub fn press(&mut self, key: MouseKey) -> Result<()> {
        #[cfg(feature = "evdev-rust")]
        {
            let event = *KeyEvent::new(KeyCode(key.keycode()), 1);
            self.device.emit(&[event])?;
        }

        #[cfg(feature = "evdev-native")]
        {
            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_KEY(key.into()),
                value: 1,
            })?;

            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                value: 0,
            })?;
        }

        Ok(())
    }

    pub fn release(&mut self, key: MouseKey) -> Result<()> {
        #[cfg(feature = "evdev-rust")]
        {
            let event = *KeyEvent::new(KeyCode(key.keycode()), 0);
            self.device.emit(&[event])?;
        }

        #[cfg(feature = "evdev-native")]
        {
            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_KEY(key.into()),
                value: 0,
            })?;

            self.device.write_event(&InputEvent {
                time: evdev_rs::TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                event_code: EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                value: 0,
            })?;
        }

        Ok(())
    }

    pub fn click(&mut self, key: MouseKey, hold_time: Option<Duration>) -> Result<()> {
        self.press(key)?;

        // wait
        std::thread::sleep(hold_time.unwrap_or(PRESS_RELEASE_DELAY_MIN));

        self.release(key)?;

        Ok(())
    }

    /// Sleep for predetermined time, used for delay between mouse actions
    pub fn sleep(&self) {
        std::thread::sleep(PRESS_RELEASE_DELAY_MIN);
    }
}
