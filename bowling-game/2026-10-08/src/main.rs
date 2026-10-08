fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Frame {
    first: i8,
    second: i8,
}

impl Frame {
    fn new(first: i8, second: Option<i8>) -> Frame {
        let mut f: i8 = first;

        if f > 10 {
            f = 10;
        }

        match second {
            Some(sec) => {
                if f + sec > 10 {
                    Frame {
                        first: f,
                        second: 10 - f,
                    }
                } else {
                    Frame {
                        first: f,
                        second: sec,
                    }
                }
            }
            None => Frame {
                first: f,
                second: 0,
            },
        }
    }
}

#[test]
fn new_frame_with_0() {
    assert_eq!(
        Frame::new(0, None),
        Frame {
            first: 0,
            second: 0
        }
    );
}

#[test]
fn new_frame_with_both_0() {
    assert_eq!(
        Frame::new(0, Some(0)),
        Frame {
            first: 0,
            second: 0
        }
    );
}

#[test]
fn new_frame_with_1() {
    assert_eq!(
        Frame::new(1, None),
        Frame {
            first: 1,
            second: 0
        }
    );
}

#[test]
fn new_frame_with_both_1() {
    assert_eq!(
        Frame::new(1, Some(1)),
        Frame {
            first: 1,
            second: 1
        }
    );
}

#[test]
fn new_frame_with_10() {
    assert_eq!(
        Frame::new(10, None),
        Frame {
            first: 10,
            second: 0
        }
    );
}

#[test]
fn new_frame_with_both_10() {
    assert_eq!(
        Frame::new(10, Some(10)),
        Frame {
            first: 10,
            second: 0
        }
    );
}

#[test]
fn new_frame_with_11() {
    assert_eq!(
        Frame::new(11, None),
        Frame {
            first: 10,
            second: 0
        }
    );
}

#[test]
fn new_frame_with_adding_to_10() {
    assert_eq!(
        Frame::new(8, Some(2)),
        Frame {
            first: 8,
            second: 2
        }
    );
}

#[test]
fn new_frame_with_adding_to_more_than_10() {
    assert_eq!(
        Frame::new(8, Some(3)),
        Frame {
            first: 8,
            second: 2
        }
    );
}
