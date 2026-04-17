/*!
An interface for defining spatial audio rendering modes and
implementing mode-agnostic renderers for the xpans Ecosystem

## Examples
For examples on how to implement rendering modes, refer to
[xpans Stereo](https://github.com/xpans-audio/xpans_stereo) and
[xpans Headphones](https://github.com/xpans-audio/xpans_headphones).

For an example on how to implement a renderer, refer to
[xpans Violet](https://github.com/xpans-audio/xpans_violet).
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod std;

pub mod input;
pub mod interpret;
pub mod output;
pub mod process;

pub mod prelude;
