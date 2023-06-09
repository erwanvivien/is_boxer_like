use std::collections::BTreeSet;

use windows::{
    messaging::{
        HWND_NOTOPMOST, HWND_TOPMOST, KF_REPEAT, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP,
    },
    process::EnumWindows,
    styles::{
        GWL_EXSTYLE, GWL_STYLE, WS_CAPTION, WS_EX_CLIENTEDGE, WS_EX_DLGMODALFRAME,
        WS_EX_STATICEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME,
    },
    vk::{self, GetAsyncKeyState, MK_LBUTTON},
    windowing::{
        GetClientRect, GetForegroundWindow, GetWindowLong, GetWindowRect, GetWindowText,
        GetWindowTextLength, MoveWindow, PostMessage, SetForegroundWindow, SetWindowLong,
        SetWindowPos, SystemParametersInfo, SPI_GETWORKAREA, SWP_FRAMECHANGED, SWP_NOMOVE,
        SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW,
        SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    },
    BOOL, HWND, LPARAM, RECT,
};

pub use windows::WPARAM;

mod config;
use crate::config::Key;
pub use crate::config::{BotAction, Config, LayoutOptions, Mode};

#[derive(Debug, Clone)]
struct Window {
    hwnd: HWND,
    title: String,
    rect: RECT,
}

pub struct App {
    main_hwnd: Option<HWND>,
    windows: Vec<Window>,

    keyboard: BTreeSet<usize>,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        let window_name = config.window_name.clone();
        let mut app = Self {
            windows: Vec::new(),
            main_hwnd: None,
            keyboard: BTreeSet::new(),
            config,
        };

        app.update_windows(window_name);
        app
    }

    pub fn is_main_focus(&self) -> bool {
        unsafe { Some(GetForegroundWindow()) == self.main_hwnd }
    }

    pub fn has_hwnd(&self, hwnd: HWND) -> bool {
        self.windows.iter().any(|w| w.hwnd == hwnd)
    }

    pub fn update_windows<P: Into<String>>(&mut self, pattern: P) {
        struct HWNDCollector(Vec<Window>, String);

        unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let name_length = GetWindowTextLength(hwnd);
            let mut buffer = vec![0u8; name_length as usize + 1];
            GetWindowText(hwnd, &mut buffer);

            let hwnds_collector = &mut *(lparam.0 as *mut HWNDCollector);

            let name = String::from_utf8_lossy(&buffer);
            if name.to_lowercase().contains(&hwnds_collector.1) {
                hwnds_collector.0.push(Window {
                    hwnd,
                    title: name.to_string(),
                    rect: get_rect(hwnd),
                });
            }

            // Return true to continue enumeration
            BOOL::from(true)
        }

        let pattern = pattern.into().to_lowercase();
        let mut collector = HWNDCollector(Vec::new(), pattern);

        unsafe {
            // SAFETY: we are passing Collector as a pointer to the callback,
            // which will be used to store the HWNDs. No other thread is using it.
            EnumWindows(Some(callback), LPARAM(&mut collector as *mut _ as isize));
        };

        self.windows = collector.0;
        self.main_hwnd = Some(self.windows[0].hwnd)
    }

    /// Returns true if the main loop needs to be restarted
    pub fn global_shortcuts(&mut self) -> bool {
        use config::Shortcut;

        // If the user is not pressing the shortcut keys, ignore (LShift + LAlt)
        if unsafe { GetAsyncKeyState(i32::from(vk::VK_LSHIFT.0)) } as u16 & 0x8000 == 0
            || unsafe { GetAsyncKeyState(i32::from(vk::VK_LMENU.0)) } as u16 & 0x8000 == 0
        {
            return false;
        }

        let shortcuts = &self.config.shortcuts.clone();
        let shortcuts_keys = shortcuts.keys().cloned().collect::<Vec<_>>();

        let mut update = false;
        for key in shortcuts_keys {
            if unsafe { GetAsyncKeyState(key as i32) as u32 } & 0x8000 == 0 {
                continue;
            }

            let action = shortcuts.get(&key).unwrap();
            match action {
                Shortcut::Foreground => self.foreground(),
                Shortcut::Layout => {
                    let foreground_hwnd = unsafe { GetForegroundWindow() };
                    if self.has_hwnd(foreground_hwnd) {
                        self.main_hwnd = Some(foreground_hwnd);
                    }

                    self.layout_windows()
                }
                Shortcut::Mode(mode) => {
                    self.config.mode = mode.clone();
                    update = true;
                }
            }
        }

        update
    }

    pub fn get_foreground_window(&mut self) -> bool {
        let hwnd = unsafe { GetForegroundWindow() };
        if Some(hwnd) == self.main_hwnd {
            return false;
        }

        if self.has_hwnd(hwnd) {
            self.main_hwnd = Some(hwnd);
        }
        return true;
    }

    pub fn mimic(&mut self) {
        enum KeyState {
            Insert,
            Remove,
            None,
        }

        fn mimic_key(app: &App, key: Key, remaped_key: Key, other_hwnds: &[Window]) -> KeyState {
            let state = unsafe { GetAsyncKeyState(key as i32) } as u16;
            let key_already_pressed = app.keyboard.contains(&(key as usize));
            if state & 0x8000 != 0 {
                app.send_key_hwnds(WM_KEYDOWN, WPARAM(remaped_key as usize), &other_hwnds);
                if key_already_pressed {
                    KeyState::None
                } else {
                    KeyState::Insert
                }
            } else if key_already_pressed {
                app.send_key_hwnds(WM_KEYUP, WPARAM(remaped_key as usize), &other_hwnds);
                KeyState::Remove
            } else {
                KeyState::None
            }
        }

        let main_hwnd = unsafe { GetForegroundWindow() };
        if !self.has_hwnd(main_hwnd) {
            return;
        }

        let other_hwnds = self
            .windows
            .iter()
            .cloned()
            .filter(|window| window.hwnd != main_hwnd)
            .collect::<Vec<_>>();

        let skip_keybind = &self.config.skip_keybind;
        let remap_keybind = &self.config.remap_keybind;

        for key in &self.config.keybind {
            if skip_keybind.contains(key) || remap_keybind.contains_key(key) {
                continue;
            }

            match mimic_key(self, *key, *key, &other_hwnds) {
                KeyState::Insert => self.keyboard.insert(*key as usize),
                KeyState::Remove => self.keyboard.remove(&(*key as usize)),
                KeyState::None => false,
            };
        }

        for (key, remaped_key) in remap_keybind {
            match mimic_key(self, *key, *remaped_key, &other_hwnds) {
                KeyState::Insert => self.keyboard.insert(*key as usize),
                KeyState::Remove => self.keyboard.remove(&(*key as usize)),
                KeyState::None => false,
            };
        }

        // for key in [VK_LBUTTON, VK_RBUTTON, VK_MBUTTON].iter() {
        //     let state = unsafe { GetAsyncKeyState(i32::from(key.0)) };
        //     if state & (1 << (i16::BITS - 1)) != 0 {
        //         let mut mouse_pos = POINT::default();
        //         unsafe {
        //             GetCursorPos(&mut mouse_pos);
        //         }

        //         dbg!(mouse_pos);

        //         self.send_mouse_hwnds(
        //             WPARAM(key.0 as usize),
        //             mouse_pos.x,
        //             mouse_pos.y,
        //             &other_hwnds,
        //         )
        //     }
        // }
    }

    pub fn swap_windows(&mut self) -> bool {
        if self.windows.len() <= 1 || self.main_hwnd.is_none() || self.is_main_focus() {
            return false;
        }

        let foreground_hwnd = unsafe { GetForegroundWindow() };
        if !self.has_hwnd(foreground_hwnd) {
            return false;
        }

        let current_main_hwnd = self.main_hwnd.unwrap();
        let mut windows = self
            .windows
            .iter_mut()
            .filter(|w| w.hwnd == foreground_hwnd || w.hwnd == current_main_hwnd)
            .collect::<Vec<_>>();

        let current = windows.pop().unwrap();
        let main = windows.pop().unwrap();

        MoveWindowRect(main.hwnd, &current.rect, true);
        MoveWindowRect(current.hwnd, &main.rect, true);

        std::mem::swap(&mut main.rect, &mut current.rect);
        self.main_hwnd = Some(foreground_hwnd);

        unsafe {
            SetForegroundWindow(foreground_hwnd);
        }

        true
    }

    pub fn layout_windows(&mut self) {
        if self.windows.len() <= 1 || self.main_hwnd.is_none() {
            return;
        }

        let mut desktop_rect = RECT::default();
        let c_void_desktop_rect = &mut desktop_rect as *mut RECT as *mut _;
        unsafe {
            SystemParametersInfo(
                SPI_GETWORKAREA,
                0,
                Some(c_void_desktop_rect),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            );
        }

        let RECT {
            left: _,
            top: _,
            right: main_width,
            bottom: main_height,
        } = desktop_rect;

        let mut i = 0i32;

        let small_width = main_width / (self.windows.len() - 1) as i32;
        let small_height = main_height / self.windows.len() as i32;

        for window in self.windows.iter_mut() {
            if window.hwnd == self.main_hwnd.unwrap() {
                window.rect = RECT {
                    left: 0,
                    top: 0,
                    right: main_width,
                    bottom: main_height - small_height,
                };
            } else {
                window.rect = RECT {
                    left: i * small_width,
                    top: main_height - small_height,
                    right: small_width,
                    bottom: small_height,
                };
                i += 1;
            };

            let rect = window.rect;
            set_borders(window.hwnd, false);
            MoveWindowRect(window.hwnd, &rect, true);
        }
    }

    pub fn foreground(&self) {
        for window in self.windows.iter() {
            // Sets the window to be foreground always on top
            SetWindowLevel(window.hwnd, HWND_TOPMOST);
            // Sets the window to be a normal window again
            SetWindowLevel(window.hwnd, HWND_NOTOPMOST);
        }
    }

    pub fn send_key_down(&self, key: WPARAM) {
        self.send_key_hwnds(WM_KEYDOWN, key, &self.windows);
    }

    pub fn send_key_up(&self, key: WPARAM) {
        self.send_key_hwnds(WM_KEYUP, key, &self.windows);
    }

    fn send_key_hwnds(&self, message: u32, key: WPARAM, hwnds: &[Window]) {
        // https://learn.microsoft.com/en-us/windows/win32/inputdev/about-keyboard-input#keystroke-message-flags
        let mut flags = (key.0 as u32) << 16;
        if self.keyboard.contains(&key.0) {
            flags |= KF_REPEAT;
        }

        let lparam = LPARAM(flags as isize);
        for window in hwnds.iter() {
            unsafe {
                PostMessage(window.hwnd, message, key, lparam);
            };

            // TODO: Add random
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    pub fn send_mouse(&self, key: WPARAM, x: i32, y: i32) {
        self.send_mouse_hwnds(key, x, y, &self.windows);
    }

    fn send_mouse_hwnds(&self, _key: WPARAM, x: i32, y: i32, hwnds: &[Window]) {
        let make_lparam = |x: i32, y: i32| LPARAM((x | (y << 16)) as isize);

        let lparam_mousedown = make_lparam(x, y);
        let _lparam_mouseup = make_lparam(x + 1, y + 1);

        for window in hwnds.iter() {
            unsafe {
                PostMessage(
                    window.hwnd,
                    WM_LBUTTONDOWN,
                    WPARAM(MK_LBUTTON.0 as usize),
                    lparam_mousedown,
                );
                PostMessage(window.hwnd, WM_LBUTTONUP, WPARAM(0), lparam_mousedown);
            };

            // TODO: Add random
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("World of Warcraft are being mimic-ed");
            if ui.button("Clear").clicked() {
                self.windows.clear();
            }

            if ui.button("Get all windows").clicked() {
                self.update_windows("warcraft")
            }
        });
    }
}

#[allow(non_snake_case)]
pub fn SetWindowLevel(hwnd: HWND, level: HWND) {
    unsafe {
        SetWindowPos(
            hwnd,
            level,
            0,
            0,
            0,
            0,
            SWP_SHOWWINDOW | SWP_NOSIZE | SWP_NOMOVE,
        );
    }
}

#[allow(non_snake_case)]
pub fn MoveWindowRect(hwnd: HWND, rect: &RECT, repaint: bool) {
    unsafe {
        MoveWindow(hwnd, rect.left, rect.top, rect.right, rect.bottom, repaint);
    }
}

pub fn set_borders(hwnd: HWND, borders: bool) {
    let [mut l_style, mut l_ex_style] = [
        unsafe { GetWindowLong(hwnd, GWL_STYLE) }, // Forces clippy to put them once per line
        unsafe { GetWindowLong(hwnd, GWL_EXSTYLE) },
    ];

    let lstyle_mask =
        (WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU).0 as isize;
    let l_ex_style_mask = (WS_EX_DLGMODALFRAME | WS_EX_CLIENTEDGE | WS_EX_STATICEDGE).0 as isize;

    if !borders {
        l_style &= !lstyle_mask;
        l_ex_style &= !l_ex_style_mask;
    } else {
        l_style |= lstyle_mask;
        l_ex_style |= l_ex_style_mask;
    }

    let uflags = SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOOWNERZORDER;
    unsafe {
        SetWindowLong(hwnd, GWL_STYLE, l_style);
        SetWindowLong(hwnd, GWL_EXSTYLE, l_ex_style);
        SetWindowPos(hwnd, HWND(0), 0, 0, 0, 0, uflags);
    };
}

fn get_rect(hwnd: HWND) -> RECT {
    let mut rect1 = RECT::default();
    let mut rect2 = RECT::default();
    unsafe {
        GetWindowRect(hwnd, &mut rect1);
        GetClientRect(hwnd, &mut rect2);
    }

    RECT {
        bottom: rect2.bottom,
        right: rect2.right,
        ..rect1
    }
}
