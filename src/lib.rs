#![allow(dead_code)]

pub trait Color {
    type Output;

    fn black(self) -> Self::Output;
    fn red(self) -> Self::Output;
    fn green(self) -> Self::Output;
    fn yellow(self) -> Self::Output;
    fn blue(self) -> Self::Output;
    fn purple(self) -> Self::Output;
    fn cyan(self) -> Self::Output;
    fn white(self) -> Self::Output;

    fn italic(self) -> Self::Output;
    fn bold(self) -> Self::Output;
    fn underlined(self) -> Self::Output;
}

impl Color for &str {
    type Output = String;

    fn black(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[30m");
        x.push_str("\x1b[0m");
        x
    }

    fn red(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[31m");
        x.push_str("\x1b[0m");
        x
    }

    fn green(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[32m");
        x.push_str("\x1b[0m");
        x
    }

    fn yellow(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[33m");
        x.push_str("\x1b[0m");
        x
    }

    fn blue(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[34m");
        x.push_str("\x1b[0m");
        x
    }

    fn purple(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[35m");
        x.push_str("\x1b[0m");
        x
    }

    fn cyan(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[36m");
        x.push_str("\x1b[0m");
        x
    }

    fn white(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[37m");
        x.push_str("\x1b[0m");
        x
    }

    fn italic(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[3m");
        x.push_str("\x1b[0m");
        x
    }

    fn bold(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[1m");
        x.push_str("\x1b[0m");
        x
    }

    fn underlined(self) -> Self::Output {
        let mut x = self.to_string();
        x.insert_str(0, "\x1b[4m");
        x.push_str("\x1b[0m");
        x
    }
}
