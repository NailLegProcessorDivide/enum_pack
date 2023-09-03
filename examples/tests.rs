enum_pack::flattenum! {
    Direction4 = {
        North,
        East,
        South,
        West
    }

    Direction5 = Direction4 + {
        Up,
    }

    Direction6 = Direction5 + {
        Down,
    }

    Block = {
        Torch(Direction5),
        Piston(Direction6),
        Dirt,
        DiamondOre,
    }
}

fn main() {
    println!("runs {}", COUNT);
    let a = Direction4::North;
    let b = Direction5::Up;
    println!("{:?} {:?}", a, b);
    println!("{:?}", Block::Torch(Direction5::Up));
    println!("{:?}", Block::Piston(Direction6::Down));
    println!("{:?}", Block::Dirt);
}

#[cfg(test)]
mod tests {
    use crate::Direction4;
    use crate::Direction6;
    #[test]
    fn create_enums() {
        let a = Direction4::North;
        let b = Direction4::West;
        let c = Direction4::North;
        assert!(format!("{:?}", a) == "North");
        assert!(a != b);
        assert!(b != c);
        assert!(a == c);

        let a = Direction6::North;
        let b = Direction6::East;
        let c = Direction6::Up;
        assert!(format!("{:?}", b) == "East");
        assert!(a != b);
        assert!(b != c);
        assert!(a != c);
    }
}
