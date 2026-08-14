//! Contains code for mouse emulation

use evdev::{AbsInfo, AbsoluteAxisCode, AttributeSet, EventType, InputEvent, KeyCode, KeyEvent, RelativeAxisCode, UinputAbsSetup, uinput::VirtualDevice};

    /*
    // TODO silent error when uinput is not rw
    let mut device = VirtualDevice::builder()?
        .name("fake-mouse")
        .with_relative_axes(&AttributeSet::from_iter([
            RelativeAxisCode::REL_X,
            RelativeAxisCode::REL_Y,
            RelativeAxisCode::REL_WHEEL,
            RelativeAxisCode::REL_HWHEEL,
        ]))?
        .with_keys(&AttributeSet::from_iter([
                KeyCode::BTN_LEFT,
                KeyCode::BTN_RIGHT,
                KeyCode::BTN_TOUCH,
        ]))?
        .build()?;

    // wait for the device (required otherwise it does not work)
    std::thread::sleep(std::time::Duration::from_millis(200));

    // TODO could i get position of the cursor using the overlay window? then i dont have to use the
    // hack below!
    //
    // NOTE reset position by moving to top left corner (HACK)
    // https://github.com/ReimuNotMoe/ydotool/blob/708e96ff27e381a8c549418a9d34cdde12305317/Client/tool_mousemove.c#L154
    let event1 = InputEvent::new_now(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, i32::MIN);
    let event2 = InputEvent::new_now(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, i32::MIN);
    device.emit(&[event1, event2])?;

    std::thread::sleep(std::time::Duration::from_millis(5));

    let event1 = InputEvent::new_now(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, i32::from(250));
    let event2 = InputEvent::new_now(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, i32::from(250));
    device.emit(&[event1, event2])?;

    std::thread::sleep(std::time::Duration::from_millis(5));

    // press
    let event = *KeyEvent::new(KeyCode(KeyCode::BTN_LEFT.0), 1);
    device.emit(&[event])?;

    std::thread::sleep(std::time::Duration::from_millis(5));

    // release
    let event = *KeyEvent::new(KeyCode(KeyCode::BTN_LEFT.0), 0);
    device.emit(&[event])?;*/
