fn read_shader_from_bytes(bytes: &[u8]) -> Vec<u32> {
    let mut cursor = std::io::Cursor::new(&bytes[..]);
    ash::util::read_spv(&mut cursor).expect("Unable to read spir-v shader")
}

pub fn get_rt_shader() -> Vec<u32> {
    let bytes = include_bytes!("spv/rt.comp.spv");
    read_shader_from_bytes(bytes)
}

pub fn get_farm_vertex_shader() -> Vec<u32> {
    let bytes = include_bytes!("spv/farm.vert.spv");
    read_shader_from_bytes(bytes)
}

pub fn get_farm_fragment_shader() -> Vec<u32> {
    let bytes = include_bytes!("spv/farm.frag.spv");
    read_shader_from_bytes(bytes)
}