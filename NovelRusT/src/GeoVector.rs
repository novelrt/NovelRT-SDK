use NovelRT_sys::GeoVectorWrapper;
use std::os::raw::c_void;
use NovelRT_sys::{createGeoVector, getX, setX, getY, setY};

pub struct GeoVector<T> {
    x: *const T,
    y: *const T
}

impl <T> GeoVector<T> where T: Copy{
    pub fn get_x(&self) -> T {
        unsafe {
            *(self.x)
        }
    }

    pub fn get_y(&self) -> T {
        unsafe {
            *(self.y)
        }
    }

    pub fn set_x(&mut self, x: *const T) {
        self.x = x
    }

    pub fn set_y(&mut self, y: *const T) {
        self.y = y
    }
}

pub fn create_geo_vector<T>(x: *mut T, y: *mut T) -> GeoVector<T> {
    let wrapper: *mut GeoVectorWrapper;
    unsafe {
        wrapper = createGeoVector(x as *mut c_void, y as *mut c_void);
        GeoVector {
            x: getX(wrapper) as *const T,
            y: getY(wrapper) as *const T
        }
    }
}