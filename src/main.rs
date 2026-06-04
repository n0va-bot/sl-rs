/*========================================
 *    main.rs: SL-rs version 1.0.0
 *	Copyright 1993-2014 Toyoda Masashi (mtoyoda@acm.org)
 *	Copyright 2026 N0\A (n0va@krzak.org)
 *========================================
 */

use libc;
use ncurses::*;
use std::env;
use std::thread;
use std::time::Duration;

static mut ACCIDENT: bool = false;
static mut LOGO: bool = false;
static mut FLY: bool = false;
static mut C51: bool = false;

const D51HEIGHT: i32 = 10;
const D51FUNNEL: i32 = 7;
const D51LENGTH: i32 = 83;
const D51PATTERNS: usize = 6;

const D51STR1: &str = "      ====        ________                ___________ ";
const D51STR2: &str = "  _D _|  |_______/        \\__I_I_____===__|_________| ";
const D51STR3: &str = "   |(_)---  |   H\\________/ |   |        =|___ ___|   ";
const D51STR4: &str = "   /     |  |   H  |  |     |   |         ||_| |_||   ";
const D51STR5: &str = "  |      |  |   H  |__--------------------| [___] |   ";
const D51STR6: &str = "  | ________|___H__/__|_____/[][]~\\_______|       |   ";
const D51STR7: &str = "  |/ |   |-----------I_____I [][] []  D   |=======|__ ";

const D51WHL11: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL12: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL13: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            ";

const D51WHL21: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL22: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        ";
const D51WHL23: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51WHL31: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ ";
const D51WHL32: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL33: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51WHL41: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ ";
const D51WHL42: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL43: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51WHL51: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL52: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        ";
const D51WHL53: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51WHL61: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ ";
const D51WHL62: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        ";
const D51WHL63: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            ";

const D51DEL: &str = "                                                      ";

const COAL01: &str = "                              ";
const COAL02: &str = "                              ";
const COAL03: &str = "    _________________         ";
const COAL04: &str = "   _|                \\_____A  ";
const COAL05: &str = " =|                        |  ";
const COAL06: &str = " -|                        |  ";
const COAL07: &str = "__|________________________|_ ";
const COAL08: &str = "|__________________________|_ ";
const COAL09: &str = "   |_D__D__D_|  |_D__D__D_|   ";
const COAL10: &str = "    \\_/   \\_/    \\_/   \\_/    ";

const COALDEL: &str = "                              ";

const LOGOHEIGHT: i32 = 6;
const LOGOFUNNEL: i32 = 4;
const LOGOLENGTH: i32 = 84;
const LOGOPATTERNS: usize = 6;

const LOGO1: &str = "     ++      +------ ";
const LOGO2: &str = "     ||      |+-+ |  ";
const LOGO3: &str = "   /---------|| | |  ";
const LOGO4: &str = "  + ========  +-+ |  ";

const LWHL11: &str = " _|--O========O~\\-+  ";
const LWHL12: &str = "//// \\_/      \\_/    ";

const LWHL21: &str = " _|--/O========O\\-+  ";
const LWHL22: &str = "//// \\_/      \\_/    ";

const LWHL31: &str = " _|--/~O========O-+  ";
const LWHL32: &str = "//// \\_/      \\_/    ";

const LWHL41: &str = " _|--/~\\------/~\\-+  ";
const LWHL42: &str = "//// \\_O========O    ";

const LWHL51: &str = " _|--/~\\------/~\\-+  ";
const LWHL52: &str = "//// \\O========O/    ";

const LWHL61: &str = " _|--/~\\------/~\\-+  ";
const LWHL62: &str = "//// O========O_/    ";

const LCOAL1: &str = "____                 ";
const LCOAL2: &str = "|   \\@@@@@@@@@@@     ";
const LCOAL3: &str = "|    \\@@@@@@@@@@@@@_ ";
const LCOAL4: &str = "|                  | ";
const LCOAL5: &str = "|__________________| ";
const LCOAL6: &str = "   (O)       (O)     ";

const LCAR1: &str = "____________________ ";
const LCAR2: &str = "|  ___ ___ ___ ___ | ";
const LCAR3: &str = "|  |_| |_| |_| |_| | ";
const LCAR4: &str = "|__________________| ";
const LCAR5: &str = "|__________________| ";
const LCAR6: &str = "   (O)        (O)    ";

const DELLN: &str = "                     ";

const C51HEIGHT: i32 = 11;
const C51FUNNEL: i32 = 7;
const C51LENGTH: i32 = 87;
const C51PATTERNS: usize = 6;

const C51DEL: &str = "                                                       ";

const C51STR1: &str = "        ___                                            ";
const C51STR2: &str = "       _|_|_  _     __       __             ___________";
const C51STR3: &str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|";
const C51STR4: &str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|  ";
const C51STR5: &str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  ";
const C51STR6: &str = "    ||        | ::       H  +=====+      |  |::  ...|  ";
const C51STR7: &str = "|    | _______|_::-----------------[][]-----|       |  ";

const C51WH61: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH62: &str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__";
const C51WH63: &str = "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     ";
const C51WH64: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51WH51: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH52: &str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__";
const C51WH53: &str = "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     ";
const C51WH54: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51WH41: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH42: &str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__";
const C51WH43: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51WH44: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51WH31: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH32: &str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__";
const C51WH33: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51WH34: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51WH21: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH22: &str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__";
const C51WH23: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     ";
const C51WH24: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

const C51WH11: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__";
const C51WH12: &str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__";
const C51WH13: &str = "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     ";
const C51WH14: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       ";

#[derive(Copy, Clone)]
struct Smoke {
    y: i32,
    x: i32,
    ptrn: i32,
    kind: i32,
}

const SMOKEPTNS: usize = 16;

static mut S: [Smoke; 1000] = [Smoke {
    y: 0,
    x: 0,
    ptrn: 0,
    kind: 0,
}; 1000];
static mut SUM: i32 = 0;

const SMOKE0: [&str; SMOKEPTNS] = [
    "(   )", "(    )", "(    )", "(   )", "(  )", "(  )", "( )", "( )", "()", "()", "O", "O", "O",
    "O", "O", " ",
];

const SMOKE1: [&str; SMOKEPTNS] = [
    "(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)", "(@@)", "(@)", "(@)", "@@", "@@", "@", "@", "@",
    "@", "@", " ",
];

const ERASER: [&str; SMOKEPTNS] = [
    "     ", "      ", "      ", "     ", "    ", "    ", "   ", "   ", "  ", "  ", " ", " ", " ",
    " ", " ", " ",
];

const DY: [i32; SMOKEPTNS] = [2, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

const DX: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3];

fn my_mvaddstr(y: i32, x: i32, str: &str) -> i32 {
    let mut x = x;
    let mut iter = str.chars();

    while x < 0 {
        match iter.next() {
            None => return ERR,
            Some(_) => {}
        }
        x += 1;
    }

    for c in iter {
        if mvaddch(y, x, c as u32) == ERR {
            return ERR;
        }
        x += 1;
    }
    OK
}

fn option(str: &str) {
    for c in str.chars() {
        match c {
            'a' => unsafe { ACCIDENT = true },
            'F' => unsafe { FLY = true },
            'l' => unsafe { LOGO = true },
            'c' => unsafe { C51 = true },
            _ => {}
        }
    }
}

#[allow(non_snake_case)]
fn add_sl(x: i32) -> i32 {
    static SL: [[&str; LOGOHEIGHT as usize + 1]; LOGOPATTERNS] = [
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
        [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN],
    ];

    static COAL: [&str; LOGOHEIGHT as usize + 1] =
        [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];

    static CAR: [&str; LOGOHEIGHT as usize + 1] = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

    if x < -LOGOLENGTH {
        return ERR;
    }
    let mut y = LINES() / 2 - 3;
    let mut py1 = 0;
    let mut py2 = 0;
    let mut py3 = 0;

    if unsafe { FLY } {
        y = (x / 6) + LINES() - (COLS() / 6) - LOGOHEIGHT;
        py1 = 2;
        py2 = 4;
        py3 = 6;
    }
    for i in 0..=LOGOHEIGHT {
        let idx = ((LOGOLENGTH + x) / 3 % LOGOPATTERNS as i32) as usize;
        my_mvaddstr(y + i, x, SL[idx][i as usize]);
        my_mvaddstr(y + i + py1, x + 21, COAL[i as usize]);
        my_mvaddstr(y + i + py2, x + 42, CAR[i as usize]);
        my_mvaddstr(y + i + py3, x + 63, CAR[i as usize]);
    }
    if unsafe { ACCIDENT } {
        add_man(y + 1, x + 14);
        add_man(y + 1 + py2, x + 45);
        add_man(y + 1 + py2, x + 53);
        add_man(y + 1 + py3, x + 66);
        add_man(y + 1 + py3, x + 74);
    }
    add_smoke(y - 1, x + LOGOFUNNEL);
    OK
}

#[allow(non_snake_case)]
fn add_D51(x: i32) -> i32 {
    static D51: [[&str; D51HEIGHT as usize + 1]; D51PATTERNS] = [
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL11, D51WHL12,
            D51WHL13, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL21, D51WHL22,
            D51WHL23, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL31, D51WHL32,
            D51WHL33, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL41, D51WHL42,
            D51WHL43, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL51, D51WHL52,
            D51WHL53, D51DEL,
        ],
        [
            D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7, D51WHL61, D51WHL62,
            D51WHL63, D51DEL,
        ],
    ];

    static COAL: [&str; D51HEIGHT as usize + 1] = [
        COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL,
    ];

    if x < -D51LENGTH {
        return ERR;
    }
    let mut y = LINES() / 2 - 5;
    let mut dy = 0;

    if unsafe { FLY } {
        y = (x / 7) + LINES() - (COLS() / 7) - D51HEIGHT;
        dy = 1;
    }
    for i in 0..=D51HEIGHT {
        let idx = ((D51LENGTH + x) % D51PATTERNS as i32) as usize;
        my_mvaddstr(y + i, x, D51[idx][i as usize]);
        my_mvaddstr(y + i + dy, x + 53, COAL[i as usize]);
    }
    if unsafe { ACCIDENT } {
        add_man(y + 2, x + 43);
        add_man(y + 2, x + 47);
    }
    add_smoke(y - 1, x + D51FUNNEL);
    OK
}

#[allow(non_snake_case)]
fn add_C51(x: i32) -> i32 {
    static C51: [[&str; C51HEIGHT as usize + 1]; C51PATTERNS] = [
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH11, C51WH12,
            C51WH13, C51WH14, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH21, C51WH22,
            C51WH23, C51WH24, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH31, C51WH32,
            C51WH33, C51WH34, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH41, C51WH42,
            C51WH43, C51WH44, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH51, C51WH52,
            C51WH53, C51WH54, C51DEL,
        ],
        [
            C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7, C51WH61, C51WH62,
            C51WH63, C51WH64, C51DEL,
        ],
    ];

    static COAL: [&str; C51HEIGHT as usize + 1] = [
        COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05, COAL06, COAL07, COAL08, COAL09, COAL10,
        COALDEL,
    ];

    if x < -C51LENGTH {
        return ERR;
    }
    let mut y = LINES() / 2 - 5;
    let mut dy = 0;

    if unsafe { FLY } {
        y = (x / 7) + LINES() - (COLS() / 7) - C51HEIGHT;
        dy = 1;
    }
    for i in 0..=C51HEIGHT {
        let idx = ((C51LENGTH + x) % C51PATTERNS as i32) as usize;
        my_mvaddstr(y + i, x, C51[idx][i as usize]);
        my_mvaddstr(y + i + dy, x + 55, COAL[i as usize]);
    }
    if unsafe { ACCIDENT } {
        add_man(y + 3, x + 45);
        add_man(y + 3, x + 49);
    }
    add_smoke(y - 1, x + C51FUNNEL);
    OK
}

fn add_man(y: i32, x: i32) {
    static MAN: [[&str; 2]; 2] = [["", "(O)"], ["Help!", "\\O/"]];

    for i in 0..2 {
        let idx = ((LOGOLENGTH + x) / 12 % 2) as usize;
        my_mvaddstr(y + i, x, MAN[idx][i as usize]);
    }
}

fn add_smoke(y: i32, x: i32) {
    if x % 4 == 0 {
        unsafe {
            for i in 0..SUM as usize {
                my_mvaddstr(S[i].y, S[i].x, ERASER[S[i].ptrn as usize]);
                S[i].y -= DY[S[i].ptrn as usize];
                S[i].x += DX[S[i].ptrn as usize];
                if S[i].ptrn < SMOKEPTNS as i32 - 1 {
                    S[i].ptrn += 1;
                }
                let kind = S[i].kind as usize;
                let ptrn = S[i].ptrn as usize;
                if kind == 0 {
                    my_mvaddstr(S[i].y, S[i].x, SMOKE0[ptrn]);
                } else {
                    my_mvaddstr(S[i].y, S[i].x, SMOKE1[ptrn]);
                }
            }
            let kind = (SUM % 2) as usize;
            if kind == 0 {
                my_mvaddstr(y, x, SMOKE0[0]);
            } else {
                my_mvaddstr(y, x, SMOKE1[0]);
            }
            let sum = SUM as usize;
            S[sum].y = y;
            S[sum].x = x;
            S[sum].ptrn = 0;
            S[sum].kind = SUM % 2;
            SUM += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        if args[i].starts_with('-') {
            option(&args[i][1..]);
        }
    }

    initscr();
    unsafe { libc::signal(libc::SIGINT, libc::SIG_IGN) };
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    nodelay(stdscr(), true);
    leaveok(stdscr(), true);
    scrollok(stdscr(), false);

    let mut x = COLS() - 1;
    loop {
        if unsafe { LOGO } {
            if add_sl(x) == ERR {
                break;
            }
        } else if unsafe { C51 } {
            if add_C51(x) == ERR {
                break;
            }
        } else {
            if add_D51(x) == ERR {
                break;
            }
        }
        getch();
        refresh();
        thread::sleep(Duration::from_micros(40000));
        x -= 1;
    }

    mvcur(0, COLS() - 1, LINES() - 1, 0);
    endwin();
}
