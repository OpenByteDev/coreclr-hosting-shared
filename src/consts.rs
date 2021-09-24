use super::size_t;

/// The maximum path length on windows platforms ([kind of / not really](https://docs.microsoft.com/en-us/windows/win32/fileio/maximum-file-path-limitation?tabs=cmd)).
pub const MAX_PATH: size_t = 260;
