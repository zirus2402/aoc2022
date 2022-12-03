enum HandShape {
    Rock(u8),
    Paper(u8),
    Scissors(u8),
}

#[derive(Debug)]
enum RoundOutcome {
    Lost(u8),
    Draw(u8),
    Win(u8),
}

impl TryFrom<char> for HandShape {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(HandShape::Rock(1)),
            'B' | 'Y' => Ok(HandShape::Paper(2)),
            'C' | 'Z' => Ok(HandShape::Scissors(3)),
            _ => Err("Invalid handshape provided."),
        }
    }
}

impl HandShape {
    fn battle(&self, opponant_handshape: HandShape) -> RoundOutcome {
        match *self {
            HandShape::Rock(shape_value) => match opponant_handshape {
                HandShape::Rock(_) => RoundOutcome::Draw(shape_value + 3),
                HandShape::Paper(_) => RoundOutcome::Lost(shape_value),
                HandShape::Scissors(_) => RoundOutcome::Win(shape_value + 6),
            },
            HandShape::Paper(shape_value) => match opponant_handshape {
                HandShape::Rock(_) => RoundOutcome::Win(shape_value + 6),
                HandShape::Paper(_) => RoundOutcome::Draw(shape_value + 3),
                HandShape::Scissors(_) => RoundOutcome::Lost(shape_value),
            },
            HandShape::Scissors(shape_value) => match opponant_handshape {
                HandShape::Rock(_) => RoundOutcome::Lost(shape_value),
                HandShape::Paper(_) => RoundOutcome::Win(shape_value + 6),
                HandShape::Scissors(_) => RoundOutcome::Draw(shape_value + 3),
            },
        }
    }
}

fn main() -> Result<(), &'static str> {
    let my_handshape = HandShape::try_from('Z')?;
    let opponant_handshape = HandShape::try_from('C')?;
    let outcome = my_handshape.battle(opponant_handshape);
    println!("{:?}", outcome);

    Ok(())
}
