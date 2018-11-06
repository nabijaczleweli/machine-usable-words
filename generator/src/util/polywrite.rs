use std::io::{Result, Write};
use std::cmp;


/// An output sink, writing the data passed thereto to two sinks contained therein.
///
/// If an error occurs, it will be propagated out of the first sink first.
///
/// The `write()` funxion returns the *bigger* of the two written lengths.
///
/// # Examples
///
/// ```
/// # use bloguen::util::PolyWrite;
/// # use std::io::Write;
/// let mut out_1 = vec![];
/// let mut out_2 = vec![];
///
/// PolyWrite(&mut out_1, &mut out_2).write_all("Бenlo".as_bytes()).unwrap();
///
/// assert_eq!(out_1, "Бenlo".as_bytes());
/// assert_eq!(out_2, "Бenlo".as_bytes());
/// ```
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolyWrite<Wr1: Write, Wr2: Write>(pub Wr1, pub Wr2);


impl<Wr1: Write, Wr2: Write> Write for PolyWrite<Wr1, Wr2> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let r0 = self.0.write(buf)?;
        let r1 = self.1.write(buf)?;

        Ok(cmp::max(r0, r1))
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()?;
        self.1.flush()?;

        Ok(())
    }
}
