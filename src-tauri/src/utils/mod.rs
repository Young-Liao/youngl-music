pub mod song;
pub mod system;

#[cfg(target_os = "windows")]
pub mod ta_win_util {
    pub fn set_aumid(id: &str) -> Result<(), String> {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows::core::PCWSTR;
        use windows::Win32::UI::Shell::SetCurrentProcessExplicitAppUserModelID;

        let wide_id: Vec<u16> = OsStr::new(id).encode_wide().chain(std::iter::once(0)).collect();
        unsafe {
            SetCurrentProcessExplicitAppUserModelID(PCWSTR(wide_id.as_ptr()))
                .map_err(|e| e.to_string())
        }
    }
}