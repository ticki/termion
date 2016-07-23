//! Style.

use std::fmt;

derive_csi_sequence!("Reset SGR parameters.", Reset, "m");
derive_csi_sequence!("Bold text.", Bold, "1m");
derive_csi_sequence!("Fainted text (not widely supported).", Faint, "2m");
derive_csi_sequence!("Italic text.", Italic, "3m");
derive_csi_sequence!("Underlined text.", Underline, "4m");
derive_csi_sequence!("Blinking text (not widely supported).", Blink, "5m");
derive_csi_sequence!("Inverted colors (negative mode).", Invert, "7m");
derive_csi_sequence!("Crossed out text (not widely supported).", CrossedOut, "9m");
derive_csi_sequence!("Framed text (not widely supported).", Framed, "51m");
