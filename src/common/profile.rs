use super::utils::AvgStd;
use std::fmt;
use std::num;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct ProfileEntry {
    pub batch_size: u32,
    pub latency: AvgStd,
    pub memory: usize,
    pub num_repeats: u32,
}

#[derive(Debug, PartialEq)]
pub struct ModelProfile {
    pub framework: String,
    pub model: String,
    pub version: String,
    pub gpu_model: String,
    pub gpu_uuid: String,
    pub forwards: Vec<ProfileEntry>,
    pub preproc: AvgStd,
    pub preproc_repeat: u32,
    pub postproc: AvgStd,
    pub postproc_repeat: u32,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("ParseError: {msg} In line: `{line}`")]
    ParseError { msg: String, line: String },

    #[error(transparent)]
    ParseIntError(#[from] num::ParseIntError),

    #[error(transparent)]
    ParseFloatError(#[from] num::ParseFloatError),

    #[error("MissingLineError")]
    MissingLineError,
}

fn next_line<'a>(iter: &'a mut std::str::Lines) -> Result<&'a str, Error> {
    iter.next().ok_or(Error::MissingLineError)
}

fn expect_str(line: &str, expected: &str) -> Result<(), Error> {
    if line == expected {
        Ok(())
    } else {
        Err(Error::ParseError {
            msg: format!("Expecting {}.", expected),
            line: line.to_owned(),
        })
    }
}

fn expect_split<'a>(
    line: &'a str,
    pattern: &str,
    expected_fields: usize,
) -> Result<Vec<&'a str>, Error> {
    let split: Vec<_> = line.split(pattern).map(str::trim).collect();
    if split.len() == expected_fields {
        Ok(split)
    } else {
        Err(Error::ParseError {
            msg: format!(
                "Expected {} fields but got {}.",
                expected_fields,
                split.len()
            ),
            line: line.to_owned(),
        })
    }
}

fn parse_avgstd_repeat(s: &str) -> Result<(AvgStd, u32), Error> {
    let split = expect_split(s, ",", 3)?;
    let avg = split[0].parse::<f64>()?;
    let std = split[1].parse::<f64>()?;
    let repeat = split[2].parse::<u32>()?;
    Ok((AvgStd { avg, std } * 1e-6, repeat))
}

impl FromStr for ProfileEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = expect_split(s, ",", 5)?;
        let batch_size = split[0].parse::<u32>()?;
        let avg = split[1].parse::<f64>()?;
        let std = split[2].parse::<f64>()?;
        let memory = split[3].parse::<usize>()?;
        let num_repeats = split[4].parse::<u32>()?;
        let latency = AvgStd { avg, std } * 1e-6;
        Ok(ProfileEntry {
            batch_size,
            latency,
            memory,
            num_repeats,
        })
    }
}

impl fmt::Display for ProfileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_us = self.latency * 1e6;
        write!(
            f,
            "{},{},{},{},{}",
            self.batch_size, lat_us.avg, lat_us.std, self.memory, self.num_repeats
        )
    }
}

const LINE_FORWARD: &str = "Forward latency";
const LINE_TABLE_HEADER: &str = "batch,latency(us),std(us),memory(B),repeat";
const LINE_PREPROCESS: &str = "Preprocess latency (mean,std,repeat)";
const LINE_POSTPROCESS: &str = "Postprocess latency (mean,std,repeat)";

impl FromStr for ModelProfile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = &mut s.trim().lines();
        let split = expect_split(next_line(iter)?, ":", 3)?;
        let framework = split[0].to_owned();
        let model = split[1].to_owned();
        let version = split[2].to_owned();
        let gpu_model = next_line(iter)?.trim().to_owned();
        let gpu_uuid = next_line(iter)?.trim().to_owned();
        expect_str(next_line(iter)?.trim(), LINE_FORWARD)?;
        expect_str(next_line(iter)?.trim(), LINE_TABLE_HEADER)?;
        let mut forwards = vec![ProfileEntry {
            batch_size: 0,
            latency: AvgStd { avg: 0.0, std: 0.0 },
            memory: 0,
            num_repeats: 0,
        }];
        loop {
            let line = next_line(iter)?.trim();
            if line == LINE_PREPROCESS {
                break;
            }
            let entry = line.parse()?;
            forwards.push(entry);
        }
        let (preproc, preproc_repeat) = parse_avgstd_repeat(next_line(iter)?)?;
        expect_str(
            next_line(iter)?.trim(),
            "Postprocess latency (mean,std,repeat)",
        )?;
        let (postproc, postproc_repeat) = parse_avgstd_repeat(next_line(iter)?)?;
        Ok(ModelProfile {
            framework,
            model,
            version,
            gpu_model,
            gpu_uuid,
            forwards,
            preproc,
            preproc_repeat,
            postproc,
            postproc_repeat,
        })
    }
}

impl fmt::Display for ModelProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:{}:{}", self.framework, self.model, self.version)?;
        writeln!(f, "{}", self.gpu_model)?;
        writeln!(f, "{}", self.gpu_uuid)?;
        writeln!(f, "{}", LINE_FORWARD)?;
        writeln!(f, "{}", LINE_TABLE_HEADER)?;
        for entry in self.forwards[1..].iter() {
            writeln!(f, "{}", entry)?;
        }
        writeln!(f, "{}", LINE_PREPROCESS)?;
        let pre = self.preproc * 1e6;
        writeln!(f, "{},{},{}", pre.avg, pre.std, self.preproc_repeat)?;
        writeln!(f, "{}", LINE_POSTPROCESS)?;
        let post = self.postproc * 1e6;
        writeln!(f, "{},{},{}", post.avg, post.std, self.postproc_repeat)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_profile_entry() {
        // Should tolerate empty spaces.
        assert_eq!(
            " 12, 88794.6, 2243.51,8054112256, 10 "
                .parse::<ProfileEntry>()
                .unwrap(),
            ProfileEntry {
                batch_size: 12,
                latency: AvgStd {
                    avg: 88794.6,
                    std: 2243.51
                } * 1e-6,
                memory: 8054112256,
                num_repeats: 10
            }
        );

        // More fields than expected.
        assert!("12, 88794.6, 2243.51,8054112256, 10, "
            .parse::<ProfileEntry>()
            .is_err());

        // Fewer fields than expected.
        assert!("12, 88794.6, 2243.51,8054112256"
            .parse::<ProfileEntry>()
            .is_err());

        // Type mismatches.
        assert!("12.3, 88794.6, 2243.51, 8054112256, 10"
            .parse::<ProfileEntry>()
            .is_err());
        assert!("12, avg, 2243.51, 8054112256, 10"
            .parse::<ProfileEntry>()
            .is_err());
        assert!("12, 88794.6, std, 8054112256, 10"
            .parse::<ProfileEntry>()
            .is_err());
        assert!("12, 88794.6, 2243.51, 8054112256.2, 10"
            .parse::<ProfileEntry>()
            .is_err());
        assert!("12, 88794.6, 2243.51, 8054112256, 10.9"
            .parse::<ProfileEntry>()
            .is_err());
    }

    #[test]
    fn serialize_profile_entry() {
        let a = " 12, 88794.6, 2243.51,8054112256, 10 "
            .parse::<ProfileEntry>()
            .unwrap();
        let s = a.to_string();
        let b = s.parse::<ProfileEntry>().unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn parse_and_serialize_profile() {
        let txt = r#"
            tensorflow:resnet_0:1
            Tesla_K80
            cfd0ec64-4978-7096-f391-921f9c5c5d27
            Forward latency
            batch,latency(us),std(us),memory(B),repeat
            1,20232.7,638.153,8054112256,10
            2,26258.7,605.536,8054112256,10
            3,31336.5,757.278,8054112256,10
            4,42357.3,8079.06,8054112256,10
            5,45092.0,647.367,8054112256,10
            Preprocess latency (mean,std,repeat)
            1073.2,107.746,901
            Postprocess latency (mean,std,repeat)
            7.888,1.335,2000
        "#;

        // Parsing
        let profile = txt.parse::<ModelProfile>().unwrap();
        assert_eq!(profile.framework, "tensorflow");
        assert_eq!(profile.model, "resnet_0");
        assert_eq!(profile.version, "1");
        assert_eq!(profile.gpu_model, "Tesla_K80");
        assert_eq!(profile.gpu_uuid, "cfd0ec64-4978-7096-f391-921f9c5c5d27");
        assert_eq!(profile.forwards.len(), 6);
        for i in 0..6 {
            assert_eq!(profile.forwards[i].batch_size, i as u32);
        }
        assert_eq!(profile.preproc.avg, 1073.2e-6);
        assert_eq!(profile.preproc.std, 107.746e-6);
        assert_eq!(profile.preproc_repeat, 901);
        assert_eq!(profile.postproc.avg, 7.888e-6);
        assert_eq!(profile.postproc.std, 1.335e-6);
        assert_eq!(profile.postproc_repeat, 2000);

        // Serialization
        let s = profile.to_string();
        let b = s.parse::<ModelProfile>().unwrap();
        assert_eq!(profile, b);
    }
}
