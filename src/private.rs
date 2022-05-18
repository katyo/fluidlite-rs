pub trait HasHandle {
    type Handle;

    fn get_handle(&self) -> std::ptr::NonNull<Self::Handle>;
}
