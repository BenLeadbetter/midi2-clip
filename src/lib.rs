#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clip(Vec<u32>);

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Error {
    IncorrectClipHeader,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectClipHeader => write!(f, "Incorrect MIDI 2.0 clip file header"),
        }
    }
}

impl std::error::Error for Error {}

impl Clip {
    pub fn read_clip_file<Read: std::io::Read>(stream: &mut Read) -> Result<Self, Error> {
        read_clip_header(stream)?;
        Ok(Clip(Vec::new()))
    }

    pub fn write_clip_file<Write: std::io::Write>(&self, _output: &mut Write) -> Self {
        todo!()
    }

    #[cfg(feature = "smf")]
    pub fn read_smf<Read: std::io::Read>(_stream: &mut Read) -> Result<Self, Error> {
        todo!()
    }

    #[cfg(feature = "smf")]
    pub fn write_smf<Write: std::io::Write>(&self, _output: &mut Write) -> Self {
        todo!()
    }
}

fn read_clip_header<Read: std::io::Read>(stream: &mut Read) -> Result<(), Error> {
    let mut buffer = [0x0_u8; 8];
    if stream.read_exact(&mut buffer).is_err()
        || buffer != [0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50]
    {
        return Err(Error::IncorrectClipHeader);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_clip_checks_for_missing_header() {
        let data: &[u8] = &[];
        let mut read = std::io::Cursor::new(data);
        assert_eq!(
            Clip::read_clip_file(&mut read),
            Err(Error::IncorrectClipHeader),
        );
    }

    #[test]
    fn read_clip_checks_for_incorrect_header() {
        let data: &[u8] = &[0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x0];
        let mut read = std::io::Cursor::new(data);
        assert_eq!(
            Clip::read_clip_file(&mut read),
            Err(Error::IncorrectClipHeader),
        );
    }

    #[test]
    fn read_clip_checks_for_correct_header() {
        let data: &[u8] = &[0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50];
        let mut read = std::io::Cursor::new(data);
        assert_eq!(Clip::read_clip_file(&mut read), Ok(Clip(Vec::new())));
    }
}
