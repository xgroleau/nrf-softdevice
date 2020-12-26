/*
 * Copyright (c) 2012 - 2019, Nordic Semiconductor ASA
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form, except as embedded into a Nordic
 *    Semiconductor ASA integrated circuit in a product or a software update for
 *    such product, must reproduce the above copyright notice, this list of
 *    conditions and the following disclaimer in the documentation and/or other
 *    materials provided with the distribution.
 *
 * 3. Neither the name of Nordic Semiconductor ASA nor the names of its
 *    contributors may be used to endorse or promote products derived from this
 *    software without specific prior written permission.
 *
 * 4. This software, with or without modification, must only be used with a
 *    Nordic Semiconductor ASA integrated circuit.
 *
 * 5. Any software provided in binary form under this license must not be reverse
 *    engineered, decompiled, modified and/or disassembled.
 *
 * THIS SOFTWARE IS PROVIDED BY NORDIC SEMICONDUCTOR ASA "AS IS" AND ANY EXPRESS
 * OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY, NONINFRINGEMENT, AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL NORDIC SEMICONDUCTOR ASA OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 * GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
 * OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_char = u8;

pub type c_short = i16;
pub type c_ushort = u16;

pub type c_int = i32;
pub type c_uint = u32;

pub type c_long = i32;
pub type c_ulong = u32;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type c_void = core::ffi::c_void;

trait ToAsm {
    fn to_asm(self) -> u32;
}

fn to_asm<T: ToAsm>(t: T) -> u32 {
    t.to_asm()
}

impl ToAsm for u32 {
    fn to_asm(self) -> u32 {
        self
    }
}

impl ToAsm for u16 {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl ToAsm for u8 {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl ToAsm for i8 {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl<T> ToAsm for *const T {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl<T> ToAsm for *mut T {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl<T: ToAsm> ToAsm for Option<T> {
    fn to_asm(self) -> u32 {
        match self {
            Some(x) => x.to_asm(),
            None => 0,
        }
    }
}

impl<X, R> ToAsm for unsafe extern "C" fn(X) -> R {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl<X, Y, R> ToAsm for unsafe extern "C" fn(X, Y) -> R {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

impl<X, Y, Z, R> ToAsm for unsafe extern "C" fn(X, Y, Z) -> R {
    fn to_asm(self) -> u32 {
        self as u32
    }
}

/* automatically generated by rust-bindgen 0.55.1 */

pub const MBR_SVC_BASE: u32 = 24;
pub const MBR_PAGE_SIZE_IN_WORDS: u32 = 1024;
pub const MBR_SIZE: u32 = 4096;
pub const MBR_BOOTLOADER_ADDR: u32 = 4088;
pub const MBR_PARAM_PAGE_ADDR: u32 = 4092;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type intmax_t = self::c_longlong;
pub type uintmax_t = self::c_ulonglong;
#[doc = "< ::sd_mbr_command"]
pub const NRF_MBR_SVCS_SD_MBR_COMMAND: NRF_MBR_SVCS = 24;
#[doc = "@brief nRF Master Boot Record API SVC numbers."]
pub type NRF_MBR_SVCS = self::c_uint;
#[doc = "< Copy a new BootLoader. @see ::sd_mbr_command_copy_bl_t"]
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_COPY_BL: NRF_MBR_COMMANDS = 0;
#[doc = "< Copy a new SoftDevice. @see ::sd_mbr_command_copy_sd_t"]
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_COPY_SD: NRF_MBR_COMMANDS = 1;
#[doc = "< Initialize forwarding interrupts to SD, and run reset function in SD. Does not require any parameters in ::sd_mbr_command_t params."]
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_INIT_SD: NRF_MBR_COMMANDS = 2;
#[doc = "< This command works like memcmp. @see ::sd_mbr_command_compare_t"]
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_COMPARE: NRF_MBR_COMMANDS = 3;
#[doc = "< Change the address the MBR starts after a reset. @see ::sd_mbr_command_vector_table_base_set_t"]
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_VECTOR_TABLE_BASE_SET: NRF_MBR_COMMANDS = 4;
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_RESERVED: NRF_MBR_COMMANDS = 5;
#[doc = "< Start forwarding all interrupts to this address. @see ::sd_mbr_command_irq_forward_address_set_t"]
pub const NRF_MBR_COMMANDS_SD_MBR_COMMAND_IRQ_FORWARD_ADDRESS_SET: NRF_MBR_COMMANDS = 6;
#[doc = "@brief Possible values for ::sd_mbr_command_t.command"]
pub type NRF_MBR_COMMANDS = self::c_uint;
#[doc = "@brief This command copies part of a new SoftDevice"]
#[doc = ""]
#[doc = " The destination area is erased before copying."]
#[doc = " If dst is in the middle of a flash page, that whole flash page will be erased."]
#[doc = " If (dst+len) is in the middle of a flash page, that whole flash page will be erased."]
#[doc = ""]
#[doc = " The user of this function is responsible for setting the BPROT registers."]
#[doc = ""]
#[doc = " @retval ::NRF_SUCCESS indicates that the contents of the memory blocks where copied correctly."]
#[doc = " @retval ::NRF_ERROR_INTERNAL indicates that the contents of the memory blocks where not verified correctly after copying."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sd_mbr_command_copy_sd_t {
    #[doc = "< Pointer to the source of data to be copied."]
    pub src: *mut u32,
    #[doc = "< Pointer to the destination where the content is to be copied."]
    pub dst: *mut u32,
    #[doc = "< Number of 32 bit words to copy. Must be a multiple of @ref MBR_PAGE_SIZE_IN_WORDS words."]
    pub len: u32,
}
#[test]
fn bindgen_test_layout_sd_mbr_command_copy_sd_t() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_copy_sd_t>(),
        12usize,
        concat!("Size of: ", stringify!(sd_mbr_command_copy_sd_t))
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_copy_sd_t>(),
        4usize,
        concat!("Alignment of ", stringify!(sd_mbr_command_copy_sd_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_copy_sd_t>())).src as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_copy_sd_t),
            "::",
            stringify!(src)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_copy_sd_t>())).dst as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_copy_sd_t),
            "::",
            stringify!(dst)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_copy_sd_t>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_copy_sd_t),
            "::",
            stringify!(len)
        )
    );
}
#[doc = "@brief This command works like memcmp, but takes the length in words."]
#[doc = ""]
#[doc = " @retval ::NRF_SUCCESS indicates that the contents of both memory blocks are equal."]
#[doc = " @retval ::NRF_ERROR_NULL indicates that the contents of the memory blocks are not equal."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sd_mbr_command_compare_t {
    #[doc = "< Pointer to block of memory."]
    pub ptr1: *mut u32,
    #[doc = "< Pointer to block of memory."]
    pub ptr2: *mut u32,
    #[doc = "< Number of 32 bit words to compare."]
    pub len: u32,
}
#[test]
fn bindgen_test_layout_sd_mbr_command_compare_t() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_compare_t>(),
        12usize,
        concat!("Size of: ", stringify!(sd_mbr_command_compare_t))
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_compare_t>(),
        4usize,
        concat!("Alignment of ", stringify!(sd_mbr_command_compare_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_compare_t>())).ptr1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_compare_t),
            "::",
            stringify!(ptr1)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_compare_t>())).ptr2 as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_compare_t),
            "::",
            stringify!(ptr2)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_compare_t>())).len as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_compare_t),
            "::",
            stringify!(len)
        )
    );
}
#[doc = "@brief This command copies a new BootLoader."]
#[doc = ""]
#[doc = " The MBR assumes that either @ref MBR_BOOTLOADER_ADDR or @ref MBR_UICR_BOOTLOADER_ADDR is set to"]
#[doc = " the address where the bootloader will be copied. If both addresses are set, the MBR will prioritize"]
#[doc = " @ref MBR_BOOTLOADER_ADDR."]
#[doc = ""]
#[doc = " The bootloader destination is erased by this function."]
#[doc = " If (destination+bl_len) is in the middle of a flash page, that whole flash page will be erased."]
#[doc = ""]
#[doc = " This command requires that @ref MBR_PARAM_PAGE_ADDR or @ref MBR_UICR_PARAM_PAGE_ADDR is set,"]
#[doc = " see @ref sd_mbr_command."]
#[doc = ""]
#[doc = " This command will use the flash protect peripheral (BPROT or ACL) to protect the flash that is"]
#[doc = " not intended to be written."]
#[doc = ""]
#[doc = " On success, this function will not return. It will start the new bootloader from reset-vector as normal."]
#[doc = ""]
#[doc = " @retval ::NRF_ERROR_INTERNAL indicates an internal error that should not happen."]
#[doc = " @retval ::NRF_ERROR_FORBIDDEN if the bootloader address is not set."]
#[doc = " @retval ::NRF_ERROR_INVALID_LENGTH if parameters attempts to read or write outside flash area."]
#[doc = " @retval ::NRF_ERROR_NO_MEM No MBR parameter page is provided. See @ref sd_mbr_command."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sd_mbr_command_copy_bl_t {
    #[doc = "< Pointer to the source of the bootloader to be be copied."]
    pub bl_src: *mut u32,
    #[doc = "< Number of 32 bit words to copy for BootLoader."]
    pub bl_len: u32,
}
#[test]
fn bindgen_test_layout_sd_mbr_command_copy_bl_t() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_copy_bl_t>(),
        8usize,
        concat!("Size of: ", stringify!(sd_mbr_command_copy_bl_t))
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_copy_bl_t>(),
        4usize,
        concat!("Alignment of ", stringify!(sd_mbr_command_copy_bl_t))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_copy_bl_t>())).bl_src as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_copy_bl_t),
            "::",
            stringify!(bl_src)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_copy_bl_t>())).bl_len as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_copy_bl_t),
            "::",
            stringify!(bl_len)
        )
    );
}
#[doc = "@brief Change the address the MBR starts after a reset"]
#[doc = ""]
#[doc = " Once this function has been called, this address is where the MBR will start to forward"]
#[doc = " interrupts to after a reset."]
#[doc = ""]
#[doc = " To restore default forwarding, this function should be called with @ref address set to 0. If a"]
#[doc = " bootloader is present, interrupts will be forwarded to the bootloader. If not, interrupts will"]
#[doc = " be forwarded to the SoftDevice."]
#[doc = ""]
#[doc = " The location of a bootloader can be specified in @ref MBR_BOOTLOADER_ADDR or"]
#[doc = " @ref MBR_UICR_BOOTLOADER_ADDR. If both addresses are set, the MBR will prioritize"]
#[doc = " @ref MBR_BOOTLOADER_ADDR."]
#[doc = ""]
#[doc = " This command requires that @ref MBR_PARAM_PAGE_ADDR or @ref MBR_UICR_PARAM_PAGE_ADDR is set,"]
#[doc = " see @ref sd_mbr_command."]
#[doc = ""]
#[doc = " On success, this function will not return. It will reset the device."]
#[doc = ""]
#[doc = " @retval ::NRF_ERROR_INTERNAL indicates an internal error that should not happen."]
#[doc = " @retval ::NRF_ERROR_INVALID_ADDR if parameter address is outside of the flash size."]
#[doc = " @retval ::NRF_ERROR_NO_MEM No MBR parameter page is provided. See @ref sd_mbr_command."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sd_mbr_command_vector_table_base_set_t {
    #[doc = "< The base address of the interrupt vector table for forwarded interrupts."]
    pub address: u32,
}
#[test]
fn bindgen_test_layout_sd_mbr_command_vector_table_base_set_t() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_vector_table_base_set_t>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(sd_mbr_command_vector_table_base_set_t)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_vector_table_base_set_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(sd_mbr_command_vector_table_base_set_t)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_vector_table_base_set_t>())).address as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_vector_table_base_set_t),
            "::",
            stringify!(address)
        )
    );
}
#[doc = "@brief Sets the base address of the interrupt vector table for interrupts forwarded from the MBR"]
#[doc = ""]
#[doc = " Unlike sd_mbr_command_vector_table_base_set_t, this function does not reset, and it does not"]
#[doc = " change where the MBR starts after reset."]
#[doc = ""]
#[doc = " @retval ::NRF_SUCCESS"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sd_mbr_command_irq_forward_address_set_t {
    #[doc = "< The base address of the interrupt vector table for forwarded interrupts."]
    pub address: u32,
}
#[test]
fn bindgen_test_layout_sd_mbr_command_irq_forward_address_set_t() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_irq_forward_address_set_t>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(sd_mbr_command_irq_forward_address_set_t)
        )
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_irq_forward_address_set_t>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(sd_mbr_command_irq_forward_address_set_t)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_irq_forward_address_set_t>())).address
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_irq_forward_address_set_t),
            "::",
            stringify!(address)
        )
    );
}
#[doc = "@brief Input structure containing data used when calling ::sd_mbr_command"]
#[doc = ""]
#[doc = " Depending on what command value that is set, the corresponding params value type must also be"]
#[doc = " set. See @ref NRF_MBR_COMMANDS for command types and corresponding params value type. If command"]
#[doc = " @ref SD_MBR_COMMAND_INIT_SD is set, it is not necessary to set any values under params."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_mbr_command_t {
    #[doc = "< Type of command to be issued. See @ref NRF_MBR_COMMANDS."]
    pub command: u32,
    #[doc = "< Command parameters."]
    pub params: sd_mbr_command_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sd_mbr_command_t__bindgen_ty_1 {
    #[doc = "< Parameters for copy SoftDevice."]
    pub copy_sd: sd_mbr_command_copy_sd_t,
    #[doc = "< Parameters for verify."]
    pub compare: sd_mbr_command_compare_t,
    #[doc = "< Parameters for copy BootLoader. Requires parameter page."]
    pub copy_bl: sd_mbr_command_copy_bl_t,
    #[doc = "< Parameters for vector table base set. Requires parameter page."]
    pub base_set: sd_mbr_command_vector_table_base_set_t,
    #[doc = "< Parameters for irq forward address set"]
    pub irq_forward_address_set: sd_mbr_command_irq_forward_address_set_t,
    _bindgen_union_align: [u32; 3usize],
}
#[test]
fn bindgen_test_layout_sd_mbr_command_t__bindgen_ty_1() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_t__bindgen_ty_1>(),
        12usize,
        concat!("Size of: ", stringify!(sd_mbr_command_t__bindgen_ty_1))
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(sd_mbr_command_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_t__bindgen_ty_1>())).copy_sd as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t__bindgen_ty_1),
            "::",
            stringify!(copy_sd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_t__bindgen_ty_1>())).compare as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t__bindgen_ty_1),
            "::",
            stringify!(compare)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_t__bindgen_ty_1>())).copy_bl as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t__bindgen_ty_1),
            "::",
            stringify!(copy_bl)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_t__bindgen_ty_1>())).base_set as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t__bindgen_ty_1),
            "::",
            stringify!(base_set)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::core::ptr::null::<sd_mbr_command_t__bindgen_ty_1>())).irq_forward_address_set
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t__bindgen_ty_1),
            "::",
            stringify!(irq_forward_address_set)
        )
    );
}
#[test]
fn bindgen_test_layout_sd_mbr_command_t() {
    assert_eq!(
        ::core::mem::size_of::<sd_mbr_command_t>(),
        16usize,
        concat!("Size of: ", stringify!(sd_mbr_command_t))
    );
    assert_eq!(
        ::core::mem::align_of::<sd_mbr_command_t>(),
        4usize,
        concat!("Alignment of ", stringify!(sd_mbr_command_t))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_t>())).command as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t),
            "::",
            stringify!(command)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<sd_mbr_command_t>())).params as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sd_mbr_command_t),
            "::",
            stringify!(params)
        )
    );
}

#[doc = "@brief Issue Master Boot Record commands"]
#[doc = ""]
#[doc = " Commands used when updating a SoftDevice and bootloader."]
#[doc = ""]
#[doc = " The @ref SD_MBR_COMMAND_COPY_BL and @ref SD_MBR_COMMAND_VECTOR_TABLE_BASE_SET requires"]
#[doc = " parameters to be retained by the MBR when resetting the IC. This is done in a separate flash"]
#[doc = " page. The location of the flash page should be provided by the application in either"]
#[doc = " @ref MBR_PARAM_PAGE_ADDR or @ref MBR_UICR_PARAM_PAGE_ADDR. If both addresses are set, the MBR"]
#[doc = " will prioritize @ref MBR_PARAM_PAGE_ADDR. This page will be cleared by the MBR and is used to"]
#[doc = " store the command before reset. When an address is specified, the page it refers to must not be"]
#[doc = " used by the application. If no address is provided by the application, i.e. both"]
#[doc = " @ref MBR_PARAM_PAGE_ADDR and @ref MBR_UICR_PARAM_PAGE_ADDR is 0xFFFFFFFF, MBR commands which use"]
#[doc = " flash will be unavailable and return @ref NRF_ERROR_NO_MEM."]
#[doc = ""]
#[doc = " @param[in]  param Pointer to a struct describing the command."]
#[doc = ""]
#[doc = " @note For a complete set of return values, see ::sd_mbr_command_copy_sd_t,"]
#[doc = "       ::sd_mbr_command_copy_bl_t, ::sd_mbr_command_compare_t,"]
#[doc = "       ::sd_mbr_command_vector_table_base_set_t, ::sd_mbr_command_irq_forward_address_set_t"]
#[doc = ""]
#[doc = " @retval ::NRF_ERROR_NO_MEM No MBR parameter page provided"]
#[doc = " @retval ::NRF_ERROR_INVALID_PARAM if an invalid command is given."]
#[inline(always)]
pub unsafe fn sd_mbr_command(param: *mut sd_mbr_command_t) -> u32 {
    let ret: u32;
    asm!("svc 24",
        inout("r0") to_asm(param) => ret,
        lateout("r1") _,
        lateout("r2") _,
        lateout("r3") _,
        lateout("r12") _,
    );
    ret
}
