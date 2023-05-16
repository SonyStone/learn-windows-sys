use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::Gdi::{
            BitBlt, CreateCompatibleDC, CreateDIBSection, CreateFontIndirectW, DeleteDC,
            DeleteObject, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
            DT_LEFT, DT_WORD_ELLIPSIS, HDC, HGDIOBJ, LOGFONTW, SRCCOPY,
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
pub unsafe fn paint_custom_caption(window: HWND, hdc: HDC) {
    let mut rc_client = RECT::default();
    GetClientRect(window, &mut rc_client);

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
            let mut dib = BITMAPINFO::default();
            dib.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
            dib.bmiHeader.biWidth = cx;
            dib.bmiHeader.biHeight = -cy;
            dib.bmiHeader.biPlanes = 1;
            dib.bmiHeader.biBitCount = 32;
            dib.bmiHeader.biCompression = BI_RGB.0 as u32;

            let hbm = CreateDIBSection(hdc, &dib, DIB_RGB_COLORS, std::ptr::null_mut(), None, 0);
            if let Ok(hbm) = hbm {
                let hbm_old = SelectObject(hdc_paint, hbm);

                // Setup the theme drawing options.
                let mut dtt_opts = DTTOPTS::default();
                dtt_opts.dwFlags = DTT_COMPOSITED | DTT_GLOWSIZE;
                dtt_opts.iGlowSize = 15;

                // Select a font.
                let mut lg_font = LOGFONTW::default();
                let mut h_font_old: Option<HGDIOBJ> = None;
                if GetThemeSysFont(h_theme, TMT_CAPTIONFONT, &mut lg_font).is_ok() {
                    let h_font = CreateFontIndirectW(&mut lg_font);
                    h_font_old = Some(SelectObject(hdc_paint, HGDIOBJ(h_font.0)));
                }

                let sz_title = w!("Custom Title");
                let sz_title = sz_title.as_wide();

                // Draw the title.
                let mut rc_paint = rc_client;
                rc_paint.top += 8;
                rc_paint.right -= 125;
                rc_paint.left += 8;
                rc_paint.bottom = 50;

                DrawThemeTextEx(
                    h_theme,
                    hdc_paint,
                    0,
                    0,
                    sz_title,
                    -1,
                    DT_LEFT | DT_WORD_ELLIPSIS,
                    &mut rc_paint,
                    Some(&dtt_opts),
                )
                .unwrap();

                // Blit text to the frame.
                BitBlt(hdc, 0, 0, cx, cy, hdc_paint, 0, 0, SRCCOPY);

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
}

pub fn rect_width(rc_client: RECT) -> i32 {
    rc_client.right - rc_client.left
}

pub fn rect_height(rc_client: RECT) -> i32 {
    rc_client.bottom - rc_client.top
}
