use crate::lv_core::obj::NativeObject;
use crate::widgets::Slider;
use crate::{AnimationState, LvResult};

impl Slider<'_> {
    /// Set a new value on the slider
    pub fn set_value(&self, value: i32, anim: AnimationState) -> LvResult<()> {
        unsafe { lvgl_sys::lv_bar_set_value(self.core.raw()?.as_ptr(), value, anim.into()) }
        Ok(())
    }

    /// Gets the current value of the slider
    pub fn get_value(&self) -> LvResult<i32> {
        unsafe { Ok(lvgl_sys::lv_bar_get_value(self.core.raw()?.as_ptr())) }
    }
}
