pub use windows::core::PCSTR;
pub use windows::Win32::Foundation::{BOOL, HWND, LPARAM, POINT, RECT, WPARAM};

// Window Styles
pub mod styles {
    pub use windows::Win32::UI::WindowsAndMessaging::{GWL_EXSTYLE, GWL_STYLE};
    pub use windows::Win32::UI::WindowsAndMessaging::{
        WS_CAPTION, WS_EX_CLIENTEDGE, WS_EX_DLGMODALFRAME, WS_EX_STATICEDGE, WS_MAXIMIZEBOX,
        WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME,
    };
}

// Windowing
pub mod windowing {
    pub use windows::Win32::UI::WindowsAndMessaging::{
        FindWindowA as FindWindow,
        GetClientRect,
        GetCursorPos,
        GetForegroundWindow, // Forces clippy to put them once per line
        GetWindowLongPtrA as GetWindowLong,
        GetWindowRect,
        GetWindowTextA as GetWindowText,
        GetWindowTextLengthA as GetWindowTextLength,
        GetWindowThreadProcessId,
        MoveWindow,
        PostMessageA as PostMessage,
        SendMessageA as SendMessage,
        SetForegroundWindow,
        SetWindowLongPtrA as SetWindowLong,
        SetWindowPos,
        ShowWindow,
        SystemParametersInfoA as SystemParametersInfo,
    };

    pub use windows::Win32::UI::WindowsAndMessaging::{
        SHOW_WINDOW_CMD, SPI_GETWORKAREA, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOOWNERZORDER,
        SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
        WM_NCACTIVATE,
    };
}

pub mod messaging {
    pub use windows::Win32::UI::WindowsAndMessaging::{
        HTCLIENT, HWND_NOTOPMOST, HWND_TOP, HWND_TOPMOST, KF_REPEAT, MA_ACTIVATE, SW_HIDE,
        SW_RESTORE, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN,
        WM_MBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_PARENTNOTIFY,
        WM_RBUTTONDOWN, WM_RBUTTONUP,
    };
}

pub mod vk {
    // pub use windows::Win32::UI::Input::KeyboardAndMouseInput::*;
    pub use windows::Win32::System::SystemServices::{MK_LBUTTON, MK_MBUTTON, MK_RBUTTON};
    pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
    pub use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetKeyboardState};
}

pub mod threading {
    pub use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    pub use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
}

pub mod process {
    pub use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
}
