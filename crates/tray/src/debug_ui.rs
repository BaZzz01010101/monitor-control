#![cfg_attr(test, allow(dead_code))]

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

#[cfg(windows)]
use anyhow::{anyhow, Context};
#[cfg(windows)]
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        HBITMAP, HGDIOBJ, SRCCOPY,
    },
    UI::WindowsAndMessaging::{
        BringWindowToTop, GetWindowRect, SetForegroundWindow, SetWindowPos, HWND_TOPMOST,
        SWP_NOMOVE, SWP_NOSIZE,
    },
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LaunchMode {
    #[default]
    Normal,
    DebugUi,
}

pub fn launch_mode_from_args<I, S>(args: I) -> LaunchMode
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    if args
        .into_iter()
        .any(|arg| matches!(arg.as_ref(), "debug-ui" | "--debug-ui"))
    {
        LaunchMode::DebugUi
    } else {
        LaunchMode::Normal
    }
}

pub fn debug_ui_output_path(base_dir: &Path, timestamp_ms: u64) -> PathBuf {
    base_dir
        .join(".tmp")
        .join(format!("dell-controller-tray-ui-{timestamp_ms}.bmp"))
}

#[cfg(windows)]
pub fn capture_window_to_bmp(hwnd: HWND, path: &Path) -> anyhow::Result<()> {
    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect).context("failed to read window bounds")? };

    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    if width <= 0 || height <= 0 {
        return Err(anyhow!("window bounds are empty"));
    }

    unsafe {
        let _ = SetWindowPos(
            hwnd,
            Some(HWND_TOPMOST),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        );
        let _ = BringWindowToTop(hwnd);
        let _ = SetForegroundWindow(hwnd);
    }
    std::thread::sleep(std::time::Duration::from_millis(120));

    let screen_dc = unsafe { GetDC(None) };
    if screen_dc.is_invalid() {
        return Err(anyhow!("failed to acquire screen device context"));
    }

    let memory_dc = unsafe { CreateCompatibleDC(Some(screen_dc)) };
    if memory_dc.is_invalid() {
        unsafe {
            ReleaseDC(None, screen_dc);
        }
        return Err(anyhow!("failed to create compatible device context"));
    }

    let bitmap = unsafe { CreateCompatibleBitmap(screen_dc, width, height) };
    if bitmap.is_invalid() {
        unsafe {
            let _ = DeleteDC(memory_dc);
            ReleaseDC(None, screen_dc);
        }
        return Err(anyhow!("failed to create compatible bitmap"));
    }

    let old_bitmap = unsafe { SelectObject(memory_dc, HGDIOBJ(bitmap.0)) };
    let capture_result = unsafe {
        BitBlt(
            memory_dc,
            0,
            0,
            width,
            height,
            Some(screen_dc),
            rect.left,
            rect.top,
            SRCCOPY,
        )
    };
    capture_result.context("failed to copy the window from the desktop")?;

    let mut info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            biSizeImage: (width * height * 4) as u32,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut pixels = vec![0u8; (width as usize) * (height as usize) * 4];
    let copied_lines = unsafe {
        GetDIBits(
            memory_dc,
            HBITMAP(bitmap.0),
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut info,
            DIB_RGB_COLORS,
        )
    };

    unsafe {
        SelectObject(memory_dc, old_bitmap);
        let _ = DeleteObject(HGDIOBJ(bitmap.0));
        let _ = DeleteDC(memory_dc);
        ReleaseDC(None, screen_dc);
    }
    if copied_lines == 0 {
        return Err(anyhow!(
            "failed to read bitmap bits from the captured window"
        ));
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create debug-ui screenshot directory {}",
                parent.display()
            )
        })?;
    }

    write_bmp(path, width as u32, height as u32, &pixels)
}

#[cfg(windows)]
fn write_bmp(path: &Path, width: u32, height: u32, pixels: &[u8]) -> anyhow::Result<()> {
    let pixel_bytes = pixels.len() as u32;
    let file_size = 14u32 + 40u32 + pixel_bytes;
    let mut file = File::create(path)
        .with_context(|| format!("failed to create debug-ui screenshot {}", path.display()))?;

    file.write_all(b"BM")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(&0u16.to_le_bytes())?;
    file.write_all(&0u16.to_le_bytes())?;
    file.write_all(&(14u32 + 40u32).to_le_bytes())?;

    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(-(height as i32)).to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&32u16.to_le_bytes())?;
    file.write_all(&BI_RGB.0.to_le_bytes())?;
    file.write_all(&pixel_bytes.to_le_bytes())?;
    file.write_all(&0i32.to_le_bytes())?;
    file.write_all(&0i32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;

    file.write_all(pixels)?;
    Ok(())
}
