//! Holds our default (initial) keyboard layout/actions
#![allow(dead_code)]
use keyberon::action::{k, l, Action::*};
use keyberon::key_code::KeyCode::*;

// NOTE: What most folks consider the "Menu" key is actually the "Application" key in Keyberon./
// TODO: Figure out a way to read this in from a config file
// TODO: Also figure out how to make this configurable via the USB serial port
// NOTE: For the prototype we're using a 4x6-ish numpad but the keys don't have a nice 1-to-1 mapping
//       of multiplexer-pin-to-key.  This was because routing on the PCB required compromises.
//       Here's the schema mapping on the PCB:
/* The mapping is in the order of the keys with the designatin: <multiplexer>:<channel>

    0:7, 0:9, 1:6, 1:1,
    0:6, 0:10, 1:5, 1:15,
    0:4, 0:11, 1:4,
                    1:14,
    0:14, 0:12, 1:3,
                    1:12,
    0:15, 0:13, 1:2,
    0:0,    1:0

*/
#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<16, 5, 7, ()> = [
    /*
    Since Keyberon was made for key switch matrices and not multi-channel analog multiplexers
    our mapping below is vastly more arbitrary and based on the tracks of the PCB rather than
    anything easy like rows/columns.  To properly translate this you need to figure out which
    key maps to what multiplexer-channel combo.
    Example: For our numpad multiplexer 0, channel 0 connects to the "0" key.  So the very
    first k() translation goes to Kp0.

    */
    // For the sake of scanning, six 16-channel multiplexers are laid out where one
    // multiplexer is the equivalent to one row in a key matrix...
    [ // Layer 0 (default layer)
        // AM0 (PC version)
        // &[k(Z),k(LAlt),k(LGui),k(LShift),k(LCtrl),k(CapsLock),k(Tab),GRAVE_AND_CALC,
        //     k(Escape),k(Kb1),k(Kb2),k(Q),k(A),k(W),k(S),Trans],
        // AM0 (Mac version Alt aka Option and LGui aka Command are swapped)
        [k(Z),k(LGui),k(LAlt),k(LShift),k(LCtrl),k(CapsLock),k(Tab),Trans,
            k(Escape),k(Kb1),k(Kb2),k(Q),k(A),k(W),k(S),Trans],
        // AM1
        [k(R),k(Kb3),k(E),k(F),k(C),l(1),k(X),k(D),
            k(Kb4),k(Kb5),k(T),k(V),k(BSpace),k(Kb6),k(G),k(B)],
        // AM2
        [l(2),k(M),k(J),k(Space),k(N),k(H),k(Y),k(Kb7),
            k(U),k(Kb8),k(Kb9),k(I),k(Comma),k(K),Trans,Trans],
        // AM3
        [k(Slash),k(SColon),k(Kb0),k(P),k(RAlt),k(Dot),k(L),k(O),
            k(Minus),k(Equal),Trans,k(LBracket),k(RBracket),k(RCtrl),k(Quote),k(Application)],
        // AM4
        [k(Right),k(Up),k(Down),k(Left),k(RShift),k(Enter),k(BSpace),k(Bslash),
            Trans, // Encoder clockwise
            Trans, // Encoder counterclockwise
            Trans, // Encoder press
            Trans,Trans, // Unused pins
            k(Delete),Trans,k(Insert)], // Macro1, Macro2, and Macro3 (respectively)
    ], [ // Layer 1 (Fun)
        // AM0
        [k(Kb1),l(3),Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,k(F1),k(F2),Trans,Trans,Trans,Trans,Trans],
        // AM1
        [Trans,k(F3),Trans,Trans,Trans,Trans,Trans,Trans,
            k(F4),k(F5),Trans,Trans,k(Delete),k(F6),Trans,Trans],
        // AM2
        [l(2),Trans,Trans,k(Insert),Trans,Trans,Trans,k(F7),
            Trans,k(F8),k(F9),Trans,Trans,Trans,Trans,Trans],
        // AM3
        [Trans,Trans,Trans,Trans,l(5),Trans,Trans,Trans,
            k(F11),k(F12),Trans,Trans,Trans,Trans,Trans,Trans],
        // AM4
        [k(End),k(PgUp),k(PgDown),k(Home),Trans,Trans,k(Delete),Trans,
            k(VolDown), // Encoder clockwise
            k(VolUp), // Encoder counterclockwise
            Trans,
            Trans, // Encoder press
            Trans,Trans, // Unused pins (grounded)
            Trans,k(ScrollLock)], // Macro1, Macro2, and Macro3 (respectively)
    ], [ // Layer 2 (More Fun)
        // AM0
        [k(Kb2),l(3),Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,k(F1),k(F2),Trans,Trans,Trans,Trans,Trans],
        // AM1
        [Trans,k(F3),Trans,Trans,Trans,l(1),Trans,Trans,
            k(F4),k(F5),Trans,Trans,Trans,k(F6),Trans,Trans],
        // AM2
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,k(F7),
            Trans,k(F8),k(F9),Trans,Trans,Trans,Trans,Trans],
        // AM3
        [Trans,Trans,Trans,Trans,l(5),Trans,Trans,Trans,
            k(F11),k(F12),Trans,Trans,Trans,Trans,Trans,Trans],
        // AM4
        [k(End),k(PgUp),k(PgDown),k(Home),Trans,Trans,k(Delete),Trans,
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans, // Unused pins (grounded)
        ], // Macro1, Macro2, and Macro3 (respectively)
    ], [ // Layer 3 (Fun-More Fun)
        // AM0
        [k(Kb3),Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM1
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM2
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM3
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM4
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans, // Encoder clockwise
            Trans, // Encoder counterclockwise
            Trans, // Encoder press
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans,Trans], // Macro1, Macro2, and Macro3 (respectively)
    ], [ // Layer 4 (LAlt-Fun or LAlt-More Fun)
        // AM0
        [k(Kb4),Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM1
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM2
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM3
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM4
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans, // Encoder clockwise
            Trans, // Encoder counterclockwise
            Trans, // Encoder press
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans,Trans], // Macro1, Macro2, and Macro3 (respectively)
    ], [ // Layer 5 (RAlt-Fun or RAlt-More Fun)
        // AM0
        [k(Kb5),Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM1
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM2
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM3
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM4
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans, // Encoder clockwise
            Trans, // Encoder counterclockwise
            Trans, // Encoder press
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans,Trans], // Macro1, Macro2, and Macro3 (respectively)
    ], [ // Layer 6
        // AM0
        [k(Kb6),Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM1
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM2
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM3
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans],
        // AM4
        [Trans,Trans,Trans,Trans,Trans,Trans,Trans,Trans,
            Trans, // Encoder clockwise
            Trans, // Encoder counterclockwise
            Trans, // Encoder press
            Trans,Trans, // Unused pins (grounded)
            Trans,Trans,Trans], // Macro1, Macro2, and Macro3 (respectively)
    ],
];

/// Map key locations to multiplexer pins
pub type MuxMap = &'static [&'static [&'static [u8; 2]]];

pub static MUX_MAPPING: MuxMap = &[
    /*
    Since Keyberon was made for key switch matrices and not multi-channel analog multiplexers
    we need to map which key goes to which channel on which multiplexer.
    */
    &[ // Row 0 (just the ESC key)
        &[0,8],
    ],
    &[ // Row 1 (Tilda row)
        &[0,0], &[0,0], &[0,0], &[0,0], &[0,0], &[0,0], &[0,0], &[0,0], &[0,0], &[0,0], &[0,0],&[0,0],
    ],
    &[ // Row 2 (Tab row)
        &[0,0],
    ],
    &[ // Row 3 (CapsLock row)
        &[0,0],
    ],
    &[ // Row 4 (Shift row)
        &[0,0],
    ],
    &[ // Row 5 (Ctrl row)
        &[0,0],
    ],
    &[ // Infrared receiver (virtual row)
        &[0,0],
    ],
];
