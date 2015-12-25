use service::*;

pub static API_VERSION: &'static str = "ProDBG View 1";

pub trait View {
    fn new(ui: &Ui, service: &Service) -> Self;
    fn update(&mut self, ui: &Ui, reader: &mut Reader, writer: &mut Writer);
}

#[repr(C)]
pub struct CViewCallbacks {
    pub name: *const c_uchar,
    pub create_instance: Option<fn(ui_api: *const c_void,
                                   service_func: extern "C" fn(service: *const c_uchar)
                                                               -> *mut c_void)
                                   -> *mut c_void>,
    pub destroy_instance: Option<fn(*mut c_void)>,
    pub update: Option<fn(ptr: *mut c_void,
                          ui: *mut c_void,
                          reader: *mut c_void,
                          writer: *mut c_void)>,
}

pub fn create_view_instance<T: View>(ui_api: *const c_void,
                                     service_func: extern "C" fn(service: *const c_uchar)
                                                                 -> *mut c_void)
                                     -> *mut c_void {
    let c_ui: &mut CPdUI = unsafe { &mut *(ui_api as *mut CPdUI) };
    let ui = Ui { api: c_ui };
    let service = Service { service_func: service_func };
    let instance = unsafe { transmute(Box::new(T::new(&ui, &service))) };
    instance
}

pub fn destroy_view_instance<T: View>(ptr: *mut c_void) {
    println!("rust: backend: destroy");
    let _: Box<T> = unsafe { transmute(ptr) };
    // implicitly dropped
}

pub fn update_view_instance<T: Backend>(ptr: *mut c_void,
                                        ui_api: *mut c_void,
                                        reader_api: *mut c_void,
                                        writer_api: *mut c_void) {
    let view: &mut T = unsafe { &mut *(ptr as *mut T) };
    let c_ui: &mut CPdUI = unsafe { &mut *(ui_api as *mut CPdUI) };
    let c_reader: &mut CPDReaderAPI = unsafe { &mut *(reader_api as *mut CPDReaderAPI) };
    let c_writer: &mut CPDWriterAPI = unsafe { &mut *(writer_api as *mut CPDWriterAPI) };
    let mut reader = Reader {
        api: c_reader,
        it: 0,
    };

    let mut writer = Writer { api: c_writer };
    let ui = Ui { api: c_ui };

    backend.update(&ui, &mut reader, &mut writer);
}

#[macro_export]
macro_rules! define_backend_plugin {
    ($x:ty) => {
        {
            static S: &'static [u8] = b"Test UI\0";
            let mut plugin = CBackendCallbacks {
                name: S.as_ptr(), 
                create_instance: Some(prodbg::view::create_view_instance::<$x>),
                destroy_instance: Some(prodbg::view::destroy_view_instance::<$x>),
                update: Some(prodbg::view::update_backend_instance::<$x>)
             };

            Box::new(plugin)
        }
    }
}
