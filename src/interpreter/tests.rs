use super::*;
use crate::parser::{CollyParser, Rule};

#[ignore]
#[test]
fn interpret_pattern_complex() {
    let mut context = Context::default();
    // let result: ast::Pattern = tests::parse_source_for_rule("|01 2|", Rule::Pattern).unwrap();

    let result: ast::Pattern = CollyParser::parse_source_for_rule(
        "| 01*:23 01[0 1 23]* (012 34)* 01(23 4)5*1: 1: |",
        Rule::Pattern,
    )
    .unwrap();
    // let pattern: types::Pattern = result.interpret(&mut context).unwrap();
    // let expected = ...;
    // assert_eq!(expected, result);
}

#[test]
fn interpret_event_group_methods() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("a*._:", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(10)),
                duration: 3.0,
                octave: None,
                beat_position: Default::default(),
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Tie,
                duration: 0.5,
                octave: None,
                beat_position: 3.0,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_event_group_alterations() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("++a-+--b", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree {
                    value: 10,
                    alteration: 2,
                }),
                duration: 1.0,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree {
                    value: 11,
                    alteration: -2,
                }),
                duration: 1.0,
                octave: None,
                beat_position: 1.0,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_event_group_octaves() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("OOaooob", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(10)),
                duration: 1.0,
                octave: Some(Octave::with_octave(7)),
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(11)),
                duration: 1.0,
                octave: Some(Octave::with_octave(4)),
                beat_position: 1.0,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_parenthesised_group_single() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("(0)", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 1.0,
            octave: None,
            beat_position: 0.0,
            beat: 0,
        },],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_parenthesised_group_simple() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("(00)", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.5,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.5,
                octave: None,
                beat_position: 0.5,
                beat: 0,
            }
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_parenthesised_group() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("(ab 0)", Rule::Event).unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(10)),
                duration: 0.25,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(11)),
                duration: 0.25,
                octave: None,
                beat_position: 0.25,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.5,
                octave: None,
                beat_position: 0.5,
                beat: 0,
            },
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_parenthesised_group_recursive() {
    use types::*;

    let mut context = Context::default();
    let event: ast::Event =
        CollyParser::parse_source_for_rule("(0(00)00 12)", Rule::Event)
            .unwrap();
    let event_interpreter = EventInterpreter {
        event,
        beat: 0,
        octave: Default::default(),
        beat_position: Default::default(),
    };

    assert_eq!(
        vec![
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.125,
                octave: None,
                beat_position: 0.0,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.0625,
                octave: None,
                beat_position: 0.125,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.0625,
                octave: None,
                beat_position: 0.1875,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.125,
                octave: None,
                beat_position: 0.25,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(0)),
                duration: 0.125,
                octave: None,
                beat_position: 0.375,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(1)),
                duration: 0.25,
                octave: None,
                beat_position: 0.5,
                beat: 0,
            },
            IntermediateEvent {
                value: Audible::Degree(Degree::from(2)),
                duration: 0.25,
                octave: None,
                beat_position: 0.75,
                beat: 0,
            },
        ],
        event_interpreter.interpret(&mut context).unwrap()
    );
}

#[test]
fn interpret_pattern_inner_simple() {
    use types::*;

    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| 01 2 |", Rule::Pattern).unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);
    let result = inner_interpreter.interpret(&mut context).unwrap();
    let expected = vec![
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            octave: None,
            duration: 0.5,
            beat_position: 0.0,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            octave: None,
            duration: 0.5,
            beat_position: 0.5,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(2)),
            octave: None,
            duration: 1.0,
            beat_position: 0.0,
            beat: 1,
        },
    ];

    assert_eq!(expected, result);
}

#[test]
fn interpret_pattern_inner_methods() {
    use types::*;

    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| 0:1. 2* |", Rule::Pattern)
            .unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);
    let result = inner_interpreter.interpret(&mut context).unwrap();
    let expected = vec![
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            octave: None,
            duration: 0.25,
            beat_position: 0.0,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            octave: None,
            duration: 0.75,
            beat_position: 0.25,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(2)),
            octave: None,
            duration: 1.0,
            beat_position: 0.0,
            beat: 1,
        },
    ];

    assert_eq!(expected, result);
}

#[test]
fn interpret_pattern_inner_parenthesised() {
    use types::*;

    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| 0(11 1)00 |", Rule::Pattern)
            .unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);
    let result = inner_interpreter.interpret(&mut context).unwrap();
    let expected = vec![
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 0.25,
            octave: None,
            beat_position: 0.0,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            duration: 0.0625,
            octave: None,
            beat_position: 0.25,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            duration: 0.0625,
            octave: None,
            beat_position: 0.3125,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(1)),
            duration: 0.125,
            octave: None,
            beat_position: 0.375,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 0.25,
            octave: None,
            beat_position: 0.5,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 0.25,
            octave: None,
            beat_position: 0.75,
            beat: 0,
        },
    ];

    assert_eq!(expected, result);
}

#[test]
fn interpret_pattern_inner_ties() {
    use types::*;

    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| 0_0_ _ 0 |", Rule::Pattern)
            .unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);
    let result = inner_interpreter.interpret(&mut context).unwrap();
    let expected = vec![
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 0.5,
            octave: None,
            beat_position: 0.0,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 1.5,
            octave: None,
            beat_position: 0.5,
            beat: 0,
        },
        IntermediateEvent {
            value: Audible::Degree(Degree::from(0)),
            duration: 1.0,
            octave: None,
            beat_position: 0.0,
            beat: 2,
        },
    ];

    assert_eq!(expected, result);
}

#[test]
fn interpret_pattern_inner_lonely_tie() {
    let mut context = Context::default();
    let pattern: ast::Pattern =
        CollyParser::parse_source_for_rule("| _ 0 |", Rule::Pattern).unwrap();
    let inner_interpreter = PatternInnerInterpreter::new(pattern.0);

    assert_eq!(
        Err(InterpreterError::LonelyTie),
        inner_interpreter.interpret(&mut context)
    );
}
