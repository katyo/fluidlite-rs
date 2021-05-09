pub trait HasHandle {
    type Handle;

    fn get_handle(&self) -> *mut Self::Handle;
}
