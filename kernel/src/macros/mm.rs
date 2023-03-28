macro_rules! new_kernel_vm {
    () => {{
        $crate::debug!("start kernel virtual memory mapping");
        let mut memory_region = MemoryRegion::new();

        $crate::debug!("mapping trampoline for kernel...");
        memory_region.map_trampoline();

        $crate::debug!("mapping .text section for kernel...");
        memory_region.add_one_segment(
            $crate::mm::memory_region::VirtSegment::new(
                stext!().into(),
                etext!().into(),
                $crate::mm::memory_region::MappingType::Identical,
                $crate::mm::memory_region::SegmentAccess::R
                    | $crate::mm::memory_region::SegmentAccess::X,
            ),
            None,
        );
        $crate::debug!("mapping .rodata section for kernel...");
        memory_region.add_one_segment(
            $crate::mm::memory_region::VirtSegment::new(
                srodata!().into(),
                erodata!().into(),
                $crate::mm::memory_region::MappingType::Identical,
                $crate::mm::memory_region::SegmentAccess::R,
            ),
            None,
        );
        $crate::debug!("mapping .data section for kernel...");
        memory_region.add_one_segment(
            $crate::mm::memory_region::VirtSegment::new(
                sdata!().into(),
                edata!().into(),
                $crate::mm::memory_region::MappingType::Identical,
                $crate::mm::memory_region::SegmentAccess::R
                    | $crate::mm::memory_region::SegmentAccess::W,
            ),
            None,
        );
        $crate::debug!("mapping .bss section for kernel...");
        memory_region.add_one_segment(
            $crate::mm::memory_region::VirtSegment::new(
                sbss!().into(),
                ebss!().into(),
                $crate::mm::memory_region::MappingType::Identical,
                $crate::mm::memory_region::SegmentAccess::R
                    | $crate::mm::memory_region::SegmentAccess::W,
            ),
            None,
        );
        $crate::debug!("mapping rest availble memory for kernel...");
        memory_region.add_one_segment(
            $crate::mm::memory_region::VirtSegment::new(
                ekernel!().into(),
                $crate::config::MEMORY_END.into(),
                $crate::mm::memory_region::MappingType::Identical,
                $crate::mm::memory_region::SegmentAccess::R
                    | $crate::mm::memory_region::SegmentAccess::W,
            ),
            None,
        );

        memory_region
    }};
}

/// kernel 根页表的 token
macro_rules! kernel_token {
    () => {{
        $crate::mm::kernel_vmm::KERNEL_VMM.lock().token()
    }};
}
