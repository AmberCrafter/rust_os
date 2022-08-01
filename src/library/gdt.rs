// GDT: Global description table

use lazy_static::lazy_static;
use x86_64::{structures::{tss::TaskStateSegment, gdt::{GlobalDescriptorTable, Descriptor}}, VirtAddr, registers::segmentation::SegmentSelector};

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}


// Task state segment (TSS) has 7 fields to record the current state
// Interrupt stack table is one of the TSS field which can select any index between 0 to 6;
// Note. TSS has three field in current Segment
// including: 
//      Privilege stack table: [u64; 3]
//      Interrupt stack table: [u64; 7]
//      I/O map base address:  u16
// 
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096*5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            
            let stack_start = VirtAddr::from_ptr(
                unsafe {
                    &STACK
                }
            );
            let stack_end = stack_start+STACK_SIZE;
            stack_end
        };
        tss
    };
}


lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {code_selector, tss_selector})
    };
}

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}