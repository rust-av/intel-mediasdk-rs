#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/mfxvideo.rs"));

impl mfxVersion {
    pub fn new(Major: mfxU16, Minor: mfxU16) -> Self {
        mfxVersion {
            __bindgen_anon_1: mfxVersion__bindgen_ty_1 { Major, Minor },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    #[test]
    fn init_and_version() {
        let mut sess = unsafe { mem::uninitialized() };
        let im: mfxIMPL = MFX_IMPL_AUTO_ANY as mfxIMPL;
        let mut ver = mfxVersion::new(MFX_VERSION_MAJOR as u16, MFX_VERSION_MINOR as u16);

        let res = unsafe {
            MFXInit(
                im,
                &mut ver as *mut mfxVersion,
                &mut sess as *mut mfxSession,
            )
        };

        println!("Res {:?}", res);
    }
}
