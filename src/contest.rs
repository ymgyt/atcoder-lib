pub mod cio {
    use std::fmt::{self, Debug};
    use std::io::{BufRead, Cursor, Stdin, StdinLock};
    use std::str::FromStr;

    const INITIAL_BUF_SIZE: usize = 1024;

    pub type Result<T, E = Error> = std::result::Result<T, E>;

    #[derive(Debug)]
    pub enum Error {
        Io { source: std::io::Error },
        Utf8 { source: std::str::Utf8Error },
        Parse { message: String },
        Eof,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl std::error::Error for Error {}

    impl From<std::io::Error> for Error {
        fn from(source: std::io::Error) -> Self {
            Error::Io { source }
        }
    }

    impl From<std::str::Utf8Error> for Error {
        fn from(source: std::str::Utf8Error) -> Self {
            Error::Utf8 { source }
        }
    }

    impl Error {
        fn parse_error<E: Debug>(err: E) -> Self {
            Error::Parse {
                message: format!("{:?}", err),
            }
        }
    }

    pub struct Scanner<R> {
        reader: R,
        buf: Vec<u8>,
        pos: usize,
    }

    impl<'a> From<&'a Stdin> for Scanner<StdinLock<'a>> {
        fn from(stdin: &'a Stdin) -> Self {
            Scanner::new(stdin.lock())
        }
    }

    impl<'a> From<&'a str> for Scanner<Cursor<&'a str>> {
        fn from(s: &'a str) -> Self {
            Scanner::new(Cursor::new(s))
        }
    }

    pub trait FromScanner<R>
    where
        R: BufRead,
    {
        fn from_scanner(s: &mut Scanner<R>) -> Self
        where
            Self: Sized,
        {
            Self::try_from_scanner(s).unwrap()
        }

        fn try_from_scanner(s: &mut Scanner<R>) -> Result<Self>
        where
            Self: Sized;
    }

    macro_rules! impl_scanner {
        ($t:ty) => {
            impl<R> FromScanner<R> for $t
            where
                R: BufRead,
            {
                fn try_from_scanner(s: &mut Scanner<R>) -> Result<Self>
                where
                    Self: Sized,
                {
                    s.try_parse()
                }
            }
        };

        ($($t:ident),+) => {
            impl<R,$($t),+ > FromScanner<R> for ($($t),+)
            where
                R: BufRead,
                $($t: FromScanner<R>),+
            {
                fn try_from_scanner(s: &mut Scanner<R>) -> Result<Self>
                where
                    Self: Sized,
                {
                    Ok((
                      $($t::try_from_scanner(s)?),+
                    ))
                }
            }
        }
    }
    impl_scanner!(usize);
    impl_scanner!(isize);
    impl_scanner!(u8);
    impl_scanner!(u32);
    impl_scanner!(u64);
    impl_scanner!(i8);
    impl_scanner!(i32);
    impl_scanner!(i64);
    impl_scanner!(String);
    impl_scanner!(char);
    impl_scanner!(T1, T2);
    impl_scanner!(T1, T2, T3);
    impl_scanner!(T1, T2, T3, T4);

    impl<R> Scanner<R>
    where
        R: BufRead,
    {
        fn new(reader: R) -> Self {
            Self {
                reader,
                buf: Vec::with_capacity(INITIAL_BUF_SIZE),
                pos: 0,
            }
        }

        pub fn scan<T>(&mut self) -> T
        where
            T: FromScanner<R>,
        {
            T::from_scanner(self)
        }

        pub fn scan_n<T>(&mut self, n: usize) -> Vec<T>
        where
            T: FromScanner<R>,
        {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(T::from_scanner(self));
            }
            v
        }

        pub fn parse<T>(&mut self) -> T
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            match self.try_parse() {
                Ok(v) => v,
                Err(err) => panic!("{}", err),
            }
        }

        pub fn try_parse<T>(&mut self) -> Result<T>
        where
            T: FromStr,
            <T as FromStr>::Err: Debug,
        {
            if self.buf.is_empty() {
                self.fill_buf()?;
            }

            let mut from = None;

            loop {
                match (self.buf[self.pos], from.is_some()) {
                    // ignore space
                    (b' ', false) => self.pos += 1,

                    // read all, so handle next line
                    (b'\n', false) => self.fill_buf()?,

                    // found target start index
                    (_, false) => {
                        from = Some(self.pos);
                        self.pos += 1;
                    }

                    // found target, try parse
                    (b' ', true) | (b'\n', true) => break,

                    // keep checking
                    (_, true) => self.pos += 1,
                }
            }

            let part = std::str::from_utf8(&self.buf[from.unwrap()..self.pos])?;
            part.parse::<T>().map_err(Error::parse_error)
        }

        /// read a line from underlying reader and store it in the buffer.
        fn fill_buf(&mut self) -> Result<()> {
            self.buf.clear();
            self.pos = 0;
            if self.reader.read_until(b'\n', &mut self.buf)? == 0 {
                Err(Error::Eof)
            } else {
                // ensure buf end in a newline
                match self.buf.last() {
                    Some(b'\n') => (),
                    Some(_) | None => self.buf.push(b'\n'),
                }
                Ok(())
            }
        }
    }

    #[allow(unused_macros)]
    macro_rules! setup {
        ( $scanner:ident ) => {
            let _stdin = std::io::stdin();
            let mut $scanner = cio::Scanner::from(&_stdin);
        };
    }
    #[allow(unused_imports)]
    pub(crate) use setup;
}

#[cfg(test)]
mod test {
    use super::cio::*;

    #[test]
    fn new() {
        let stdin = std::io::stdin();
        let _ = Scanner::from(&stdin);

        let input = "1 2 3";
        let _ = Scanner::from(input);
    }

    #[test]
    fn scan() {
        let input = "1 -20\nABC 30.1\n";
        let mut scanner = Scanner::from(input);

        assert_eq!(scanner.parse::<i64>(), 1);
        assert_eq!(scanner.parse::<i64>(), -20);
        assert_eq!(scanner.parse::<String>(), String::from("ABC"));
        assert_eq!(scanner.parse::<f64>(), 30.1);
    }

    #[test]
    fn eof() {
        let input = "10\n";
        let mut scanner = Scanner::from(input);

        assert_eq!(scanner.parse::<i64>(), 10);
        assert!(matches!(scanner.try_parse::<i64>(), Err(Error::Eof)));
    }

    #[test]
    fn no_newline() {
        let input = "10 20";
        let mut scanner = Scanner::from(input);

        assert_eq!(scanner.parse::<u32>(), 10);
        assert_eq!(scanner.parse::<u32>(), 20);
    }

    #[test]
    fn should_scan() {
        let input = "123 10 20 1 1 2 2 3 3";
        let mut scanner = Scanner::from(input);

        assert_eq!(scanner.scan::<usize>(), 123);
        assert_eq!(scanner.scan::<(usize, usize)>(), (10, 20));
        assert_eq!(
            scanner.scan_n::<(usize, usize)>(3),
            vec![(1, 1), (2, 2), (3, 3),]
        )
    }
}
