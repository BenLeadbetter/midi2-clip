const CLIP_FILE_HEADER: [u8; 8] = [0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50];

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct Clip(Vec<u32>);

#[derive(Debug)]
pub enum Error {
    Parse(&'static str),
    Ump(midi2::error::InvalidData),
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Io(err) => write!(f, "IO Error occurred: {err}"),
            Parse(err) => write!(f, "Parsing Error occurred: {err}"),
            Ump(err) => write!(f, "Ump formatarsing Error occurred: {err}"),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl std::convert::From<midi2::error::InvalidData> for Error {
    fn from(value: midi2::error::InvalidData) -> Self {
        Error::Ump(value)
    }
}

impl std::error::Error for Error {}

impl Clip {
    pub fn read_clip_file<Read: std::io::Read>(stream: &mut Read) -> Result<Self, Error> {
        read_clip_header(stream)?;
        let mut clip = Clip(Vec::new());
        read_clip_configuration_header(stream, &mut clip)?;
        Ok(clip)
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
    stream.read_exact(&mut buffer)?;
    if buffer != CLIP_FILE_HEADER {
        return Err(Error::Parse("Incorrect clip header"));
    }
    Ok(())
}

fn read_u32<Read: std::io::Read>(stream: &mut Read, dst: &mut [u32]) -> Result<(), Error> {
    let mut buffer = [0u8; 4];
    for num in dst.iter_mut() {
        stream.read_exact(&mut buffer)?;
        *num = u32::from_be_bytes(buffer);
    }
    Ok(())
}

fn read_clip_configuration_header<Read: std::io::Read>(
    stream: &mut Read,
    clip: &mut Clip,
) -> Result<(), Error> {
    let mut buffer = [0x0_u32; 4];

    // todo: read profiles

    {
        // empty delta clockstamp
        read_u32(stream, &mut buffer[..1])?;
        let delta_clock_stamp = midi2::utility::DeltaClockstamp::try_from(&buffer[..1])?;
        if delta_clock_stamp.time_data() != 0x0 {
            return Err(Error::Parse(
                "Initial delta clockstamp message should be set to 0",
            ));
        }
        clip.0.extend_from_slice(&buffer[..1]);
    }

    {
        // delta clockstamp ticks per quarter note
        read_u32(stream, &mut buffer[..1])?;
        midi2::utility::DeltaClockstampTpq::try_from(&buffer[..1])?;
        clip.0.extend_from_slice(&buffer[..1]);
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
    fn read_single_u32() {
        let data: &[u8] = &[0x12, 0x34, 0x56, 0x78];
        let mut read = std::io::Cursor::new(data);
        let mut buffer = [0x0];
        read_u32(&mut read, &mut buffer[..]).unwrap();
        assert_eq!(buffer, [0x1234_5678]);
    }

    #[test]
    fn read_two_u32() {
        let data: &[u8] = &[0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44];
        let mut read = std::io::Cursor::new(data);
        let mut buffer = [0x0, 0x0];
        read_u32(&mut read, &mut buffer[..]).unwrap();
        assert_eq!(buffer, [0x1234_5678, 0x1122_3344]);
    }

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
        let data: &[u8] = &[
            0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50, // header
            0x00, 0x20, 0x00, 0x00, // dcs
            0x00, 0x30, 0x00, 0x00, // dcstpqn
        ];
        let mut read = std::io::Cursor::new(data);
        assert_eq!(
            Clip::read_clip_file(&mut read).unwrap(),
            Clip(vec![0x0020_0000, 0x0030_0000,])
        );
    }

    #[test]
    fn write_clip_writes_correct_file_header() {
        let mut data = Vec::new();
        Clip(Vec::new()).write_clip_file(&mut data).unwrap();
        assert_eq!(
            &data[..],
            &[0x53, 0x4D, 0x46, 0x32, 0x43, 0x4C, 0x49, 0x50][..]
        );
    }
}
