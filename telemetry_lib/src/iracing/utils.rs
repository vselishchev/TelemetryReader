
static IRSDK_VER : i32 = 2; // Latest sdk version.
const IRSDK_MAX_BUFS : i32 = 4; // Maximum buffers number;

#[repr(C)]
pub struct IRacingVariableBuffer {
    tick_count : i32, // Used to detect changes in data
    buffer_offset : i32, // offset from header
    pad : i64 // 16 bytes align.
}

#[repr(C)]
pub struct IRacingHeader {
    version: i32, // This api header version, see IRSDK_VER
    status: i32, // 1 if connected
    tick_rate: i32, // Ticks per second (60 or 360 etc)

    // session information, updated periodicaly
    session_info_update: i32,	// Incremented when session info changes
	session_info_len: i32,	// Length in bytes of session info string
	session_info_offset: i32,	// Session info, encoded in YAML format

    // State data, output at tickRate
    variables_number: i32, // Length of array pointed to by var_header_offset
    variable_header_offset: i32, // Offset to irsdk_variable_header[variables_number] array, Describes the variables received in variable_buffer

    buffers_number: i32, // <= IRSD_MAX_BUFS
    buffer_length: i32, // Length in bites for one line.
    pad1: i64, // 16 bytes align.
    variable_buffer: [IRacingVariableBuffer; IRSDK_MAX_BUFS as usize] // Buffers of data being written to.
}