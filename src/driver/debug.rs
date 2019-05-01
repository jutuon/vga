
use crate::{
    raw::{
        UnknownValue,
        general::*,
        attribute_controller::*,
        crt_controller::*,
        graphics_controller::*,
        sequencer::*,
    },
    driver::register::*,
    io::*,
};

#[derive(Debug)]
pub struct GeneralDebug {
    f0: InputStatusRegister0Flags,
    f1: InputStatusRegister1Flags,
    f2: MiscellaneousOutputRegisterFlags,
    vertical_size: Result<VerticalSize, UnknownValue>,
    clock_select: Result<ClockSelect, UnknownValue>,
    video_subsystem_enable: bool,
}

impl GeneralDebug {
    pub fn read_registers<T: PortIo>(r: &mut RegisterHandler<T>) -> Self {
        let r0 = r.read_miscellaneous_output();

        Self {
            f0: r.read_input_status_0().flags(),
            f1: r.read_input_status_1().flags(),
            f2: r0.flags(),
            vertical_size: r0.vertical_size(),
            clock_select: r0.clock_select(),
            video_subsystem_enable: r.read_video_subsystem_enable().video_subsystem_enable(),
        }
    }
}

#[derive(Debug)]
pub struct AttributeControllerDebug {
    internal_palette_address_source: bool,
    flags0: AttributeModeControlRegisterFlags,
    flags1: ColorPlaneEnableRegisterFlags,
    flags2: ColorSelectRegisterFlags,
    horizontal_pel_panning: u8,
    palette_address: [u8; 16],
    overscan_palette_address: u8,
}

impl AttributeControllerDebug {
    pub fn read_registers<T: PortIo>(r: &mut RegisterHandler<T>) -> Self {
        Self {
            internal_palette_address_source: r.read_attribute_address().internal_palette_address_source(),
            flags0: r.attribute_mode_control().flags(),
            flags1: r.color_plane_enable().flags(),
            flags2: r.color_select().flags(),
            horizontal_pel_panning: r.horizontal_pel_panning().horizontal_pel_panning(),
            palette_address: {
                let mut i = InternalPaletteIndex::VALUES.iter()
                    .map(|i| r.read_internal_palette(*i).palette_address());
                [i.next().unwrap(); 16]
            },
            overscan_palette_address: r.overscan_color().overscan_palette_address(),
        }
    }
}

#[derive(Debug)]
pub struct CrtControllerDebug {
    f0: CrtModeControlRegisterFlags,
    cursor_skew_control: SkewControl,
    row_scan_cursor_ends: u8,
    cursor_location: u16,
    cursor_off: bool,
    row_scan_cursor_begins: u8,
    end_horizontal_blanking_skew: SkewControl,
    end_horizontal_blanking: u8,
    horizontal_retrace_delay: SkewControl,
    end_horizontal_retrace: u8,
    end_vertical_blanking: u8,
    horizontal_display_enable_end: u8,
    horizontal_total: u8,
    line_compare: u16,
    line_conversion_200_to_400: bool,
    maximum_scan_line: u8,
    logical_line_width_of_the_screen: u8,
    f1: PresetRowScanRegisterFlags,
    starting_row_scan_count: u8,
    start_address: u16,
    start_horizontal_blanking: u8,
    start_horizontal_retrace_pulse: u8,
    start_vertical_blanking: u16,
    f2: UnderlineLocationRegisterFlags,
    vertical_display_enable_end: u16,
    f3: VerticalRetraceEndRegisterFlags,
    vertical_retrace_end: u8,
    vertical_retrace_start: u16,
    vertical_total: u16,
}

impl CrtControllerDebug {
    pub fn read_registers<T: PortIo>(r: &mut RegisterHandler<T>) -> Self {
        let (cursor_skew_control, row_scan_cursor_ends) = {
            let r = r.cursor_end();
            (r.cursor_skew_control(), r.row_scan_cursor_ends())
        };
        let (cursor_off, row_scan_cursor_begins) = {
            let r = r.cursor_start();
            (r.cursor_off(), r.row_scan_cursor_begins())
        };

        let (horizontal_retrace_delay, end_horizontal_retrace) = {
            let r = r.end_horizontal_retrace();
            (r.horizontal_retrace_delay(), r.end_horizontal_retrace())
        };

        let (line_conversion_200_to_400, maximum_scan_line) = {
            let r = r.maximum_scan_line();
            (r.line_conversion_200_to_400(), r.maximum_scan_line())
        };

        let (f1, starting_row_scan_count) = {
            let r = r.preset_row_scan();
            (r.flags(), r.starting_row_scan_count())
        };

        let (f3, vertical_retrace_end) = {
            let r = r.vertical_retrace_end();
            (r.flags(), r.vertical_retrace_end())
        };

        Self {
            f0: r.crt_mode_control().flags(),
            cursor_skew_control,
            row_scan_cursor_ends,
            cursor_location: r.crt().cursor_location(),
            cursor_off,
            row_scan_cursor_begins,
            end_horizontal_blanking_skew: r.end_horizontal_blanking().skew_control(),
            end_horizontal_blanking: r.crt().end_horizontal_blanking(),
            horizontal_retrace_delay,
            end_horizontal_retrace,
            end_vertical_blanking: r.end_vertical_blanking().end_vertical_blanking(),
            horizontal_display_enable_end: r.horizontal_display_enable_end().horizontal_display_enable_end(),
            horizontal_total: r.horizontal_total().horizontal_total(),
            line_compare: r.crt().line_compare(),
            line_conversion_200_to_400,
            maximum_scan_line,
            logical_line_width_of_the_screen: r.offset().logical_line_width_of_the_screen(),
            f1,
            starting_row_scan_count,
            start_address: r.crt().start_address(),
            start_horizontal_blanking: r.start_horizontal_blanking().start_horizontal_blanking(),
            start_horizontal_retrace_pulse: r.start_horizontal_retrace_pulse().start_horizontal_retrace_pulse(),
            start_vertical_blanking: r.crt().start_vertical_blanking(),
            f2: r.underline_location().flags(),
            vertical_display_enable_end: r.crt().vertical_display_enable_end(),
            f3,
            vertical_retrace_end,
            vertical_retrace_start: r.crt().vertical_retrace_start(),
            vertical_total: r.crt().vertical_total(),
        }
    }
}

#[derive(Debug)]
pub struct GraphicsControllerDebug {
    bit_mask: u8,
    color_compare: MapFlags,
    color_do_not_care: MapFlags,
    function_select: DataFunction,
    rotate_count: u8,
    enable_set_slash_reset: MapFlags,
    f0: GraphicsModeRegisterFlags,
    write_mode: WriteMode,
    f1: MiscellaneousRegisterFlags,
    addressing_assignment: AddressingAssignment,
    read_map_select: MapSelect,
    set_slash_reset: MapFlags,
}


impl GraphicsControllerDebug {
    pub fn read_registers<T: PortIo>(r: &mut RegisterHandler<T>) -> Self {
        let (function_select, rotate_count) = {
            let r = r.data_rotate();
            (r.function_select(), r.rotate_count())
        };

        let (f0, write_mode) = {
            let r = r.graphics_mode();
            (r.flags(), r.write_mode())
        };

        let (f1, addressing_assignment) = {
            let r = r.miscellaneous();
            (r.flags(), r.addressing_assignment())
        };

        Self {
            bit_mask: r.bit_mask().bit_mask(),
            color_compare: r.color_compare().flags(),
            color_do_not_care: r.color_do_not_care().flags(),
            function_select,
            rotate_count,
            enable_set_slash_reset: r.enable_set_slash_reset().flags(),
            f0,
            write_mode,
            f1,
            addressing_assignment,
            read_map_select: r.map_select().map_select(),
            set_slash_reset: r.set_slash_reset().flags(),
        }
    }
}


#[derive(Debug)]
pub struct SequencerDebug {
    character_map_a_select: CharacterMapASelect,
    character_map_b_select: CharacterMapBSelect,
    f0: ClockingModeRegisterFlags,
    f1: MapMaskRegisterFlags,
    f2: MemoryModeRegisterFlags,
    f3: ResetRegisterFlags,
}

impl SequencerDebug {
    pub fn read_registers<T: PortIo>(r: &mut RegisterHandler<T>) -> Self {
        let (character_map_a_select, character_map_b_select) = {
            let r = r.character_map_select();
            (r.character_map_a_select(), r.character_map_b_select())
        };

        Self {
            character_map_a_select,
            character_map_b_select,
            f0: r.clocking_mode().flags(),
            f1: r.map_mask().flags(),
            f2: r.memory_mode().flags(),
            f3: r.reset().flags(),
        }
    }
}
