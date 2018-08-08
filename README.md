[![Plop Grizzly](https://plopgrizzly.com/images/logo-bar.png)](https://plopgrizzly.com)

# [Aldaron's Window Interface](https://crates.io/crates/awi)
Create a window and handle it's input

This project is part of [ADI](https://crates.io/crates/adi).

## Features
* Create a window
* Poll the window's input
* Get the window's native handle

## Platform Support
Plans are for awi to support all of the following platforms (or any other
platform that exists).  AWI's goal is to be able to run on any computer.
If you see a missing platform you would like supported, open an issue.

| platform           | support |
| --------           | ------- |
| 1. Windows         | Yes (windows) |
| 2. Linux / BSD     | Yes (linux - TODO: Wayland, fallback on XCB) |
| 3. Raspberry Pi¹   | TODO (rpi - directfb) |
| 4. Deskron²        | TODO (deskron) |
| 5. Android         | TODO (android) |
| 6. MacOS / iOS     | TODO (apple - TODO: Cocoa) |
| 7. Web App         | TODO (wasm32) |
| 8. Nintendo Switch | TODO (switch) |
| 9. Redox           | TODO (redox) |
| 10. XBox One       | TODO (xbox) |

* ¹ Needs feature `directfb` enabled, since target_os is linux.
* ² Needs feature `deskron` enabled, since the DE can be installed on any OS.

## Roadmap to 1.0 (Future Features)
* Complete the first 9 platforms.

## Roadmap to 1.1
* Complete the first 10 platforms.

## Change Log
### 0.8
* Update afi
* Update dl_api

### 0.7
* Use c_void in std instead of in libc, since libc isn't needed on Windows.

### 0.6
* Simplified input.
* Added support for joysticks / controllers.
