const CLIP_FILE_HEADER: [u8; 8] = [0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50];

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Clip(Vec<u32>);

#[derive(Debug)]
pub enum Error {
    IncorrectClipHeader,
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncorrectClipHeader => write!(f, "Incorrect MIDI 2.0 clip file header"),
            Self::Io(io_err) => write!(f, "IO Error occurred: {io_err}"),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl std::error::Error for Error {}

impl Clip {
    pub fn read_clip_file<Read: std::io::Read>(stream: &mut Read) -> Result<Self, Error> {
        read_clip_header(stream)?;
        Ok(Clip(Vec::new()))
    }

    pub fn write_clip_file<Write: std::io::Write>(&self, output: &mut Write) -> Result<(), Error> {
        write_clip_header(output)?;
        Ok(())
    }

    #[cfg(feature = "smf")]
    pub fn read_smf<Read: std::io::Read>(_stream: &mut Read) -> Result<Self, Error> {
        todo!()
    }

    #[cfg(feature = "smf")]
    pub fn write_smf<Write: std::io::Write>(&self, _output: &mut Write) -> Result<(), Error> {
        todo!()
    }
}

fn read_clip_header<Read: std::io::Read>(stream: &mut Read) -> Result<(), Error> {
    let mut buffer = [0x0_u8; 8];
    if stream.read_exact(&mut buffer).is_err() || buffer != CLIP_FILE_HEADER {
        return Err(Error::IncorrectClipHeader);
    }
    Ok(())
}

fn write_clip_header<Write: std::io::Write>(output: &mut Write) -> Result<(), Error> {
    output.write_all(&CLIP_FILE_HEADER[..])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_clip_checks_for_missing_header() {
        let data: &[u8] = &[];
        let mut read = std::io::Cursor::new(data);
        assert!(Clip::read_clip_file(&mut read).is_err());
    }

    #[test]
    fn read_clip_checks_for_incorrect_header() {
        let data: &[u8] = &[0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x0];
        let mut read = std::io::Cursor::new(data);
        assert!(Clip::read_clip_file(&mut read).is_err());
    }

    #[test]
    fn read_clip_checks_for_correct_header() {
        let data: &[u8] = &[0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50];
        let mut read = std::io::Cursor::new(data);
        assert_eq!(Clip::read_clip_file(&mut read).unwrap(), Clip(Vec::new()));
    }

    #[test]
    fn default_clip() {
        assert_eq!(Clip::default(), Clip(Vec::new()));
    }

    #[test]
    fn write_clip_writes_correct_file_header() {
        let mut data = Vec::new();
        Clip::default().write_clip_file(&mut data).unwrap();
        assert_eq!(
            &data[..],
            &[0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50][..]
        );
    }
}
