extern crate libc;
use libc::{c_int, c_float};
use std::os::raw::c_void;

extern "C" {
    pub fn createRunner(display_number: c_int) -> *mut NovelRunner;
    fn destroyRunner(runner: *mut NovelRunner);
    pub fn runOnUpdate(runner: *mut NovelRunner, subscriber: extern "C" fn (c_float) -> ());
    pub fn runNovel(runner: *mut NovelRunner);

    pub fn createGeoVector(x: *mut c_void, y: *mut c_void) -> *mut GeoVectorWrapper;
    fn destroyGeoVector(geo_vector: *mut GeoVectorWrapper);
    pub fn getX(geo_vector: *mut GeoVectorWrapper) -> *mut c_void;
    pub fn setX(geo_vector: *mut GeoVectorWrapper, x: *mut c_void);
    pub fn getY(geo_vector: *mut GeoVectorWrapper) -> *mut c_void;
    pub fn setY(geo_vector: *mut GeoVectorWrapper, y: *mut c_void);
}

#[repr(C)]
pub struct NovelRunner;

impl Drop for NovelRunner {
    fn drop(&mut self) {
        unsafe {
            destroyRunner(self)
        }
    }
}

#[repr(C)]
pub struct GeoVectorWrapper;

impl Drop for GeoVectorWrapper {
    fn drop(&mut self) {
        unsafe {
            destroyGeoVector(self)
        }
    }
}

