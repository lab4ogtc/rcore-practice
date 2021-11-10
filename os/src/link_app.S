
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 3
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_2_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00_write_a.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01_write_b.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02_write_c.bin"
app_2_end:

    .section .data
    .global _app_names
_app_names:
    .quad app_0_name
    .quad app_1_name
    .quad app_2_name
    .quad app_name_end

    .section .data
    .global app_0_name
app_0_name:
    .string "00_write_a"
    .global app_1_name
app_1_name:
    .string "01_write_b"
    .global app_2_name
app_2_name:
    .string "02_write_c"
app_name_end:
