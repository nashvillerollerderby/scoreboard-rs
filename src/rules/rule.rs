use crate::rules::rule_definition::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rule {
    NumberPeriods,
    PeriodDuration,
    PeriodDirection,
    PeriodEndBetweenJams,
    JamsPerPeriod,

    JamNumberPerPeriod,
    JamDuration,
    JamDirection,
    SuddenScoring,
    SuddenScoringMinPointsDifference,
    SuddenScoringMaxTrailingPoints,
    SuddenScoringJamDuration,
    InjuryContinuation,

    LineupDuration,
    OvertimeLineupDuration,
    LineupDirection,
    LineupStopsPeriodClock,

    TtoDuration,
    TimeoutDirection,
    StopPcOnTo,
    StopPcOnOto,
    StopPcOnTto,
    StopPcOnOr,
    StopPcAfterToDuration,
    ExtraJamAfterOto,
    ToJam,
    NoToClockStop,

    IntermissionDurations,
    IntermissionDirection,

    NumberTimeouts,
    TimeoutsPerPeriod,
    NumberReviews,
    ReviewsPerPeriod,
    NumberRetains,
    RdclPerHalfRules,

    WftdaLateScoreRule,
    PointsOnInitial,
    EnforceTimeToOr,

    PenaltiesFile,
    FoLimit,
    PenaltyDuration,
    PenaltyPointsDeduction,
}

impl Rule {
    pub fn index(self) -> usize {
        self as usize
    }

    pub fn definition(self) -> &'static RuleDefinition {
        match self {
            Rule::NumberPeriods => &RuleDefinition {
                name: "Period.Number",
                description: "Number of periods",
                default_value: "2",
                rule_type: RuleType::Integer,
            },
            Rule::PeriodDuration => &RuleDefinition {
                name: "Period.Duration",
                description: "Duration of a period",
                default_value: "30:00",
                rule_type: RuleType::Time,
            },
            Rule::PeriodDirection => &RuleDefinition {
                name: "Period.ClockDirection",
                description: "Which way should the period clock count?",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Count Down",
                    false_value: "Count Up",
                },
            },
            Rule::PeriodEndBetweenJams => &RuleDefinition {
                name: "Period.EndBetweenJams",
                description: "When can a period end?",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Anytime outside a jam",
                    false_value: "Only on jam end",
                },
            },
            Rule::JamsPerPeriod => &RuleDefinition {
                name: "Period.JamsPer",
                description: "Number of jams per period. 0 for duration-based periods",
                default_value: "0",
                rule_type: RuleType::Integer,
            },

            Rule::JamNumberPerPeriod => &RuleDefinition {
                name: "Jam.ResetNumberEachPeriod",
                description: "How to handle Jam Numbers",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Reset each period",
                    false_value: "Continue counting",
                },
            },
            Rule::JamDuration => &RuleDefinition {
                name: "Jam.Duration",
                description: "Maximum duration of a jam",
                default_value: "2:00",
                rule_type: RuleType::Time,
            },
            Rule::JamDirection => &RuleDefinition {
                name: "Jam.ClockDirection",
                description: "Which way should the jam clock count?",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Count Down",
                    false_value: "Count Up",
                },
            },
            Rule::SuddenScoring => &RuleDefinition {
                name: "Jam.SuddenScoring",
                description: "Use JRDA Sudden Scoring rule?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },
            Rule::SuddenScoringMinPointsDifference => &RuleDefinition {
                name: "Jam.SuddenScoringMinPointsDifference",
                description: "Minimal score difference at halftime at which sudden scoring is activated.",
                default_value: "150",
                rule_type: RuleType::Integer,
            },
            Rule::SuddenScoringMaxTrailingPoints => &RuleDefinition {
                name: "Jam.SuddenScoringMaxTrailingPoints",
                description: "Maximum points the trailing team may have at halftime to trigger sudden scoring.",
                default_value: "25",
                rule_type: RuleType::Integer,
            },
            Rule::SuddenScoringJamDuration => &RuleDefinition {
                name: "Jam.SuddenScoringDuration",
                description: "Maximum duration of a jam when sudden scoring is in effect",
                default_value: "1:00",
                rule_type: RuleType::Time,
            },
            Rule::InjuryContinuation => &RuleDefinition {
                name: "Jam.InjuryContinuation",
                description: "A jam called for injury can be followed by a continuation jam.",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },

            Rule::LineupDuration => &RuleDefinition {
                name: "Lineup.Duration",
                description: "Duration of lineup",
                default_value: "0:30",
                rule_type: RuleType::Time,
            },
            Rule::OvertimeLineupDuration => &RuleDefinition {
                name: "Lineup.OvertimeDuration",
                description: "Duration of lineup before an overtime jam",
                default_value: "1:00",
                rule_type: RuleType::Time,
            },
            Rule::LineupDirection => &RuleDefinition {
                name: "Lineup.ClockDirection",
                description: "Which way should the lineup clock count?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Count Down",
                    false_value: "Count Up",
                },
            },
            Rule::LineupStopsPeriodClock => &RuleDefinition {
                name: "Lineup.StopsPeriodClock",
                description: "Stop the period clock during lineup",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },

            Rule::TtoDuration => &RuleDefinition {
                name: "Timeout.TeamTODuration",
                description: "Duration of a team timeout",
                default_value: "1:00",
                rule_type: RuleType::Time,
            },
            Rule::TimeoutDirection => &RuleDefinition {
                name: "Timeout.ClockDirection",
                description: "Which way should the timeout clock count?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Count Down",
                    false_value: "Count Up",
                },
            },
            Rule::StopPcOnTo => &RuleDefinition {
                name: "Timeout.StopPeriodClockAlways",
                description: "Stop the period clock on every timeout? If false, the options below control the \
                     behaviour per type of timeout.",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },
            Rule::StopPcOnOto => &RuleDefinition {
                name: "Timeout.StopPeriodClockOnOTO",
                description: "Stop the period clock on official timeouts?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },
            Rule::StopPcOnTto => &RuleDefinition {
                name: "Timeout.StopPeriodClockOnTTO",
                description: "Stop the period clock on team timeouts?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },
            Rule::StopPcOnOr => &RuleDefinition {
                name: "Timeout.StopPeriodClockOnOR",
                description: "Stop the period clock on official reviews?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },
            Rule::StopPcAfterToDuration => &RuleDefinition {
                name: "Timeout.StopPeriodClockAfterTODuration",
                description: "Stop the period clock, if a timeout lasts longer than this time. Set to a high value to disable.",
                default_value: "60:00",
                rule_type: RuleType::Time,
            },
            Rule::ExtraJamAfterOto => &RuleDefinition {
                name: "Timeout.ExtraJamAfterOTO",
                description: "Can an OTO cause an extra Jam to be played when there wouldn't be one otherwise?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },
            Rule::ToJam => &RuleDefinition {
                name: "Timeout.JamDuring",
                description: "Allow a jam to happen with stopped period clock?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },
            Rule::NoToClockStop => &RuleDefinition {
                name: "Timeout.NoClockStop",
                description: "Should the timeout clock continue to run after the end of a timeout?",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "True",
                    false_value: "False",
                },
            },

            Rule::IntermissionDurations => &RuleDefinition {
                name: "Intermission.Durations",
                description: "List of the duration of intermissions as they appear in the game, separated by commas.",
                default_value: "15:00,60:00",
                rule_type: RuleType::String,
            },
            Rule::IntermissionDirection => &RuleDefinition {
                name: "Intermission.ClockDirection",
                description: "Which way should the intermission clock count?",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Count Down",
                    false_value: "Count Up",
                },
            },

            Rule::NumberTimeouts => &RuleDefinition {
                name: "Team.Timeouts",
                description: "How many timeouts each team is granted per game or period",
                default_value: "3",
                rule_type: RuleType::Integer,
            },
            Rule::TimeoutsPerPeriod => &RuleDefinition {
                name: "Team.TimeoutsPer",
                description: "Are timeouts granted per period or per game?",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Period",
                    false_value: "Game",
                },
            },
            Rule::NumberReviews => &RuleDefinition {
                name: "Team.OfficialReviews",
                description: "How many official reviews each team is granted per game or period",
                default_value: "1",
                rule_type: RuleType::Integer,
            },
            Rule::ReviewsPerPeriod => &RuleDefinition {
                name: "Team.OfficialReviewsPer",
                description: "Are official reviews granted per period or per game?",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Period",
                    false_value: "Game",
                },
            },
            Rule::NumberRetains => &RuleDefinition {
                name: "Team.MaxRetains",
                description: "How many times per game or period a team can retain an official review",
                default_value: "1",
                rule_type: RuleType::Integer,
            },
            Rule::RdclPerHalfRules => &RuleDefinition {
                name: "Team.RDCLPerHalfRules",
                description: "Restrict one TTO to the first two periods and one to the rest of the game. \
                     Stretch per period ORs to first two resp. all other periods.",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },

            Rule::WftdaLateScoreRule => &RuleDefinition {
                name: "Score.WftdaLateChangeRule",
                description: "Score changes after the end of the following jam don't affect the game score. With less \
                     than 2 minutes left in the game this applies to changes after the next jam starts.",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },
            Rule::PointsOnInitial => &RuleDefinition {
                name: "Score.PointsOnInitial",
                description: "Don't advance out of initial trip when points are added",
                default_value: "false",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },
            Rule::EnforceTimeToOr => &RuleDefinition {
                name: "Score.EnforceTimeToOr",
                description: "If either team has an OR left, disallow setting final score earlier than 30s \
                     after the end of the period or earlier than 10s after the last score change",
                default_value: "true",
                rule_type: RuleType::Boolean {
                    true_value: "Enabled",
                    false_value: "Disabled",
                },
            },

            Rule::PenaltiesFile => &RuleDefinition {
                name: "Penalties.DefinitionFile",
                description: "File that contains the penalty code definitions to be used",
                default_value: "/config/penalties/wftda2018.json",
                rule_type: RuleType::String,
            },
            Rule::FoLimit => &RuleDefinition {
                name: "Penalties.NumberToFoulout",
                description: "After how many penalties a skater has fouled out of the game. Note that the software \
                     currently does not support more than 9 penalties per skater.",
                default_value: "7",
                rule_type: RuleType::Integer,
            },
            Rule::PenaltyDuration => &RuleDefinition {
                name: "Penalties.Duration",
                description: "How long does a penalty last.",
                default_value: "0:30",
                rule_type: RuleType::Time,
            },
            Rule::PenaltyPointsDeduction => &RuleDefinition {
                name: "Penalties.PointsDeduction",
                description: "How many points to deduct from a team's score for each penalty",
                default_value: "0",
                rule_type: RuleType::Integer,
            },
        }
    }
}
