use windows::{
    core::{IntoParam, Result},
    Foundation::Numerics::Matrix3x2,
    Win32::Graphics::{
        Direct2D::{
            Common::{D2D1_COLOR_F, D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_F},
            ID2D1Bitmap1, ID2D1Brush, ID2D1DeviceContext, ID2D1Geometry, ID2D1Image,
            ID2D1SolidColorBrush, ID2D1StrokeStyle, D2D1_BITMAP_PROPERTIES1, D2D1_BRUSH_PROPERTIES,
            D2D1_ELLIPSE, D2D1_UNIT_MODE,
        },
        Direct3D::{D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_11_0},
        Direct3D12::{
            D3D12CreateDevice, D3D12GetDebugInterface, ID3D12CommandAllocator, ID3D12CommandQueue,
            ID3D12Debug, ID3D12DescriptorHeap, ID3D12Device, ID3D12GraphicsCommandList,
        },
        Dxgi::{
            CreateDXGIFactory2, IDXGIAdapter1, IDXGIFactory4, IDXGISurface, DXGI_ADAPTER_DESC1,
            DXGI_ADAPTER_FLAG_SOFTWARE,
        },
    },
};

pub struct Game {
    command_queue: Option<ID3D12CommandQueue>,
    rtv_descriptor_heap: Option<ID3D12DescriptorHeap>,
    dsv_descriptor_heap: Option<ID3D12DescriptorHeap>,
    command_allocators: Option<ID3D12CommandAllocator>,
    command_list: Option<ID3D12GraphicsCommandList>,
}

impl Drop for Game {
    fn drop(&mut self) {
        // unsafe { WaitForGpu() };
    }
}

impl Game {
    pub fn initialize() -> Result<Self> {
        let dxgi_factory_flags = 0;

        if cfg!(debug_assertions) {
            let debug_controller: ID3D12Debug = create_debug_controller()?;
            unsafe { debug_controller.EnableDebugLayer() };
        }

        let dxgi_factory: IDXGIFactory4 = unsafe { CreateDXGIFactory2(dxgi_factory_flags) }?;

        let feature_level = D3D_FEATURE_LEVEL_11_0;

        let adapter = {
            let mut adapter_index = 0;
            loop {
                let adapter = unsafe { dxgi_factory.EnumAdapters1(adapter_index) }?;
                adapter_index += 1;

                let mut desc = DXGI_ADAPTER_DESC1::default();
                unsafe { adapter.GetDesc1(&mut desc) }?;

                if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 != 0 {
                    continue;
                }

                let mut device: Option<ID3D12Device> = None;
                if unsafe { D3D12CreateDevice(&adapter, feature_level, &mut device) }.is_ok() {
                    break adapter;
                }
            }
        };

        println!("adapter!");

        Ok(Game {
            command_queue: None,
            rtv_descriptor_heap: None,
            dsv_descriptor_heap: None,
            command_allocators: None,
            command_list: None,
        })
    }

    fn tick(self) {}

    fn update() {}

    fn render() {}

    fn clear() {}

    fn present() {}

    fn on_activated() {
        // TODO: Game is becoming active window.
        todo!();
    }

    fn on_deactivated() {
        // TODO: Game is becoming background window.
        todo!();
    }

    fn on_suspending() {
        // TODO: Game is being power-suspended (or minimized).
    }

    fn on_resuming() {
        // TODO: Game is being power-resumed (or returning from minimize).
    }

    fn on_window_size_changed() {}

    fn get_default_size() {}

    pub fn create_device() {
        let dxgi_factory_flags = 0;
    }

    fn create_resources() {}

    fn wait_for_gpu() {}
}

fn create_debug_controller() -> Result<ID3D12Debug> {
    let mut debug_controller: Option<ID3D12Debug> = None;
    unsafe { D3D12GetDebugInterface(&mut debug_controller) }?;

    Ok(debug_controller.unwrap())
}
