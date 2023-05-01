use std::collections::HashMap;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::{io::Read, path::Path};

use windows::vk;
use windows::vk::VIRTUAL_KEY;

use serde::Deserialize;

// Ignore case
#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Key {
    VK_0 = vk::VK_0.0 as isize,
    VK_1 = vk::VK_1.0 as isize,
    VK_2 = vk::VK_2.0 as isize,
    VK_3 = vk::VK_3.0 as isize,
    VK_4 = vk::VK_4.0 as isize,
    VK_5 = vk::VK_5.0 as isize,
    VK_6 = vk::VK_6.0 as isize,
    VK_7 = vk::VK_7.0 as isize,
    VK_8 = vk::VK_8.0 as isize,
    VK_9 = vk::VK_9.0 as isize,
    VK_A = vk::VK_A.0 as isize,
    VK_B = vk::VK_B.0 as isize,
    VK_C = vk::VK_C.0 as isize,
    VK_D = vk::VK_D.0 as isize,
    VK_E = vk::VK_E.0 as isize,
    VK_F = vk::VK_F.0 as isize,
    VK_G = vk::VK_G.0 as isize,
    VK_H = vk::VK_H.0 as isize,
    VK_I = vk::VK_I.0 as isize,
    VK_J = vk::VK_J.0 as isize,
    VK_K = vk::VK_K.0 as isize,
    VK_L = vk::VK_L.0 as isize,
    VK_M = vk::VK_M.0 as isize,
    VK_N = vk::VK_N.0 as isize,
    VK_O = vk::VK_O.0 as isize,
    VK_P = vk::VK_P.0 as isize,
    VK_Q = vk::VK_Q.0 as isize,
    VK_R = vk::VK_R.0 as isize,
    VK_S = vk::VK_S.0 as isize,
    VK_T = vk::VK_T.0 as isize,
    VK_U = vk::VK_U.0 as isize,
    VK_V = vk::VK_V.0 as isize,
    VK_W = vk::VK_W.0 as isize,
    VK_X = vk::VK_X.0 as isize,
    VK_Y = vk::VK_Y.0 as isize,
    VK_Z = vk::VK_Z.0 as isize,
    VK_ABNT_C1 = vk::VK_ABNT_C1.0 as isize,
    VK_ABNT_C2 = vk::VK_ABNT_C2.0 as isize,
    VK__none_ = vk::VK__none_.0 as isize,
    VK_LBUTTON = vk::VK_LBUTTON.0 as isize,
    VK_RBUTTON = vk::VK_RBUTTON.0 as isize,
    VK_CANCEL = vk::VK_CANCEL.0 as isize,
    VK_MBUTTON = vk::VK_MBUTTON.0 as isize,
    VK_XBUTTON1 = vk::VK_XBUTTON1.0 as isize,
    VK_XBUTTON2 = vk::VK_XBUTTON2.0 as isize,
    VK_BACK = vk::VK_BACK.0 as isize,
    VK_TAB = vk::VK_TAB.0 as isize,
    VK_CLEAR = vk::VK_CLEAR.0 as isize,
    VK_RETURN = vk::VK_RETURN.0 as isize,
    VK_SHIFT = vk::VK_SHIFT.0 as isize,
    VK_CONTROL = vk::VK_CONTROL.0 as isize,
    VK_MENU = vk::VK_MENU.0 as isize,
    VK_PAUSE = vk::VK_PAUSE.0 as isize,
    VK_CAPITAL = vk::VK_CAPITAL.0 as isize,
    VK_KANA = vk::VK_KANA.0 as isize,
    VK_IME_ON = vk::VK_IME_ON.0 as isize,
    VK_JUNJA = vk::VK_JUNJA.0 as isize,
    VK_FINAL = vk::VK_FINAL.0 as isize,
    VK_KANJI = vk::VK_KANJI.0 as isize,
    VK_IME_OFF = vk::VK_IME_OFF.0 as isize,
    VK_ESCAPE = vk::VK_ESCAPE.0 as isize,
    VK_CONVERT = vk::VK_CONVERT.0 as isize,
    VK_NONCONVERT = vk::VK_NONCONVERT.0 as isize,
    VK_ACCEPT = vk::VK_ACCEPT.0 as isize,
    VK_MODECHANGE = vk::VK_MODECHANGE.0 as isize,
    VK_SPACE = vk::VK_SPACE.0 as isize,
    VK_PRIOR = vk::VK_PRIOR.0 as isize,
    VK_NEXT = vk::VK_NEXT.0 as isize,
    VK_END = vk::VK_END.0 as isize,
    VK_HOME = vk::VK_HOME.0 as isize,
    VK_LEFT = vk::VK_LEFT.0 as isize,
    VK_UP = vk::VK_UP.0 as isize,
    VK_RIGHT = vk::VK_RIGHT.0 as isize,
    VK_DOWN = vk::VK_DOWN.0 as isize,
    VK_SELECT = vk::VK_SELECT.0 as isize,
    VK_PRINT = vk::VK_PRINT.0 as isize,
    VK_EXECUTE = vk::VK_EXECUTE.0 as isize,
    VK_SNAPSHOT = vk::VK_SNAPSHOT.0 as isize,
    VK_INSERT = vk::VK_INSERT.0 as isize,
    VK_DELETE = vk::VK_DELETE.0 as isize,
    VK_HELP = vk::VK_HELP.0 as isize,
    VK_LWIN = vk::VK_LWIN.0 as isize,
    VK_RWIN = vk::VK_RWIN.0 as isize,
    VK_APPS = vk::VK_APPS.0 as isize,
    VK_SLEEP = vk::VK_SLEEP.0 as isize,
    VK_NUMPAD0 = vk::VK_NUMPAD0.0 as isize,
    VK_NUMPAD1 = vk::VK_NUMPAD1.0 as isize,
    VK_NUMPAD2 = vk::VK_NUMPAD2.0 as isize,
    VK_NUMPAD3 = vk::VK_NUMPAD3.0 as isize,
    VK_NUMPAD4 = vk::VK_NUMPAD4.0 as isize,
    VK_NUMPAD5 = vk::VK_NUMPAD5.0 as isize,
    VK_NUMPAD6 = vk::VK_NUMPAD6.0 as isize,
    VK_NUMPAD7 = vk::VK_NUMPAD7.0 as isize,
    VK_NUMPAD8 = vk::VK_NUMPAD8.0 as isize,
    VK_NUMPAD9 = vk::VK_NUMPAD9.0 as isize,
    VK_MULTIPLY = vk::VK_MULTIPLY.0 as isize,
    VK_ADD = vk::VK_ADD.0 as isize,
    VK_SEPARATOR = vk::VK_SEPARATOR.0 as isize,
    VK_SUBTRACT = vk::VK_SUBTRACT.0 as isize,
    VK_DECIMAL = vk::VK_DECIMAL.0 as isize,
    VK_DIVIDE = vk::VK_DIVIDE.0 as isize,
    VK_F1 = vk::VK_F1.0 as isize,
    VK_F2 = vk::VK_F2.0 as isize,
    VK_F3 = vk::VK_F3.0 as isize,
    VK_F4 = vk::VK_F4.0 as isize,
    VK_F5 = vk::VK_F5.0 as isize,
    VK_F6 = vk::VK_F6.0 as isize,
    VK_F7 = vk::VK_F7.0 as isize,
    VK_F8 = vk::VK_F8.0 as isize,
    VK_F9 = vk::VK_F9.0 as isize,
    VK_F10 = vk::VK_F10.0 as isize,
    VK_F11 = vk::VK_F11.0 as isize,
    VK_F12 = vk::VK_F12.0 as isize,
    VK_F13 = vk::VK_F13.0 as isize,
    VK_F14 = vk::VK_F14.0 as isize,
    VK_F15 = vk::VK_F15.0 as isize,
    VK_F16 = vk::VK_F16.0 as isize,
    VK_F17 = vk::VK_F17.0 as isize,
    VK_F18 = vk::VK_F18.0 as isize,
    VK_F19 = vk::VK_F19.0 as isize,
    VK_F20 = vk::VK_F20.0 as isize,
    VK_F21 = vk::VK_F21.0 as isize,
    VK_F22 = vk::VK_F22.0 as isize,
    VK_F23 = vk::VK_F23.0 as isize,
    VK_F24 = vk::VK_F24.0 as isize,
    VK_NAVIGATION_VIEW = vk::VK_NAVIGATION_VIEW.0 as isize,
    VK_NAVIGATION_MENU = vk::VK_NAVIGATION_MENU.0 as isize,
    VK_NAVIGATION_UP = vk::VK_NAVIGATION_UP.0 as isize,
    VK_NAVIGATION_DOWN = vk::VK_NAVIGATION_DOWN.0 as isize,
    VK_NAVIGATION_LEFT = vk::VK_NAVIGATION_LEFT.0 as isize,
    VK_NAVIGATION_RIGHT = vk::VK_NAVIGATION_RIGHT.0 as isize,
    VK_NAVIGATION_ACCEPT = vk::VK_NAVIGATION_ACCEPT.0 as isize,
    VK_NAVIGATION_CANCEL = vk::VK_NAVIGATION_CANCEL.0 as isize,
    VK_NUMLOCK = vk::VK_NUMLOCK.0 as isize,
    VK_SCROLL = vk::VK_SCROLL.0 as isize,
    VK_OEM_NEC_EQUAL = vk::VK_OEM_NEC_EQUAL.0 as isize,
    VK_OEM_FJ_MASSHOU = vk::VK_OEM_FJ_MASSHOU.0 as isize,
    VK_OEM_FJ_TOUROKU = vk::VK_OEM_FJ_TOUROKU.0 as isize,
    VK_OEM_FJ_LOYA = vk::VK_OEM_FJ_LOYA.0 as isize,
    VK_OEM_FJ_ROYA = vk::VK_OEM_FJ_ROYA.0 as isize,
    VK_LSHIFT = vk::VK_LSHIFT.0 as isize,
    VK_RSHIFT = vk::VK_RSHIFT.0 as isize,
    VK_LCONTROL = vk::VK_LCONTROL.0 as isize,
    VK_RCONTROL = vk::VK_RCONTROL.0 as isize,
    VK_LMENU = vk::VK_LMENU.0 as isize,
    VK_RMENU = vk::VK_RMENU.0 as isize,
    VK_BROWSER_BACK = vk::VK_BROWSER_BACK.0 as isize,
    VK_BROWSER_FORWARD = vk::VK_BROWSER_FORWARD.0 as isize,
    VK_BROWSER_REFRESH = vk::VK_BROWSER_REFRESH.0 as isize,
    VK_BROWSER_STOP = vk::VK_BROWSER_STOP.0 as isize,
    VK_BROWSER_SEARCH = vk::VK_BROWSER_SEARCH.0 as isize,
    VK_BROWSER_FAVORITES = vk::VK_BROWSER_FAVORITES.0 as isize,
    VK_BROWSER_HOME = vk::VK_BROWSER_HOME.0 as isize,
    VK_VOLUME_MUTE = vk::VK_VOLUME_MUTE.0 as isize,
    VK_VOLUME_DOWN = vk::VK_VOLUME_DOWN.0 as isize,
    VK_VOLUME_UP = vk::VK_VOLUME_UP.0 as isize,
    VK_MEDIA_NEXT_TRACK = vk::VK_MEDIA_NEXT_TRACK.0 as isize,
    VK_MEDIA_PREV_TRACK = vk::VK_MEDIA_PREV_TRACK.0 as isize,
    VK_MEDIA_STOP = vk::VK_MEDIA_STOP.0 as isize,
    VK_MEDIA_PLAY_PAUSE = vk::VK_MEDIA_PLAY_PAUSE.0 as isize,
    VK_LAUNCH_MAIL = vk::VK_LAUNCH_MAIL.0 as isize,
    VK_LAUNCH_MEDIA_SELECT = vk::VK_LAUNCH_MEDIA_SELECT.0 as isize,
    VK_LAUNCH_APP1 = vk::VK_LAUNCH_APP1.0 as isize,
    VK_LAUNCH_APP2 = vk::VK_LAUNCH_APP2.0 as isize,
    VK_OEM_1 = vk::VK_OEM_1.0 as isize,
    VK_OEM_PLUS = vk::VK_OEM_PLUS.0 as isize,
    VK_OEM_COMMA = vk::VK_OEM_COMMA.0 as isize,
    VK_OEM_MINUS = vk::VK_OEM_MINUS.0 as isize,
    VK_OEM_PERIOD = vk::VK_OEM_PERIOD.0 as isize,
    VK_OEM_2 = vk::VK_OEM_2.0 as isize,
    VK_OEM_3 = vk::VK_OEM_3.0 as isize,
    VK_GAMEPAD_A = vk::VK_GAMEPAD_A.0 as isize,
    VK_GAMEPAD_B = vk::VK_GAMEPAD_B.0 as isize,
    VK_GAMEPAD_X = vk::VK_GAMEPAD_X.0 as isize,
    VK_GAMEPAD_Y = vk::VK_GAMEPAD_Y.0 as isize,
    VK_GAMEPAD_RIGHT_SHOULDER = vk::VK_GAMEPAD_RIGHT_SHOULDER.0 as isize,
    VK_GAMEPAD_LEFT_SHOULDER = vk::VK_GAMEPAD_LEFT_SHOULDER.0 as isize,
    VK_GAMEPAD_LEFT_TRIGGER = vk::VK_GAMEPAD_LEFT_TRIGGER.0 as isize,
    VK_GAMEPAD_RIGHT_TRIGGER = vk::VK_GAMEPAD_RIGHT_TRIGGER.0 as isize,
    VK_GAMEPAD_DPAD_UP = vk::VK_GAMEPAD_DPAD_UP.0 as isize,
    VK_GAMEPAD_DPAD_DOWN = vk::VK_GAMEPAD_DPAD_DOWN.0 as isize,
    VK_GAMEPAD_DPAD_LEFT = vk::VK_GAMEPAD_DPAD_LEFT.0 as isize,
    VK_GAMEPAD_DPAD_RIGHT = vk::VK_GAMEPAD_DPAD_RIGHT.0 as isize,
    VK_GAMEPAD_MENU = vk::VK_GAMEPAD_MENU.0 as isize,
    VK_GAMEPAD_VIEW = vk::VK_GAMEPAD_VIEW.0 as isize,
    VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON = vk::VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON.0 as isize,
    VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON = vk::VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON.0 as isize,
    VK_GAMEPAD_LEFT_THUMBSTICK_UP = vk::VK_GAMEPAD_LEFT_THUMBSTICK_UP.0 as isize,
    VK_GAMEPAD_LEFT_THUMBSTICK_DOWN = vk::VK_GAMEPAD_LEFT_THUMBSTICK_DOWN.0 as isize,
    VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT = vk::VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT.0 as isize,
    VK_GAMEPAD_LEFT_THUMBSTICK_LEFT = vk::VK_GAMEPAD_LEFT_THUMBSTICK_LEFT.0 as isize,
    VK_GAMEPAD_RIGHT_THUMBSTICK_UP = vk::VK_GAMEPAD_RIGHT_THUMBSTICK_UP.0 as isize,
    VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN = vk::VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN.0 as isize,
    VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT = vk::VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT.0 as isize,
    VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT = vk::VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT.0 as isize,
    VK_OEM_4 = vk::VK_OEM_4.0 as isize,
    VK_OEM_5 = vk::VK_OEM_5.0 as isize,
    VK_OEM_6 = vk::VK_OEM_6.0 as isize,
    VK_OEM_7 = vk::VK_OEM_7.0 as isize,
    VK_OEM_8 = vk::VK_OEM_8.0 as isize,
    VK_OEM_AX = vk::VK_OEM_AX.0 as isize,
    VK_OEM_102 = vk::VK_OEM_102.0 as isize,
    VK_ICO_HELP = vk::VK_ICO_HELP.0 as isize,
    VK_ICO_00 = vk::VK_ICO_00.0 as isize,
    VK_PROCESSKEY = vk::VK_PROCESSKEY.0 as isize,
    VK_ICO_CLEAR = vk::VK_ICO_CLEAR.0 as isize,
    VK_PACKET = vk::VK_PACKET.0 as isize,
    VK_OEM_RESET = vk::VK_OEM_RESET.0 as isize,
    VK_OEM_JUMP = vk::VK_OEM_JUMP.0 as isize,
    VK_OEM_PA1 = vk::VK_OEM_PA1.0 as isize,
    VK_OEM_PA2 = vk::VK_OEM_PA2.0 as isize,
    VK_OEM_PA3 = vk::VK_OEM_PA3.0 as isize,
    VK_OEM_WSCTRL = vk::VK_OEM_WSCTRL.0 as isize,
    VK_OEM_CUSEL = vk::VK_OEM_CUSEL.0 as isize,
    VK_OEM_ATTN = vk::VK_OEM_ATTN.0 as isize,
    VK_OEM_FINISH = vk::VK_OEM_FINISH.0 as isize,
    VK_OEM_COPY = vk::VK_OEM_COPY.0 as isize,
    VK_OEM_AUTO = vk::VK_OEM_AUTO.0 as isize,
    VK_OEM_ENLW = vk::VK_OEM_ENLW.0 as isize,
    VK_OEM_BACKTAB = vk::VK_OEM_BACKTAB.0 as isize,
    VK_ATTN = vk::VK_ATTN.0 as isize,
    VK_CRSEL = vk::VK_CRSEL.0 as isize,
    VK_EXSEL = vk::VK_EXSEL.0 as isize,
    VK_EREOF = vk::VK_EREOF.0 as isize,
    VK_PLAY = vk::VK_PLAY.0 as isize,
    VK_ZOOM = vk::VK_ZOOM.0 as isize,
    VK_NONAME = vk::VK_NONAME.0 as isize,
    VK_PA1 = vk::VK_PA1.0 as isize,
    VK_OEM_CLEAR = vk::VK_OEM_CLEAR.0 as isize,
}

impl Into<VIRTUAL_KEY> for Key {
    fn into(self) -> VIRTUAL_KEY {
        VIRTUAL_KEY(self as u16)
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub enum Duration {
    Milliseconds(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Weeks(u64),
    Months(u64),
    Years(u64),
}

impl Into<std::time::Duration> for Duration {
    fn into(self) -> std::time::Duration {
        match self {
            Duration::Milliseconds(n) => std::time::Duration::from_millis(n),
            Duration::Seconds(n) => std::time::Duration::from_secs(n),
            Duration::Minutes(n) => std::time::Duration::from_secs(n * 60),
            Duration::Hours(n) => std::time::Duration::from_secs(n * 60 * 60),
            Duration::Days(n) => std::time::Duration::from_secs(n * 60 * 60 * 24),
            Duration::Weeks(n) => std::time::Duration::from_secs(n * 60 * 60 * 24 * 7),
            Duration::Months(n) => std::time::Duration::from_secs(n * 60 * 60 * 24 * 30),
            Duration::Years(n) => std::time::Duration::from_secs(n * 60 * 60 * 24 * 365),
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum BotAction {
    Sleep(Duration),
    MouseTo(usize, usize),
    KeyStroke(Key),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum Mode {
    Mimic(Duration),
    Bot(Vec<BotAction>),
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum LayoutOptions {
    Never,
    Init,
    Always,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        LayoutOptions::Init
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Config {
    pub window_name: String,
    #[serde(default)]
    pub layout: LayoutOptions,
    pub mode: Mode,
    #[serde(default = "default_remap")]
    pub remap_keybind: HashMap<Key, Key>,
    #[serde(default = "default_skip")]
    pub skip_keybind: Vec<Key>,
    #[serde(default = "default_keybind")]
    pub keybind: Vec<Key>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        ron::from_str(&contents).unwrap()
    }
}

pub fn default_keybind() -> Vec<Key> {
    pub use Key::*;

    let mut keybind = Vec::new();

    #[rustfmt::skip]
    const KEYS: &[Key] = &[
        VK_LCONTROL, VK_LSHIFT, VK_LMENU, VK_RCONTROL, VK_RSHIFT, VK_RMENU,
        VK_SPACE, VK_OEM_102, VK_OEM_8, VK_OEM_COMMA, VK_OEM_1, VK_RETURN,
        VK_BACK, VK_TAB, VK_CAPITAL, VK_ESCAPE, VK_PRIOR, VK_NEXT, VK_END,
    ];

    for key in KEYS.iter() {
        // SAFETY: We know that the key is a valid key because it's been parsed
        // from the range of valid keys.
        keybind.push(unsafe { std::mem::transmute::<i8, Key>(*key as i8) });
    }

    const RANGES: [RangeInclusive<i8>; 5] = [
        VK_NUMPAD0 as i8..=VK_NUMPAD9 as i8,
        VK_F1 as i8..=VK_F12 as i8,
        VK_0 as i8..=VK_9 as i8,
        VK_LEFT as i8..=VK_DOWN as i8,
        VK_A as i8..=VK_Z as i8,
    ];

    for range in RANGES.into_iter() {
        for key in range {
            // SAFETY: We know that the key is a valid key because it's been parsed
            // from the range of valid keys.
            keybind.push(unsafe { std::mem::transmute::<i8, Key>(key) });
        }
    }

    keybind
}

pub fn default_remap() -> HashMap<Key, Key> {
    pub use Key::{VK_LMENU, VK_SPACE};

    let default_remap = [(VK_LMENU, VK_SPACE)];
    HashMap::from(default_remap)
}

pub fn default_skip() -> Vec<Key> {
    pub use Key::*;

    [VK_D, VK_Q, VK_S, VK_SPACE, VK_Z]
        .into_iter()
        .map(|key| unsafe { std::mem::transmute::<i8, Key>(key as i8) })
        .collect::<Vec<_>>()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_name: String::from("warcraft"),
            layout: LayoutOptions::Init,
            mode: Mode::Mimic(Duration::Milliseconds(10)),
            remap_keybind: default_remap(),
            skip_keybind: default_skip(),
            keybind: default_keybind(),
        }
    }
}
