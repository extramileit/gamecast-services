use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    PitchStrike ,
    PitchBall,
    Single,
    Double,
    Triple,
    HomeRun,
    Walk,
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PITCH_STRIKE" => Ok(Self::PitchStrike),
            "PITCH_BALL" => Ok(Self::PitchBall),
            "SINGLE" => Ok(Self::Single),
            "DOUBLE" => Ok(Self::Double),
            "TRIPLE" => Ok(Self::Triple),
            "HOME_RUN" => Ok(Self::HomeRun),
            "WALK" => Ok(Self::Walk),
            _ => Err(CommandError),
        }
    }
}

pub struct CommandError;
