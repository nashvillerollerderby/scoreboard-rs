struct ScoreBoard {
    current_period_number: usize,
    current_period: String, // period UUID
    in_period: bool,
    upcoming_jam: String, // jam UUID
    upcoming_jam_number: usize,
    in_jam: bool,
    in_overtime: bool,
    current_timeout: String, // timeout UUID
    timeout_owner: TimeoutOwner,
    official_review: bool,
    no_more_jam: bool,
    official_score: bool,
    clock: Clock,
    penalty_codes: HashMap<String, String>,
    rulesets: Vec<Ruleset>,
    media: Media,
    clients: Clients,
    current_game: Game,
}

struct Ruleset {}

struct PreparedTeam {
    id: String,
    name: String,
    full_name: String,
    league_name: String,
    team_name: String,
    uniform_color: String,
    alternate_name: String,
    team_cue: String,
    color: String,
}

// game related models

struct Game {
    name: String,
    name_format: String,
    state: State,
    current_period_number: usize,
    current_period: usize,
    upcoming_jam: Jam,
    upcoming_jam_number: usize,
    in_period: bool,
    in_overtime: bool,
    in_sudden_scoring: bool, // JRDA rules
    injury_continuation_upcoming: bool,
    inhibit_final_score: bool,
    official_score: bool,
    abort_reason: String,
    current_timeout: Timeout,
    timeout_owner: TimeoutOwner,
    official_review: bool,
    or_is_to: bool,
    no_more_jam: bool,
    ruleset: Ruleset,
    ruleset_name: String,
    head_nso: Official,
    head_ref: Official,
    suspensions_served: String,
    filename: String,
    last_file_update: String,
    update_in_progress: String,
    statsbook_exists: bool,
    json_exists: bool,
    clock_during_final_score: bool,
    export_blocked_by: bool,
    fiiiive_seconds: bool,
    auto_five: bool,
    five_indicator: String,
    official_crew: OfficialsCrew,
}

struct Command {}

enum State {
    Prepared,
    Running,
    Finished,
}

// todo: chrono
struct Clock {
    id: ClockID,
    name: String,
    number: usize,
    time: u64,
    inverted_time: u64,
    maximum_time: u64,
    direction: bool,
    running: bool,
}

struct Team {
    skaters: Vec<Skater>,
    position: Vec<Position>,
    box_trips: Vec<BoxTrip>,
    score_adjustment: Vec<ScoreAdjustment>,
}

struct ScoreAdjustment {
    id: String,
    read_only: bool,
    amount: usize,
    jam_recorded: String, // uuid of the Jam
    period_number_recorded: usize,
    recorded_during_jam: bool,
    last_two_minutes: bool,
    open: bool,
    applied_to: bool,
}

struct Skater {
    id: String,
    read_only: bool,
    name: Name,
    roster_number: usize,
    penalty_box: bool,
    current_box_symbols: String,
    flags: String,
    role: String,
    base_role: String,
    current_penalties: String,
    has_unserved: bool,
    position: String,
    penalty_count: usize,
    extra_penalty_time: usize,
    penalty_details: String,
}

struct Jam {
    period_number: usize,
    number: usize,
}

struct Period {}

struct Penalty {}

struct BoxTrip {
    is_current: bool,
    current_fielding: Fielding,
    start_fielding: Fielding,
    start_after_s_p: bool,
    end_fielding: Fielding,

    end_jam_number: usize,
    end_between_jams: bool,
    end_after_s_p: bool,
    // todo chrono
    walltime_start: u64,
    walltime_end: u64,
    jam_clock_start: u64,
    jam_clock_end: u64,
    duration: u64,
    current_skater: Skater,
    roster_number: String,
    penalty_codes: String,
    total_penalties: usize,
    timing_stopped: String,
    time: u64,
    shortened: usize,
    penalty_details: String,
    jammer: bool,
}

struct Expulsion {
    // todo figure this out
}

struct Official {
    id: String,
    read_only: bool,
    role: OfficialRole,
    league: String,
    cert: String,
    team: String,
}

struct PreparedOfficial {
    id: String,
    read_only: bool,
    name: String,
    league: String,
    cert: String,
    full_info: String,
}

enum OfficialRole {
    HeadRef,
    JammerRef,
    InsidePackRef,
    OutsidePackRef,
    RefAlt,
    HeadNso,
    ScoreboardOperator,
    Scorekeeper,
    PenaltyLineupTracker,
    PenaltyWrangler,
    JamTimer,
    PenaltyTracker,
    PenaltyBoxManager,
    PenaltyBoxTimer,
    LineupTracker,
    NsoAlt,
    InsideWhiteboardOperator,
}

enum SkaterRole {
    Jammer,
    Pivot,
    Blocker,
    Bench,
    NotInGame,
    Ineligible,
}

// device related models

struct Client {
    id: String,
    device: Device,
}

struct Device {
    id: String,
    read_only: bool,
    name: String,
    remote_addr: String,
    platform: String,
    create: usize,
    wrote: usize,
    accessed: usize,
    maywrite: bool,
    num_clients: usize,
}
