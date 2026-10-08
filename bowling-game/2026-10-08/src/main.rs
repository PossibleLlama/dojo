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

fn get_score(bowl: [Frame; 12]) -> i16 {
    let mut score: i16 = 0;
    for i in 0..10 {
        let frame = bowl[i];
        if frame.first + frame.second < 10 {
            score += (frame.first + frame.second) as i16;
        } else if frame.first == 10 {
            // strike
            score += 10 + (bowl[i + 1].first + bowl[i + 2].first) as i16
        } else {
            // spare
            score += 10 + bowl[i + 1].first as i16
        }
    }
    score
}

#[test]
fn get_score_with_misses() {
    assert_eq!(
        get_score([
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, Some(0)),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        0
    );
}

#[test]
fn get_score_with_one_point_each_frame() {
    assert_eq!(
        get_score([
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        10
    );
}

#[test]
fn get_score_with_one_point_each_frame_ignores_invalid_frames() {
    assert_eq!(
        get_score([
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(1, None)
        ]),
        10
    );
}

#[test]
fn get_score_with_one_point_in_second_half() {
    assert_eq!(
        get_score([
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, Some(1)),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        10
    );
}

#[test]
fn get_score_with_two_points_each_frame() {
    assert_eq!(
        get_score([
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(1, Some(1)),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        20
    );
}

#[test]
fn get_score_with_miss_then_spares() {
    assert_eq!(
        get_score([
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, Some(10)),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        100
    );
}

#[test]
fn get_score_with_5_then_spares() {
    assert_eq!(
        get_score([
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, Some(5)),
            Frame::new(5, None),
            Frame::new(0, None)
        ]),
        150
    );
}

#[test]
fn get_score_with_strike_then_miss() {
    assert_eq!(
        get_score([
            Frame::new(10, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(10, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(10, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        30
    );
}

#[test]
fn get_score_with_strike_plus_next_two_then_miss() {
    assert_eq!(
        get_score([
            Frame::new(10, None),
            Frame::new(1, None),
            Frame::new(1, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        14
    );
}

#[test]
fn get_score_with_perfect_score() {
    assert_eq!(
        get_score([
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None),
            Frame::new(10, None)
        ]),
        300
    );
}

#[test]
fn get_score_with_strike_ignores_second() {
    assert_eq!(
        get_score([
            Frame::new(10, Some(1)),
            Frame {
                first: 10,
                second: 1
            },
            Frame {
                first: 10,
                second: 1
            },
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None),
            Frame::new(0, None)
        ]),
        60
    );
}
