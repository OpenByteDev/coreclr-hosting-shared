/// The char type used in the hosting components. Defined as [`u16`] on windows and as [`c_char`](std::os::raw::c_char) otherwise.
#[cfg(all(not(windows), std))]
pub type char_t = std::os::raw::c_char;
/// The char type used in the hosting components. Defined as [`u16`] on windows and as [`c_char`](cty::c_char) otherwise.
#[cfg(all(not(windows), not(std)))]
pub type char_t = cty::c_char;
/// The char type used in the hosting components. Defined as [`u16`] on windows and as [`c_char`](std::os::raw::c_char) otherwise.
#[cfg(all(windows, std))]
pub type char_t = u16;
/// The char type used in the hosting components. Defined as [`u16`] on windows and as [`c_char`](cty::c_char) otherwise.
#[cfg(all(windows, not(std)))]
pub type char_t = u16;

/// Equivalent to `size_t` in C.
pub type size_t = usize; // TODO: use `core::os::raw::c_size_t` instead once stabilized.
