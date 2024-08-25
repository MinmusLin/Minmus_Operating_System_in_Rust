/*
 * Project Name:  MinmusOS
 * File Name:     linker.ld
 * File Function: Linker script of boot
 * Author:        Jishen Lin
 * License:       MIT License
 */

ENTRY(_start)

SECTIONS {
    . = 0X500;

    _stack_start = .;

    . = 0X7C00;

    _stack_end = .;

    .boot :
    {
        *(.boot .boot.*)
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.got .got.*)
    }

    . = 0X7C00 + 0X1B8;

    .diskid :
    {
        BYTE(0XFB)
        BYTE(0X00)
        BYTE(0XB1)
        BYTE(0XE5)
    }

    .reserved :
    {
        BYTE(0X00)
        BYTE(0X00)
    }

    .first_table :
    {
        BYTE(0X80)
        BYTE(0X20)
        BYTE(0X21)
        BYTE(0X00)
        BYTE(0X00)
        BYTE(0X41)
        BYTE(0X01)
        BYTE(0X00)
        BYTE(0X00)
        BYTE(0X08)
        BYTE(0X00)
        BYTE(0X00)
        BYTE(0X00)
        BYTE(0X08)
        BYTE(0X00)
        BYTE(0X00)
    }

    .second_table :
    {
        BYTE(0X00)
        BYTE(0X41)
        BYTE(0X02)
        BYTE(0X00)
        BYTE(0X06)
        BYTE(0X8A)
        BYTE(0X08)
        BYTE(0X82)
        BYTE(0X00)
        BYTE(0X10)
        BYTE(0X00)
        BYTE(0X00)
        BYTE(0X00)
        BYTE(0XF0)
        BYTE(0X1F)
        BYTE(0X00)
    }

    . = 0X7C00 + 0X1FE;

    .magic_number :
    {
        SHORT(0XAA55)
    }

    _bootloader_start = .;
}