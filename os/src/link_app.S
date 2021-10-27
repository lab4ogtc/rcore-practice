
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 8
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_7_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/00_hello_world.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/01_store_fault.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/02_power.bin"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/03_write0.bin"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/04_write1.bin"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
app_5_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/05_bad_instruction.bin"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
app_6_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/06_bad_address.bin"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
app_7_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/07_bad_register.bin"
app_7_end:

    .section .data
    .global _app_names
_app_names:
    .quad app_0_name
    .quad app_1_name
    .quad app_2_name
    .quad app_3_name
    .quad app_4_name
    .quad app_5_name
    .quad app_6_name
    .quad app_7_name
    .quad app_name_end

    .section .data
    .global app_0_name
app_0_name:
    .string "00_hello_world"
    .global app_1_name
app_1_name:
    .string "01_store_fault"
    .global app_2_name
app_2_name:
    .string "02_power"
    .global app_3_name
app_3_name:
    .string "03_write0"
    .global app_4_name
app_4_name:
    .string "04_write1"
    .global app_5_name
app_5_name:
    .string "05_bad_instruction"
    .global app_6_name
app_6_name:
    .string "06_bad_address"
    .global app_7_name
app_7_name:
    .string "07_bad_register"
app_name_end:
