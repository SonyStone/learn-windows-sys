use std::os::raw::c_void;

use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::{
            Dwm::{DwmGetWindowAttribute, DWMWA_CAPTION_BUTTON_BOUNDS},
            Gdi::{
                BitBlt, CreateCompatibleDC, CreateDIBSection, CreateFontIndirectW, DeleteDC,
                DeleteObject, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
                DT_LEFT, DT_WORD_ELLIPSIS, HDC, HGDIOBJ, LOGFONTW, SRCCOPY,
            },
        },
        UI::{
            Controls::{
                CloseThemeData, DrawThemeTextEx, GetThemeSysFont, OpenThemeData, DTTOPTS,
                DTT_COMPOSITED, DTT_GLOWSIZE, HTHEME, TMT_CAPTIONFONT,
            },
            WindowsAndMessaging::*,
        },
    },
};

// Paint the title on the custom frame.
pub unsafe fn paint_custom_caption(window: HWND, hdc: HDC) -> Result<()> {
    let mut rc_client = RECT::default();

    let mut rect_caption_button_bounds = RECT::default();

    if DwmGetWindowAttribute(
        window,
        DWMWA_CAPTION_BUTTON_BOUNDS,
        &mut rect_caption_button_bounds as *mut _ as *mut c_void,
        std::mem::size_of::<RECT>() as u32,
    )
    .is_ok()
    {
        GetClientRect(window, &mut rc_client).unwrap();

        let h_theme: HTHEME = OpenThemeData(None, w!("CompositedWindow::Window"));
        if !h_theme.is_invalid() {
            let hdc_paint = CreateCompatibleDC(hdc);
            if !hdc_paint.is_invalid() {
                let cx = rect_width(rc_client);
                let cy = rect_height(rc_client);

                // Define the BITMAPINFO structure used to draw text.
                // Note that biHeight is negative. This is done because
                // DrawThemeTextEx() needs the bitmap to be in top-to-bottom
                // order.
                let dib = BITMAPINFO {
                    bmiHeader: BITMAPINFOHEADER {
                        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                        biWidth: cx,
                        biHeight: -cy,
                        biPlanes: 1,
                        biBitCount: 32,
                        biCompression: BI_RGB.0 as u32,
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let hbm =
                    CreateDIBSection(hdc, &dib, DIB_RGB_COLORS, std::ptr::null_mut(), None, 0);

                if let Ok(hbm) = hbm {
                    let hbm_old = SelectObject(hdc_paint, hbm);

                    // Setup the theme drawing options.
                    let dtt_opts = DTTOPTS {
                        dwFlags: DTT_COMPOSITED | DTT_GLOWSIZE,
                        iGlowSize: 15,
                        ..Default::default()
                    };

                    // Select a font.
                    let mut lg_font = LOGFONTW::default();
                    let mut h_font_old: Option<HGDIOBJ> = None;
                    if GetThemeSysFont(h_theme, TMT_CAPTIONFONT, &mut lg_font).is_ok() {
                        let h_font = CreateFontIndirectW(&mut lg_font);
                        h_font_old = Some(SelectObject(hdc_paint, HGDIOBJ(h_font.0)));
                    }

                    // Draw the title.
                    let mut rc_paint = RECT {
                        top: rc_client.top + 8,
                        right: rc_client.right - 125,
                        left: rc_client.left + 8,
                        bottom: 50,
                    };

                    println!("h_font_old {:?}", h_font_old);

                    // let sz_title: PCWSTR = w!("Custom Title");
                    // let sz_title = sz_title.as_wide();

                    let sz_title = &[67, 117, 115, 116, 111, 109, 32, 84, 105, 116, 108, 101, 0];

                    println!("h_theme {:?}", h_theme);
                    println!("hdc_paint {:?}", hdc_paint);
                    println!("sz_title {:?}, {:?}", sz_title, sz_title.len());
                    println!("rc_paint {:?}", rc_paint);
                    println!("dtt_opts {:?}", dtt_opts);

                    // ! Error goes here !
                    DrawThemeTextEx(
                        h_theme,
                        hdc_paint,
                        0,
                        0,
                        sz_title,
                        DT_LEFT | DT_WORD_ELLIPSIS,
                        &mut rc_paint,
                        Some(&dtt_opts),
                    )
                    .unwrap();

                    // Blit text to the frame.
                    BitBlt(hdc, 0, 0, cx, cy, hdc_paint, 0, 0, SRCCOPY).unwrap();

                    SelectObject(hdc_paint, hbm_old);
                    if let Some(h_font_old) = h_font_old {
                        SelectObject(hdc_paint, h_font_old);
                    }
                    DeleteObject(hbm);
                }

                DeleteDC(hdc_paint);
            }
            CloseThemeData(h_theme).unwrap();
        }
    };

    Ok(())
}

pub fn rect_width(rc_client: RECT) -> i32 {
    rc_client.right - rc_client.left
}

pub fn rect_height(rc_client: RECT) -> i32 {
    rc_client.bottom - rc_client.top
}
