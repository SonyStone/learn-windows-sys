use crate::{com_initialized, droppable::Droppable, graphics::direct_2d_factory::Direct2DFactory};

use windows::{
    core::Result,
    Win32::Graphics::Direct2D::{
        D2D1CreateFactory, ID2D1Factory1, D2D1_DEBUG_LEVEL_INFORMATION, D2D1_FACTORY_OPTIONS,
        D2D1_FACTORY_TYPE_SINGLE_THREADED,
    },
};

pub struct Test {
    pub factory: ID2D1Factory1,
    pub drop_log: Droppable,
}

impl Test {
    pub fn new() -> Result<Test> {
        let factory = create_factory()?;
        let drop_log = Droppable::new("Test");
        Ok(Test { factory, drop_log })
    }
}

impl Drop for Test {
    fn drop(&mut self) {}
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, Some(&options)) }
}
