use glam::{Affine2, IVec2, Vec2};
use windows::{
    core::Result,
    Foundation::Numerics::Matrix3x2,
    Win32::{
        Foundation::{DXGI_STATUS_OCCLUDED, HWND},
        Graphics::{
            Direct2D::{
                Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F},
                ID2D1SolidColorBrush, ID2D1StrokeStyle, D2D1_BRUSH_PROPERTIES, D2D1_ELLIPSE,
                D2D1_UNIT_MODE_DIPS,
            },
            Dxgi::{Common::DXGI_FORMAT_UNKNOWN, CreateDXGIFactory1, IDXGIFactory2},
        },
        System::{
            Com::{CoCreateInstance, CLSCTX_ALL},
            Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
        },
        UI::{
            Animation::{
                IUIAnimationManager, IUIAnimationTransition, IUIAnimationTransitionLibrary,
                IUIAnimationVariable, UIAnimationManager, UIAnimationTransitionLibrary,
            },
            WindowsAndMessaging::WM_USER,
        },
    },
};

use crate::{
    com_initialized,
    graphics::{
        brush::{Brush, SolidColorBrush},
        device_context::DeviceContext,
        direct_2d_factory::Direct2DFactory,
        direct_3d_device::Direct3D11Device,
        dxgi_swap_chain::DXGISwapChain,
    },
};
#[derive(Debug)]
pub struct Direct2d {
    pub factory: Direct2DFactory,
    // ! What the purpose?
    pub dx_factory: IDXGIFactory2,
    pub style: ID2D1StrokeStyle,
    pub transition: IUIAnimationTransition,
    pub manager: IUIAnimationManager,
    pub variable: IUIAnimationVariable,
    pub frequency: i64,
    pub dpi: f32,
    pub context: Option<DeviceContext>,
    pub brush: Option<ID2D1SolidColorBrush>,
    pub swapchain: Option<DXGISwapChain>,
    pub camera: Affine2,
    pub handle: Option<HWND>,
    pub point_center: Option<Vec2>,
    pub touches: Vec<Vec2>,
}

impl Direct2d {
    pub fn new() -> Result<Self> {
        com_initialized::com_initialized();

        let factory = Direct2DFactory::new()?;
        let dx_factory: IDXGIFactory2 = unsafe { CreateDXGIFactory1()? };
        let style = factory.create_stroke_style(None)?;
        let manager: IUIAnimationManager =
            unsafe { CoCreateInstance(&UIAnimationManager, None, CLSCTX_ALL)? };

        let transition = create_transition()?;

        let dpi = factory.get_desktop_dpi();

        let frequency = {
            let mut frequency = 0;
            unsafe { QueryPerformanceFrequency(&mut frequency) };
            frequency
        };

        let variable = unsafe {
            let variable = manager.CreateAnimationVariable(0.0)?;

            manager.ScheduleTransition(&variable, &transition, get_time(frequency)?)?;

            variable
        };

        Ok(Direct2d {
            factory,
            dx_factory,
            style,
            manager,
            transition,
            frequency,
            variable,
            dpi,
            context: None,
            brush: None,
            swapchain: None,
            camera: Affine2::IDENTITY,
            handle: None,
            point_center: None,
            touches: Vec::new(),
        })
    }

    pub fn set_handle(&mut self, handle: HWND) {
        self.handle = Some(handle)
    }

    pub fn render(&mut self) -> Result<()> {
        if self.handle.is_none() {
            return Ok(());
        }

        if self.context.is_none() {
            let device = Direct3D11Device::new()?;

            let d2device = self.factory.create_device(&device)?;
            let context = d2device.create_device_context()?;
            context.set_unit_mode(D2D1_UNIT_MODE_DIPS);
            context.set_dpi_scale(self.dpi);

            let swapchain = device.create_swapchain(self.handle.unwrap())?;

            swapchain.create_swapchain_bitmap(&context)?;

            self.brush = create_brush(&context).ok();
            self.context = Some(context);
            self.swapchain = Some(swapchain);
        }

        let context = self.context.as_ref().unwrap();
        let brush = self.brush.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();

        // Draw
        {
            context.begin_draw();

            context.clear(Some(&D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }));

            context.set_transform(&self.camera);

            {
                let radius = 50.0;

                let ellipse = D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: 0.0, y: 0.0 },
                    radiusX: radius,
                    radiusY: radius,
                };
                context.draw_ellipse(&ellipse, brush, 5.0, None);
            }

            context.draw_rectangle(
                &D2D_RECT_F {
                    left: 100.0,
                    top: 100.0,
                    right: -100.0,
                    bottom: -100.0,
                },
                brush,
                3.0,
                None,
            );

            if let Some(point_center) = self.point_center {
                context.set_transform(&Affine2::default());

                for touch in self.touches.iter() {
                    context.draw_line((*touch, point_center), brush, 2.5, None);
                    let point = D2D1_ELLIPSE {
                        point: D2D_POINT_2F {
                            x: touch.x,
                            y: touch.y,
                        },
                        radiusX: 25.0,
                        radiusY: 25.0,
                    };
                    context.draw_ellipse(&point, brush, 2.5, None);
                }

                let point = D2D1_ELLIPSE {
                    point: D2D_POINT_2F {
                        x: point_center.x,
                        y: point_center.y,
                    },
                    radiusX: 15.0,
                    radiusY: 15.0,
                };

                context.draw_ellipse(&point, brush, 2.5, None);
            }

            context.end_draw(None, None)?;
        }

        if let Err(error) = swapchain.present(1, 0) {
            if error.code() == DXGI_STATUS_OCCLUDED {
                let occlusion = unsafe {
                    self.dx_factory
                        .RegisterOcclusionStatusWindow(self.handle.unwrap(), WM_USER)?
                };
            } else {
                self.release_device();
            }
        }

        Ok(())
    }

    pub fn resize_swapchain_bitmap(&mut self) -> Result<()> {
        if let Some(target) = &self.context {
            let swapchain = self.swapchain.as_ref().unwrap();
            target.set_target(None);

            if swapchain
                .resize_buffers(0, 0, 0, DXGI_FORMAT_UNKNOWN, 0)
                .is_ok()
            {
                swapchain.create_swapchain_bitmap(target)?;
            } else {
                self.release_device();
            }
        }

        Ok(())
    }

    pub fn release_device(&mut self) {
        self.context = None;
        self.swapchain = None;
        self.brush = None;
    }
}

impl Drop for Direct2d {
    fn drop(&mut self) {
        println!("🚮 Direct2d dropped here")
    }
}

fn create_transition() -> Result<IUIAnimationTransition> {
    unsafe {
        let library: IUIAnimationTransitionLibrary =
            CoCreateInstance(&UIAnimationTransitionLibrary, None, CLSCTX_ALL)?;
        library.CreateAccelerateDecelerateTransition(5.0, 1.0, 0.2, 0.8)
    }
}

fn get_time(frequency: i64) -> Result<f64> {
    unsafe {
        let mut time = 0;
        QueryPerformanceCounter(&mut time);
        Ok(time as f64 / frequency as f64)
    }
}

fn create_brush(target: &DeviceContext) -> Result<ID2D1SolidColorBrush> {
    let color = D2D1_COLOR_F {
        r: 0.92,
        g: 0.38,
        b: 0.208,
        a: 1.0,
    };

    let properties = D2D1_BRUSH_PROPERTIES {
        opacity: 1.0,
        transform: Matrix3x2::identity(),
    };

    let brush = target.create_solid_color_brush(&color, Some(&properties))?;

    Ok(brush)
}
