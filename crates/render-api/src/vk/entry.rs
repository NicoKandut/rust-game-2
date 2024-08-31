pub(crate) fn load_entry() -> ash::Entry {
    unsafe { ash::Entry::load().expect("Failed to load entry!") }
}