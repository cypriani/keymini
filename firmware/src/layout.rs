use keyberon::action::{k, l, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

macro_rules! hold_tap {
    ($hold:expr, $tap:expr) => {
        HoldTap(&HoldTapAction {
            timeout: 200,
            tap_hold_interval: 0,
            config: HoldTapConfig::Default,
            hold: $hold,
            tap: $tap,
        })
    };
}

/* Miscellaneous actions */

const CUT: Action = m(&[LShift, Delete].as_slice());
const COPY: Action = m(&[LCtrl, Insert].as_slice());
const PASTE: Action = m(&[LShift, Insert].as_slice());

const ATAB: Action = m(&[LAlt, Tab].as_slice());
const STAB: Action = m(&[LShift, Tab].as_slice());

// Map to Ergo-L key combinations
const COLON: Action = m(&[LShift, N].as_slice());
const COMMA: Action = k(Dot);
const DOT: Action = k(N);
const TNBS: Action = m(&[LShift, Space].as_slice());

/* Home row mods, left hand */

// main layer
const H_F: Action = hold_tap!(k(LAlt), k(F));
const H_D: Action = hold_tap!(k(LCtrl), k(D));
const H_S: Action = hold_tap!(k(LGui), k(S));

// num layer
const H_4: Action = hold_tap!(k(LAlt), k(Kb4));
const H_3: Action = hold_tap!(k(LCtrl), k(Kb3));
const H_2: Action = hold_tap!(k(LGui), k(Kb2));

// nav layer
const F_TAB: Action = hold_tap!(k(LAlt), k(Tab));
const D_ESC: Action = k(LCtrl);
const S_GUI: Action = k(LGui);

/* Home row mods, right hand */
// NB We use LAlt here because RAlt is AltGr.

// main layer
const H_J: Action = hold_tap!(k(LAlt), k(J));
const H_K: Action = hold_tap!(k(RCtrl), k(K));
const H_L: Action = hold_tap!(k(RGui), k(L));

// num layer
const H_7: Action = hold_tap!(k(LAlt), k(Kb7));
const H_8: Action = hold_tap!(k(RCtrl), k(Kb8));
const H_9: Action = hold_tap!(k(RGui), k(Kb9));

// nav layer
// No HRM under the right hand (we need to be able to hold the arrows).

/* Thumb key actions */
const T_1: Action = l(3); // fn layer
const T_2: Action = hold_tap!(k(LShift), k(BSpace)); // Shift / Backspace / Del (on nav)
const T_3: Action = hold_tap!(l(1), k(Space)); // nav layer / Space
const T_4: Action = T_3;
const T_5: Action = hold_tap!(k(RAlt), k(Enter)); // AltGr / Enter / Esc (on nav)
const T_6: Action = l(2); // num layer

/* Layer definitions */

pub const NUM_LAYERS: usize = 5;

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<10, 4, NUM_LAYERS, ()> = keyberon::layout::layout! {
    // t = transparent (main layer action)
    // n = NoOp
    // Custom(()) = reflashing mode

    // Main layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [   Q       W       E       R       T       Y       U       I       O       P   ],
        [   A     {H_S}   {H_D}   {H_F}     G       H     {H_J}   {H_K}   {H_L}     ;   ],
        [   Z       X       C       V       B       N       M       ,       .       /   ],
        [   n       n     {T_1}   {T_2}   {T_3}   {T_4}   {T_5}   {T_6}     n       n   ],
    }

    // Layer (1) - nav layer
    // TODO: mouse emulation.
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [ Insert PScreen ScrollLock Pause Application
                                                  Home   PgDown   PgUp     End      n   ],
        [ CapsLock {S_GUI} {D_ESC} {F_TAB} {STAB} Left    Down     Up     Right     n   ],
        [ Undo    {CUT}  {COPY}  {PASTE} {ATAB}     n       n       n       n       n   ],
        [   n       n       t    Delete     t       t    Escape     t       n       n   ],
    }

    // Layer (2) - num layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [   !       @       #       $       %       ^       &       *      '('     ')'  ],
        [   1     {H_2}   {H_3}   {H_4}     5       6     {H_7}   {H_8}   {H_9}     0   ],
        [   n    {COLON} {COMMA}  {DOT}     n    KpEqual KpPlus KpMinus KpSlash KpAsterisk],
        [   n       n      (4)      t    {TNBS}     t       t       t       n       n   ],
    }

    // Layer (3) - fn layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [  F1      F2      F3      F4      F5      F6      F7      F8      F9      F10  ],
        [   n     LAlt    LCtrl  LShift    '['     ']'   RShift   RCtrl   LAlt      n   ],
        [  F11     F12 NonUsBslash '`'      n Application '\''    '\\'      -       =   ],
        [   n       n       t       t       t       t       t      (4)      n       n   ],
    } //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],

    // Layer (4) - utility/media layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [{Custom(())} n     n       n       n MediaMute MediaVolDown MediaVolUp n   n   ],
        [   n       n       n       n       n MediaPreviousSong MediaPlayPause MediaNextSong n n ],
        [MediaSleep n       n       n       n       n       n       n       n       n   ],
        [   n       n       t       t       n       n       t       t       n       n   ],
    } //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
};
