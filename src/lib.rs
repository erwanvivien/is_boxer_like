use std::collections::BTreeSet;

use egui::{Key, ScrollArea};
use windows::{
    messaging::{WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP},
    process::EnumWindows,
    styles::{
        GWL_EXSTYLE, GWL_STYLE, WS_CAPTION, WS_EX_CLIENTEDGE, WS_EX_DLGMODALFRAME,
        WS_EX_STATICEDGE, WS_MAXIMIZEBOX, WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME,
    },
    threading::{AttachThreadInput, GetCurrentThreadId},
    vk::{VIRTUAL_KEY, VK_LBUTTON},
    windowing::{
        GetClientRect, GetForegroundWindow, GetWindowLong, GetWindowRect, GetWindowText,
        GetWindowTextLength, GetWindowThreadProcessId, MoveWindow, PostMessage,
        SetForegroundWindow, SetWindowLong, SetWindowPos, SystemParametersInfo, SPI_GETWORKAREA,
        SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW,
        SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    },
    BOOL, HWND, LPARAM, RECT,
};

pub use windows::WPARAM;

pub enum Mode {
    Spamming,
    Duplicate,
}

pub struct App {
    main_hwnd: Option<HWND>,
    hwnds: Vec<HWND>,

    mode: Mode,
    keyboard: BTreeSet<VIRTUAL_KEY>,
}

impl App {
    pub fn new() -> Self {
        Self {
            hwnds: Vec::new(),
            main_hwnd: None,
            mode: Mode::Spamming,
            keyboard: BTreeSet::new(),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn update_windows<P: Into<String>>(&mut self, pattern: P) {
        struct HWNDCollector(Vec<HWND>, String);

        unsafe extern "system" fn callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let name_length = GetWindowTextLength(hwnd);
            let mut buffer = vec![0u8; name_length as usize + 1];
            GetWindowText(hwnd, &mut buffer);

            let hwnds_collector = &mut *(lparam.0 as *mut HWNDCollector);

            let name = String::from_utf8_lossy(&buffer);
            if name.to_lowercase().contains(&hwnds_collector.1) {
                hwnds_collector.0.push(hwnd);
            }

            // Return true to continue enumeration
            BOOL::from(true)
        }

        let pattern: String = pattern.into();
        let pattern = pattern.to_lowercase();

        let mut collector = HWNDCollector(Vec::new(), pattern);

        unsafe {
            // SAFETY: we are passing Collector as a pointer to the callback,
            // which will be used to store the HWNDs. No other thread is using it.
            EnumWindows(Some(callback), LPARAM(&mut collector as *mut _ as isize));
        };

        self.hwnds = collector.0;
    }

    pub fn layout_windows(&self) {
        if self.hwnds.is_empty() {
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

        let main_hwnd = self.main_hwnd.unwrap_or(self.hwnds[0]);
        for i in 0..self.hwnds.len() as i32 {
            unsafe {
                MoveWindow(
                    self.hwnds[i as usize],
                    (desktop_rect.right / 2) * i,
                    0,
                    desktop_rect.right / 2,
                    desktop_rect.bottom,
                    true,
                );
            };
        }
    }

    pub fn send_key(&self, key: WPARAM) {
        let lparam = LPARAM(key.0 as isize);
        for hwnd in self.hwnds.iter().cloned() {
            unsafe {
                PostMessage(hwnd, WM_KEYDOWN, key, lparam);
                PostMessage(hwnd, WM_KEYUP, key, lparam);
            };

            // TODO: Add random
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    pub fn send_mouse(&self, x: i32, y: i32) {
        let lparam_mousedown = LPARAM((x | (y << 16)) as isize);
        let lparam_mouseup = LPARAM((x | (y << 16) + 1) as isize);

        let wparam = WPARAM(usize::from(VK_LBUTTON.0));

        for hwnd in self.hwnds.iter().cloned() {
            unsafe {
                PostMessage(hwnd, WM_LBUTTONDOWN, wparam, lparam_mousedown);
                PostMessage(hwnd, WM_LBUTTONUP, wparam, lparam_mouseup);
            };

            // TODO: Add random
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("World of Warcraft are being mimicked");
            if ui.button("Clear").clicked() {
                self.hwnds.clear();
            }

            if ui.button("Get all windows").clicked() {
                self.update_windows("warcraft")
            }
        });
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

fn _get_rect(hwnd: HWND) -> RECT {
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

pub fn force_foreground_window(hwnd: HWND) -> bool {
    let current_hwnd = unsafe { GetForegroundWindow() };

    let current_thread_id = unsafe { GetCurrentThreadId() };
    let remote_thread = unsafe { GetWindowThreadProcessId(current_hwnd, None) };

    unsafe {
        AttachThreadInput(current_thread_id, remote_thread, true);
        SetForegroundWindow(hwnd);
        AttachThreadInput(current_thread_id, remote_thread, false);
    };

    return hwnd == unsafe { GetForegroundWindow() };
}
