pub use windows::core::PCSTR;
pub use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT, WPARAM};

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
        GetForegroundWindow, // Forces clippy to put them once per line
        GetWindowLongPtrA as GetWindowLong,
        GetWindowRect,
        GetWindowTextA as GetWindowText,
        GetWindowTextLengthA as GetWindowTextLength,
        GetWindowThreadProcessId,
        MoveWindow,
        PostMessageA as PostMessage,
        SetForegroundWindow,
        SetWindowLongPtrA as SetWindowLong,
        SetWindowPos,
        SystemParametersInfoA as SystemParametersInfo,
    };

    pub use windows::Win32::UI::WindowsAndMessaging::{
        SPI_GETWORKAREA, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOSIZE, SWP_NOZORDER,
        SWP_SHOWWINDOW, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };
}

pub mod messaging {
    pub use windows::Win32::UI::WindowsAndMessaging::{
        WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
        WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
    };
}

pub mod vk {
    // pub use windows::Win32::UI::Input::KeyboardAndMouseInput::*;
    pub use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    pub use windows::Win32::UI::Input::KeyboardAndMouse::*;
}

pub mod threading {
    pub use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    pub use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
}

pub mod process {
    pub use windows::Win32::UI::WindowsAndMessaging::EnumWindows;
}
