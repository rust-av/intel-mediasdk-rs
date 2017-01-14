// TODO do w/out the unions?
#![feature(untagged_unions)]


pub mod mfxvideo;

#[cfg(test)]
mod tests {
    use super::mfxvideo::*;
    use std::mem;
    #[test]
    fn init_and_version() {
        let mut sess = unsafe { mem::uninitialized() };
        let im : mfxIMPL = MFX_IMPL_AUTO_ANY as mfxIMPL;
        // FIXME make the union a little saner
        let mut ver = mfxVersion { __bindgen_anon_1 : _bindgen_ty_6__bindgen_ty_1 { Major : MFX_VERSION_MAJOR as u16, Minor : MFX_VERSION_MINOR as u16 } };

        let res = unsafe { MFXInit(im, &mut ver as *mut mfxVersion, &mut sess as *mut mfxSession) };


        println!("Res {:?}", res);
    }
}
