/*
 * DEPRECTATED
 *
 * This module relies on a Linux kernel interface that was removed in 6.0. The interface was
 * deprecated with the 5.10 release of the Linux kernel.
 *
 * This module is included for support for kernels < 5.10. It should not be used for current or
 * future Linux kernels.
 */
mod efi_variables;

pub use efi_variables::EfiVariables;
