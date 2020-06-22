use std::io::{self};

pub struct StringStream {
    inner: String,
    index: usize,
}

impl StringStream {
    pub fn from(s : String) -> StringStream {
        StringStream {
            inner:s,
            index:0,
        }
    }
}

impl io::Read for StringStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut slice = &self.inner.as_bytes()[self.index..self.inner.len()];
        let n = slice.read(buf)?;
        self.index += n;
        return Ok(n)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_string_stream() {
        let mut ss = StringStream {
            inner: "toti".to_string(),
            index: 0
        };
        let rss : &mut dyn io::Read = &mut ss;
        let mut arr : [u8; 2] = [0; 2];

        let n = rss.read(&mut arr);
        assert_eq!(n.unwrap(), 2);
        assert_eq!(arr, ['t' as u8, 'o' as u8]);
        let n = rss.read(&mut arr);
        assert_eq!(n.unwrap(), 2);
        assert_eq!(arr, ['t' as u8, 'i' as u8]);
        let n = rss.read(&mut arr);
        assert_eq!(n.unwrap(), 0);
    }

}
