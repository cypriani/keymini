use keyberon::action::{k, m, Action::*, HoldTapAction, HoldTapConfig};
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
const SENTER: Action = m(&[LShift, Enter].as_slice());
const CBS: Action = m(&[LCtrl, BSpace].as_slice());

const COLON: Action = m(&[LShift, N].as_slice());
const EQUAL: Action = m(&[RAlt, G].as_slice());

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
const S_TAB: Action = hold_tap!(k(LAlt), k(Tab));
const C_ESC: Action = hold_tap!(k(LCtrl), k(Escape));
// LGui is direct access

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

/* Layer definitions */

pub const NUM_LAYERS: usize = 4;

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<10, 4, NUM_LAYERS, ()> = keyberon::layout::layout! {
    // Main layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [   Q       W       E       R       T       Y       U       I       O       P   ],
        [   A     {H_S}   {H_D}   {H_F}     G       H     {H_J}   {H_K}   {H_L}     ;   ],
        [   Z       X       C       V       B       N       M       ,       .       /   ],
        [   n       n Application Space    (1)     (2)   RShift   RAlt      n       n   ],
    }
    // Layer (1) - nav layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [ CapsLock PScreen ScrollLock Pause n     Home   PgDown   PgUp     End      n   ],
        [   n     LGui   {C_ESC} {S_TAB} {STAB}   Left    Down     Up     Right   Enter ],
        [ Undo    {CUT}  {COPY}  {PASTE} {ATAB}   {CBS}  BSpace  Delete  Insert {SENTER} ],
        [   n       n       t       t       n      (3)      t       t       n       n   ],
    }
    // Layer (2) - num layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [   !       @       #       $       %       ^       &       *      '('     ')'  ],
        [   1     {H_2}   {H_3}   {H_4}     5       6     {H_7}   {H_8}   {H_9}     0   ],
        [   n       n    {COLON}    .    {EQUAL}    N  KpPlus KpMinus KpSlash KpAsterisk],
        [   n       n       t       t      (3)      n       t       t       n       n   ],
    }
    // Layer (3) - fn layer
    { //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
        [  F1      F2      F3      F4      F5      F6      F7      F8      F9      F10  ],
        [   n     LAlt    LCtrl  LShift    '['     ']'   RShift   RCtrl   LAlt      n   ],
        [  F11     F12 NonUsBslash '`'      n       n     '\''    '\\'      -       =   ],
        [   n       n  {Custom(())} t       n       n       t       t       n       n   ],
    } //[···+··· ···+··· ···+··· ···+··· ···+···|···+··· ···+··· ···+··· ···+··· ···+···],
};
