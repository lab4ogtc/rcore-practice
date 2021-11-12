
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 13
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_3_start
    .quad app_4_start
    .quad app_5_start
    .quad app_6_start
    .quad app_7_start
    .quad app_8_start
    .quad app_9_start
    .quad app_10_start
    .quad app_11_start
    .quad app_12_start
    .quad app_12_end

    .section .data
    .global app_0_start
    .global app_0_end
app_0_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_0_setprio.bin"
app_0_end:

    .section .data
    .global app_1_start
    .global app_1_end
app_1_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_0_sleep.bin"
app_1_end:

    .section .data
    .global app_2_start
    .global app_2_end
app_2_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_0_sleep1.bin"
app_2_end:

    .section .data
    .global app_3_start
    .global app_3_end
app_3_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_1_yield0.bin"
app_3_end:

    .section .data
    .global app_4_start
    .global app_4_end
app_4_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_1_yield1.bin"
app_4_end:

    .section .data
    .global app_5_start
    .global app_5_end
app_5_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_1_yield2.bin"
app_5_end:

    .section .data
    .global app_6_start
    .global app_6_end
app_6_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_2_stride0.bin"
app_6_end:

    .section .data
    .global app_7_start
    .global app_7_end
app_7_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_2_stride1.bin"
app_7_end:

    .section .data
    .global app_8_start
    .global app_8_end
app_8_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_2_stride2.bin"
app_8_end:

    .section .data
    .global app_9_start
    .global app_9_end
app_9_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_2_stride3.bin"
app_9_end:

    .section .data
    .global app_10_start
    .global app_10_end
app_10_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_2_stride4.bin"
app_10_end:

    .section .data
    .global app_11_start
    .global app_11_end
app_11_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3_2_stride5.bin"
app_11_end:

    .section .data
    .global app_12_start
    .global app_12_end
app_12_start:
    .incbin "../user/target/riscv64gc-unknown-none-elf/release/ch3t_deadloop.bin"
app_12_end:

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
    .quad app_8_name
    .quad app_9_name
    .quad app_10_name
    .quad app_11_name
    .quad app_12_name
    .quad app_name_end

    .section .data
    .global app_0_name
app_0_name:
    .string "ch3_0_setprio"
    .global app_1_name
app_1_name:
    .string "ch3_0_sleep"
    .global app_2_name
app_2_name:
    .string "ch3_0_sleep1"
    .global app_3_name
app_3_name:
    .string "ch3_1_yield0"
    .global app_4_name
app_4_name:
    .string "ch3_1_yield1"
    .global app_5_name
app_5_name:
    .string "ch3_1_yield2"
    .global app_6_name
app_6_name:
    .string "ch3_2_stride0"
    .global app_7_name
app_7_name:
    .string "ch3_2_stride1"
    .global app_8_name
app_8_name:
    .string "ch3_2_stride2"
    .global app_9_name
app_9_name:
    .string "ch3_2_stride3"
    .global app_10_name
app_10_name:
    .string "ch3_2_stride4"
    .global app_11_name
app_11_name:
    .string "ch3_2_stride5"
    .global app_12_name
app_12_name:
    .string "ch3t_deadloop"
app_name_end:
