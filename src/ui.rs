use std::process::exit;

use ncurses::*;

use crate::{
    constants::obtener_nombre_lenguaje,
    files::{format_permissions, is_file, Archivo},
    word::KeyType,
};

const START_X: i32 = 7; // x=0 in the editor
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
    pub start_horizontal: i32,
    pub end_horizontal: i32,
    pub command: String,
    pub terminado: bool,
    pub buffer_len: Vec<i32>,
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

        let mut buffer_len = Vec::new();
        let mut counter = 0;
        for i in &archivo.buffer {
            for j in i {
                counter += j.txt.len() as i32;
            }
            counter += i.len() as i32 - 1;
            buffer_len.push(counter);
            counter = 0;
        }

        Self {
            buffer_len,
            archivo,
            terminado: false,
            w,
            win,
            start_horizontal: 0,
            end_horizontal: w - 2,
            x: START_X,
            y: START_Y,
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
            panic!();
            self.win = newwin(h, w, 0, 0);
            self.h = h;
            self.w = w;
            self.end = w - 5;
            self.idx_x = 0;
            self.x = START_X;
            self.y = START_Y;
            self.idx_y = 0;
            self.start = 0;
            self.end_horizontal = w - 2;
            self.start_horizontal = 0;
            self.display();
        }

        if self.archivo.width == 0 {
            for (idx, _i) in (self.start..self.end + self.start - 1).enumerate() {
                mvwprintw(self.win, (idx + 1) as i32, 1, "~");
            }

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
            for (_idx, i) in (self.start..self.end + self.start).enumerate() {
                if i > (self.archivo.buffer.len() - 1) as i32 {
                    break;
                }
                let mut counter = 0;
                for (_, y) in self.archivo.buffer[i as usize].iter().enumerate() {
                    match y.keyword {
                        KeyType::Keyword => {
                            wattron(self.win, COLOR_PAIR(5));
                            mvwprintw(self.win, (_idx + 1) as i32, counter + START_X, &y.txt);
                            wattroff(self.win, COLOR_PAIR(5))
                        }
                        _ => mvwprintw(self.win, (_idx + 1) as i32, counter + START_X, &y.txt),
                    };
                    counter += y.txt.len() as i32 + 1;
                }
                let f = format!("{:<5}", i + 1);
                mvwprintw(self.win, (_idx + 1) as i32, 2, &f);
            }

            for i in self.archivo.buffer.len()..(self.end - 2) as usize {
                mvwprintw(self.win, (i + 1) as i32, 1, "~");
            }

            self.display_bar_debug();
        }
        let ff = format!("{}{}", self.start, self.end);
        mvwprintw(self.win, 2, 50, &ff);

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

        let format = format!("'{}' {}KB  ", self.archivo.path, metadata.len(),);
        let right = format!("{:?} {} {}/{} ", lang, per, self.idx_x, self.idx_y);

        let x = getmaxx(self.win);
        mvwhline(self.win, self.h - 2, 1, 32, x - 2);
        if !self.mode {
            mvwprintw(self.win, self.h - 2, 2, "NORMAL");
        } else {
            mvwprintw(self.win, self.h - 2, 2, "INSERT");
        }
        mvwprintw(self.win, self.h - 2, 10, &format);
        mvwprintw(self.win, self.h - 2, self.w - right.len() as i32, &right);
        wmove(self.win, self.y, self.x);
        wrefresh(self.win);
    }

    pub fn display_bar_debug(&self) {
        let f = match &self.archivo.file {
            Some(file) => file,
            None => return,
        };
        let metadata = match f.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return,
        };

        let format = format!("'{}' {}KB  ", self.archivo.path, metadata.len(),);
        let right = format!("xy{}/{} idx{}/{} ", self.x, self.y, self.idx_x, self.idx_y);

        let x = getmaxx(self.win);
        mvwhline(self.win, self.h - 2, 1, 32, x - 2);
        if !self.mode {
            mvwprintw(self.win, self.h - 2, 2, "NORMAL");
        } else {
            mvwprintw(self.win, self.h - 2, 2, "INSERT");
        }
        mvwprintw(self.win, self.h - 2, 10, &format);
        mvwprintw(self.win, self.h - 2, self.w - right.len() as i32, &right);
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
                106 => {
                    self.handle_movment_down();
                }
                107 => {
                    self.handle_movment_up();
                }
                104 => {
                    self.handle_movment_left();
                }
                108 => {
                    self.handle_movment_right();
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
                let f = format!("Command not found: {}", self.command);
                self.display();
                mvwprintw(self.win, self.h - 2, 1, &f);
            }
        }
    }

    fn handle_movment_left(&mut self) {
        if self.x > START_X {
            self.x -= 1;
            self.idx_x -= 1;
        }
    }
    //L
    fn handle_movment_right(&mut self) {
        if self.x > self.x - 2 && self.x - START_X < self.buffer_len[self.idx_y] as i32 {
            self.x += 1;
            self.idx_x += 1;
        }
    }
    //K
    fn handle_movment_up(&mut self) {
        if self.y > START_Y {
            self.y -= 1;
            self.idx_y -= 1;
            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
            self.idx_x = self.archivo.buffer[self.idx_y].len();
        } else if self.start > 0 {
            self.start -= 1;
            self.idx_y -= 1;
            self.idx_x = self.archivo.buffer[self.idx_y].len();
            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
            wclear(self.win);
        }
    }
    //J
    fn handle_movment_down(&mut self) {
        if self.y <= self.h - 6 && self.idx_y < self.archivo.buffer.len() - 1 {
            self.y += 1;
            self.idx_y += 1;
            self.x = self.buffer_len[self.idx_y] as i32 + START_X;
            self.idx_x = self.buffer_len[self.idx_y] as usize;
        } else if self.idx_y < self.archivo.buffer.len() - 1 {
            self.start += 1;
            self.idx_y += 1;
            self.idx_x = self.buffer_len[self.idx_y] as usize;
            self.x = self.buffer_len[self.idx_y] as i32 + START_X;
            wclear(self.win);
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
