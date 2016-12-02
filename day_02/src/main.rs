//! # Advent of Code 2016 - Day 2
//!
//! My solution to part 1 of Day 2 of the Advent of Code
//! 2016 challenge, "Bathroom Security." Full details of the
//! challenge can be found on [the challenge page][page].
//!
//! [page]: http://adventofcode.com/2016/day/2

pub trait Numpad: Sized + Copy {
    
    /// Returns the default starting position
    fn new() -> Self;

    /// Returns the numerical value of the button
    fn to_int(&self) -> u8;

    /// Consumes this button and returns the button that is
    /// above it.
    fn up(self) -> Self;

    /// Consumes this button and returns the button that is
    /// below it.
    fn down(self) -> Self;

    /// Consumes this button and returns the button that is
    /// to the left of it.
    fn left(self) -> Self;

    /// Consumes this button and returns the button that is
    /// to the right of it.
    fn right(self) -> Self;

    /// Interperates the first of the input str as a series of
    /// numpad instructions and returns the corresponding final button.
    fn find_button(self, instr_line: &str) -> Self {
                instr_line.chars()
                    .fold(self,
                          |button, direction| {
                              match direction {
                                  'L' => button.left(),
                                  'R' => button.right(),
                                  'U' => button.up(),
                                  'D' => button.down(),
                                  _   => button
                              }
                          })
            }
}

macro_rules! numpad_impl {
    (
        name => $name:ident ; start => $default:ident ;
        $(
            $button:ident => ($val:expr, $up:ident, $down:ident, $left:ident, $right:ident)
        ),+
    ) => {

        #[derive(Debug, Copy, Clone)]
        pub enum $name {
            $($button),+
        }

        impl Numpad for $name {
            fn new() -> Self {
                $name::$default
            }

            fn to_int(&self) -> u8 {
                match *self {
                    $($name::$button => $val),+
                }
            }

            fn up(self) -> Self {
                match self {
                    $($name::$button => $name::$up),+
                }
            }

            fn down(self) -> Self {
                match self {
                    $($name::$button => $name::$down),+
                }
            }

            fn left(self) -> Self {
                match self {
                    $($name::$button => $name::$left),+
                }
            }

            fn right(self) -> Self {
                match self {
                    $($name::$button => $name::$right),+
                }
            }
        }
        
    }
}

// 1 2 3
// 4 5 6
// 7 8 9

numpad_impl! {
    name => NumpadButton ; start => N5 ;
    // => (val, U , D , L , R )
    N1 => (0x1, N1, N4, N1, N2),
    N2 => (0x2, N2, N5, N1, N3),
    N3 => (0x3, N3, N6, N2, N3),
    N4 => (0x4, N1, N7, N4, N5),
    N5 => (0x5, N2, N8, N4, N6),
    N6 => (0x6, N3, N9, N5, N6),
    N7 => (0x7, N4, N7, N7, N8),
    N8 => (0x8, N5, N8, N7, N9),
    N9 => (0x9, N6, N9, N8, N9)
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
numpad_impl! {
    name => ActualButton; start => N5;
    // => (val, U , D , L , R )
    N1 => (0x1, N1, N3, N1, N1),
    N2 => (0x2, N2, N6, N2, N3),
    N3 => (0x3, N1, N7, N2, N4),
    N4 => (0x4, N4, N8, N3, N4),
    N5 => (0x5, N5, N5, N5, N6),
    N6 => (0x6, N2, Na, N5, N7),
    N7 => (0x7, N3, Nb, N6, N8),
    N8 => (0x8, N4, Nc, N7, N9),
    N9 => (0x9, N9, N9, N8, N9),
    Na => (0xa, N6, Na, Na, Nb),
    Nb => (0xb, N7, Nd, Na, Nc),
    Nc => (0xc, N8, Nc, Nb, Nc),
    Nd => (0xd, Nb, Nd, Nd, Nd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
        let instructions = "ULL
RRDDD
LURDL
UUUUD";

        assert_eq!(find_door_code::<NumpadButton>(instructions),
                   vec![1, 9, 8, 5]);
    }
    
}
    
pub fn find_door_code<T: Numpad>(instructions: &str) -> Vec<u8> {
    let mut buttons = Vec::with_capacity(4);

    for (i, instr_line) in instructions.lines().enumerate() {
        let old_button = if i == 0 { T::new() } else { buttons[i-1] };
        let new_button = old_button.find_button(instr_line);

        buttons.push(new_button);
    }

    buttons.iter().map(T::to_int).collect()
}

fn main() {
    let challenge_input = "DLRURUDLULRDRUDDRLUUUDLDLDLRLRRDRRRLLLLLDDRRRDRRDRRRLRRURLRDUULRLRRDDLULRLLDUDLULURRLRLDUDLURURLDRDDULDRDRDLDLLULULLDDLRRUDULLUULRRLLLURDRLDDLDDLDRLRRLLRURRUURRRRLUDLRDDDDRDULRLLDDUURDUDRLUDULLUDLUDURRDRDUUUUDDUDLLLRLUULRUURDLRLLRRLRLLDLLRLLRRRURLRRLURRLDLLLUUDURUDDLLUURRDRDRRDLLDDLLRDRDRRLURLDLDRDLURLDULDRURRRUDLLULDUDRURULDUDLULULRRRUDLUURRDURRURRLRRLLRDDUUUUUDUULDRLDLLRRUDRRDULLLDUDDUDUURLRDLULUUDLDRDUUUDDDUDLDURRULUULUUULDRUDDLLLDLULLRLRLUDULLDLLRLDLDDDUUDURDDDLURDRRDDLDRLLRLRR
RLDUDURDRLLLLDDRRRURLLLRUUDDLRDRDDDUDLLUDDLRDURLDRDLLDRULDDRLDDDRLDRDDDRLLULDURRRLULDRLRDRDURURRDUDRURLDRLURDRLUULLULLDLUDUDRDRDDLDDDDRDURDLUDRDRURUDDLLLRLDDRURLLUDULULDDLLLDLUDLDULUUDLRLURLDRLURURRDUUDLRDDDDDRLDULUDLDDURDLURLUURDLURLDRURRLDLLRRUDRUULLRLDUUDURRLDURRLRUULDDLDLDUUDDRLDLLRRRUURLLUURURRURRLLLUDLDRRDLUULULUDDULLUDRLDDRURDRDUDULUDRLRRRUULLDRDRLULLLDURURURLURDLRRLLLDRLDUDLLLLDUUURULDDLDLLRRUDDDURULRLLUDLRDLUUDDRDDLLLRLUURLDLRUURDURDDDLLLLLULRRRURRDLUDLUURRDRLRUDUUUURRURLRDRRLRDRDULLDRDRLDURDDUURLRUDDDDDLRLLRUDDDDDURURRLDRRUUUDLURUUDRRDLLULDRRLRRRLUUUD
RDRURLLUUDURURDUUULLRDRLRRLRUDDUDRURLLDLUUDLRLLDDURRURLUDUDDURLURLRRURLLURRUDRUDLDRLLURLRUUURRUDDDURRRLULLLLURDLRLLDDRLDRLLRRDLURDLRDLDUDRUULLDUUUDLURRLLRUDDDUUURLURUUDRLRULUURLLRLUDDLLDURULLLDURDLULDLDDUDULUDDULLRDRURDRRLLDLDDDDRUDLDRRLLLRLLLRRULDLRLRLRLLDLRDRDLLUDRDRULDUURRDDDRLLRLDLDRDUDRULUDRDLDLDDLLRULURLLURDLRRDUDLULLDLULLUDRRDDRLRURRLDUDLRRUUDLDRLRLDRLRRDURRDRRDDULURUUDDUUULRLDRLLDURRDLUULLUDRDDDLRUDLRULLDDDLURLURLRDRLLURRRUDLRRLURDUUDRLRUUDUULLRUUUDUUDDUURULDLDLURLRURLRUDLULLULRULDRDRLLLRRDLU
RRRRDRLUUULLLRLDDLULRUUURRDRDRURRUURUDUULRULULRDRLRRLURDRRRULUUULRRUUULULRDDLLUURRLLDUDRLRRLDDLDLLDURLLUDLDDRRURLDLULRDUULDRLRDLLDLRULLRULLUDUDUDDUULDLUUDDLUDDUULLLLLURRDRULURDUUUDULRUDLLRUUULLUULLLRUUDDRRLRDUDDRULRDLDLLLLRLDDRRRULULLLDLRLURRDULRDRDUDDRLRLDRRDLRRRLLDLLDULLUDDUDDRULLLUDDRLLRRRLDRRURUUURRDLDLURRDLURULULRDUURLLULDULDUDLLULDDUURRRLDURDLUDURLDDRDUDDLLUULDRRLDLLUDRDURLLDRLDDUDURDLUUUUURRUULULLURLDUUULLRURLLLUURDULLUULDRULLUULRDRUULLRUDLDDLRLURRUUDRLRRRULRUUULRULRRLDLUDRRLL
ULRLDLLURDRRUULRDUDDURDDDLRRRURLDRUDDLUDDDLLLRDLRLLRRUUDRRDRUULLLULULUUDRRRDRDRUUUUULRURUULULLULDULURRLURUDRDRUDRURURUDLDURUDUDDDRLRLLLLURULUDLRLDDLRUDDUUDURUULRLLLDDLLLLRRRDDLRLUDDUULRRLLRDUDLLDLRRUUULRLRDLRDUDLLLDLRULDRURDLLULLLRRRURDLLUURUDDURLDUUDLLDDRUUDULDRDRDRDDUDURLRRRRUDURLRRUDUDUURDRDULRLRLLRLUDLURUDRUDLULLULRLLULRUDDURUURDLRUULDURDRRRLLLLLUUUULUULDLDULLRURLUDLDRLRLRLRDLDRUDULDDRRDURDDULRULDRLRULDRLDLLUDLDRLRLRUDRDDR";
    
    let door_code = find_door_code::<NumpadButton>(challenge_input);
    let actual_door_code = find_door_code::<ActualButton>(challenge_input);
    print!("Idealised Bathroom code = ");
    for code in door_code {
        print!("{:x}", code);
    }
    println!("");
        
    print!("Actual bathroom code = ");
    for code in actual_door_code {
        print!("{:X}", code);
    }
    println!("");
}
