use std::process::exit;

use ncurses::*;

use crate::{
    constants::obtener_nombre_lenguaje,
    files::{format_permissions, is_file, Archivo},
};

const START_X: i32 = 1; // x=0 in the editor
const START_Y: i32 = 1;

#[derive(Debug)]
pub struct Ui {
    pub archivo: Archivo,
    pub w: i32,
    pub h: i32,
    pub win: WINDOW,
    pub mode: bool,
    pub x: i32,
    pub y: i32,
    pub idx_x: usize,
    pub idx_y: usize,
    pub start: i32,
    pub end: i32,
    pub command: String,
    pub terminado: bool,
}

impl Ui {
    pub fn new(p: Option<&str>) -> Self {
        initscr();
        noecho();
        keypad(stdscr(), true);
        raw();
        start_color();
        cbreak();
        init_color(COLOR_BLACK as i16, 110, 110, 110);
        init_color(COLOR_BLUE as i16, 40, 40, 1000);
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);
        init_pair(5, COLOR_RED, COLOR_BLACK);

        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());
        let end = h - 2;

        let win = newwin(h, w, 0, 0);

        let archivo = match p {
            Some(path) => match is_file(path) {
                true => Archivo::new(path),
                false => Archivo::default(),
            },
            None => Archivo::default(),
        };

        Self {
            archivo,
            terminado: false,
            w,
            win,
            x: START_X,
            y: START_X,
            command: String::new(),
            h,
            mode: false,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end,
        }
    }

    pub fn display(&mut self) {
        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());

        if w != self.w || h != self.h {
            self.win = newwin(h, w, 0, 0);
            self.h = h;
            self.w = w;
            self.end = w - 2;
            self.idx_x = 0;
            self.x = START_X;
            self.y = START_Y;
            self.idx_y = 0;
            self.start = 0;
            wrefresh(self.win);
        }

        for (idx, _i) in (self.start..self.end + self.start).enumerate() {
            mvwprintw(self.win, (idx + 1) as i32, 1, "~");
        }
        if self.archivo.width == 0 {
            let textos = [
                "Escriba :q para salir",
                "Rim es de codigo abierto y se puede distribuid libremente",
                "por Mikel Apesteguia",
                "",
                "Rim v0.0.1",
            ];
            for (i, v) in textos.iter().enumerate() {
                let pos_y = self.h / 2 - i as i32;
                let pos_x = (self.w / 2) - (v.len() as i32 / 2);
                mvwprintw(self.win, pos_y, pos_x, v);
            }
            self.display_bar_nofile();
        } else {
            self.display_bar_file();
        }

        wmove(self.win, self.y, self.x);
        wrefresh(self.win);
    }

    pub fn display_bar_nofile(&self) {
        let format = format!("{}/{}", self.idx_y, self.idx_x);

        let x = getmaxx(self.win);
        mvwhline(self.win, self.h - 2, 1, 32, x - 2);
        if !self.mode {
            mvwprintw(self.win, self.h - 2, 1, "NORMAL");
        } else {
            mvwprintw(self.win, self.h - 2, 1, "INSERT");
        }
        mvwprintw(
            self.win,
            self.h - 2,
            self.w - format.len() as i32 - 5,
            &format,
        );
        wmove(self.win, self.y, self.x);
        wrefresh(self.win);
    }

    pub fn display_bar_file(&self) {
        let f = match &self.archivo.file {
            Some(file) => file,
            None => return,
        };
        let metadata = match f.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return,
        };
        let per = format_permissions(metadata.permissions(), false);

        let file = self.archivo.path.split('/').last().unwrap();
        let lenguaje = file.split('.').last().unwrap();
        let lang = obtener_nombre_lenguaje(lenguaje);

        let format = format!(
            "{:?} {}  {}KB  {}:{}  x:{} y:{} realx:{}   realy:{}",
            lang,
            per,
            metadata.len(),
            self.idx_y,
            self.archivo.buffer.len(),
            self.x,
            self.y,
            self.idx_x,
            self.idx_y,
        );

        let x = getmaxx(self.win);
        mvwhline(self.win, self.h - 2, 1, 32, x - 2);
        if !self.mode {
            mvwprintw(self.win, self.h - 2, 2, "NORMAL");
        } else {
            mvwprintw(self.win, self.h - 2, 2, "INSERT");
        }
        mvwprintw(self.win, self.h - 2, 10, &format);
        wmove(self.win, self.y, self.x);
        wrefresh(self.win);
    }

    pub fn update(&mut self) {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        while !self.terminado {
            match ch {
                58 => {
                    self.handle_command();
                    self.handle_action();
                }
                _ => (),
            }
            self.display();
            ch = wgetch(self.win);
        }
        endwin();
    }

    pub fn handle_action(&mut self) {
        match self.command.as_str() {
            ":q" => {
                self.terminado = true;
                endwin();
                exit(0);
            }
            _ => {
                //mvwprintw(self.win, 10, 10, &self.command);
            }
        }
    }

    pub fn handle_command(&mut self) {
        let x = getmaxx(self.win);
        mvwhline(self.win, self.h - 2, 1, 32, x - 2);
        mvwprintw(self.win, self.h - 2, 1, ":");
        self.command.push(':');
        wrefresh(self.win);
        let mut terminado = false;

        let mut ch = wgetch(self.win);
        while !terminado {
            if ch == KEY_BACKSPACE {
                if self.command.is_empty() {
                    return;
                }
                self.command.pop();
                mvwhline(self.win, self.h - 2, 1, 32, x - 2);
                mvwprintw(self.win, self.h - 2, 1, &self.command);
            } else if ch == '\n' as i32 {
                mvwprintw(self.win, self.h - 2, 1, &self.command);
                terminado = true;
                self.handle_action();
            } else {
                self.command.push(char::from_u32(ch as u32).unwrap());
                mvwprintw(self.win, self.h - 2, 1, &self.command);
            }
            wrefresh(self.win);
            ch = wgetch(self.win);
        }
        self.command.clear();
        wclear(self.win);
    }
}
