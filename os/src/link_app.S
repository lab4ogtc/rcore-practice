
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 6
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_5_end

    .section .data
    .global app_0_start
    .global app_0_end
    .align 3
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch4_mmap0"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
    .align 3
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch4_mmap1"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
    .align 3
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch4_mmap2"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
    .align 3
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch4_mmap3"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
    .align 3
app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch4_unmap"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
    .align 3
app_5_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch4_unmap2"
app_5_end:

    .section .data
    .global _app_names
_app_names:
    .quad app_0_name
    .quad app_1_name
    .quad app_2_name
    .quad app_3_name
    .quad app_4_name
    .quad app_5_name
    .quad app_name_end

    .section .data
    .global app_0_name
app_0_name:
    .string "ch4_mmap0"
    .global app_1_name
app_1_name:
    .string "ch4_mmap1"
    .global app_2_name
app_2_name:
    .string "ch4_mmap2"
    .global app_3_name
app_3_name:
    .string "ch4_mmap3"
    .global app_4_name
app_4_name:
    .string "ch4_unmap"
    .global app_5_name
app_5_name:
    .string "ch4_unmap2"
app_name_end:
