use std::{collections::{HashMap, HashSet}, sync::Arc};

/// The Object Model
///
/// Every structure and enum with every field that can be read or set.
/// Derived from rollerderby/scoreboard's core/interfaces and
/// https://github.com/rollerderby/scoreboard/wiki/WebSocket-Channels .

struct ValWithId {
    id: String,
    val: String,
}

struct ScoreBoard {
    id: String,
    read_only: bool,
    blank_statsbook_found: String,
    imports_in_progress: usize,
    version: HashSet<ValWithId>,
    settings: HashSet<Settings>,
    media: HashSet<Media>,
    clients: HashSet<Clients>,
    rulesets: HashSet<Rulesets>,
    game: HashSet<Game>,
    prepared_team: HashSet<Team>, // PreparedX means "X when it's on disk",
    prepared_official: HashSet<Official>, // PreparedX means "X when it's on disk"
    officials_crew: HashSet<OfficialsCrew>,
    current_game: HashSet<Game>, // why is this a Child<Game> instead of Value<Game>
}

struct ScoringTrip {
    id: String,
    read_only: bool,
    number: usize,
    score: usize,
    after_s_p: bool,
    current: bool,
    duration: u64, // todo: chrono
    jam_clock_start: u64, // todo: chrono
    jam_clock_end: u64, // todo: chrono
    annotation: String,
}

struct Settings {
    id: String,
    read_only: bool,
    setting: HashSet<String, String>,
}

struct Rulesets {
    id: String,
    read_only: bool,
    default: Arc<Ruleset>,
    rule_definition: HashSet<RuleDefinition>,
    ruleset: HashSet<RuleDefinition>,
}

struct Ruleset {
    id: String,
    read_only: bool,
    parent: Arc<Ruleset>,
    name: String,
    rule: HashSet<(String, String)>, // the key is aRule.toString
}

struct RuleDefinition {
    id: String,
    read_only: bool,
    description: String,
    value: RuleValue,
}

enum RuleValue {
    Boolean(bool, String, String),
    Integer(usize),
    r#String(String),
    Time(u64), // todo: chrono
}

struct Game {
    id: String,
    read_only: bool,
    name: String,
    name_format: String,
    state: GameState,
    current_period_number: usize,
    current_period: Arc<Period>,
    upcoming_jam: Arc<Jam>,
    upcoming_jam_number: usize,
    in_period: bool,
    in_jam: bool,
    in_overtime: bool,
    in_sudden_scoring: bool, // JRDA rules
    injury_continuation_upcoming: bool,
    inhibit_final_score: bool,
    official_score: bool,
    abort_reason: String,
    current_timeout: Arc<Timeout>,
    timeout_owner: Arc<TimeoutOwner>,
    official_review: bool,
    or_is_to: bool,
    no_more_jam: bool,
    ruleset: Arc<Ruleset>,
    ruleset_name: String,
    head_nso: Arc<Official>,
    head_ref: Arc<Official>,
    suspensions_served: String,
    filename: String,
    last_file_update: String,
    update_in_progress: bool,
    statsbook_exists: bool,
    json_exists: bool,
    clock_during_final_score: bool,
    export_blocked_by: bool,
    fiiive_seconds: bool,
    auto_five: bool,
    five_indicator: String,
    official_crew: Arc<OfficialsCrew>,
    clock: HashSet<Clock>,
    team: HashSet<Team>,
    rule: HashSet<ValWithId>,
    penalty_code: HashSet<ValWithId>,
    label: HashSet<(Button, String)>,
    event_info: HashSet<(EventInfo, String)>,
    nso: HashSet<Official>,
    //r#ref: HashSet<Official>, or
    referee: HashSet<Official>,
    official_position: HashSet<OfficialPosition>,
    expulsion: HashSet<Expulsion>,
    period: HashMap<usize, Period>,
}

enum Button {
    Start,
    Stop,
    Timeout,
    Undo,
    Replaced,
}

enum EventInfo {
    Venue,
    City,
    State, // todo: deUSize
    Tournament,
    HostLeague,
    GameNo,
    Date,
    StartTime,
}

enum GameState {
    Prepared,
    Running,
    Finished,
}

enum ClockID {
    Period,
    Jam,
    Lineup,
    Timeout,
    Intermission,
}

// todo: chrono
struct Clock {
    id: ClockID,
    read_only: bool,
    name: String,
    number: usize,
    time: u64,
    inverted_time: u64,
    maximum_time: u64,
    direction: bool,
    running: bool,
}

struct Team {
    id: String,
    read_only: bool,
    display_name: String,
    full_name: String,
    league_name: String,
    team_name: String,
    file_name: String,
    initials: String,
    uniform_color: String, // [sic]
    logo: String,
    running_or_upcoming_team_jam: bool,
    running_or_ended_team_jam: bool,
    plt_teram_jam: Arc<TeamJam>,
    fielding_advance_pending: bool,
    current_trip: Arc<ScoringTrip>,
    score: usize,
    jam_score: usize,
    trip_score: usize,
    last_score: usize,
    timeouts: usize,
    official_reviews: usize,
    last_review: Arc<Timeout>,
    in_timeout: bool,
    in_official_review: bool,
    no_pivot: bool,
    retained_official_review: bool,
    lost: bool,
    lead: bool,
    calloff: bool,
    injury: bool,
    no_initial: bool,
    display_lead: bool,
    star_pass: bool,
    star_pass_trip: Arc<ScoringTrip>,
    prepared_team: Arc<Team>, // PreparedX means "X when it's on disk"
    prepared_team_connected: bool, // todo: wtf?
    captain: Arc<Skater>,
    active_score_adjustment: Arc<ScoreAdjustment>,
    active_score_adjustment_amount: isize,
    total_penalties: usize,
    all_blockers_set: bool,
    on_track_count: usize,
    alternate_name: HashSet<ValWithId>, // ids appear domain-limited to Alternate?NameId
    color: HashSet<ValWithId>, // [sic]
    skater: HashSet<Skater>,
    position: HashSet<Position>,
    time_out: HashSet<Timeout>,
    box_trip: HashSet<BoxTrip>,
    score_adjustment: HashSet<ScoreAdjustment>,

}

enum AlternateNameId {
    Scoreboard,
    Whiteboard,
    Operator,
    PLT,
    r#Box,
    Overlay,
    Twitter,
}

struct TeamJam {
    id: String,
    read_only: bool,
    number: usize,
    current_trip: Arc<ScoringTrip>,
    current_trip_number: usize,
    last_score: usize,
    os_offset: isize,
    os_offset_reason: String,
    jam_score: usize,
    after_s_p_score: usize,
    total_score: usize,
    lost: bool,
    lead: bool,
    calloff: bool,
    no_initial: bool,
    injury: bool,
    display_lead: bool,
    star_pass: bool,
    star_pass_trip: Arc<ScoringTrip>,
    no_pivot: bool,
    all_blockers_set: bool,
    on_track_count: usize,
    lt_annotation: String,
    sk_annotation: String,
    fielding: HashSet<Fielding>,
    scoring_trip: HashMap<usize, ScoringTrip>,
}

struct Timeout {
    id: String,
    read_only: bool,
    owner: Arc<TimeoutOwner>,
    review: bool,
    retained_review: bool,
    or_request: String,
    or_result: String,
    running: bool,
    preceding_jam: Arc<Jam>,
    preceding_jam_number: usize,
    duration: u64, // todo: chrono
    period_clock_elapsed_start: u64, // todo: chrono
    period_clock_elapsed_end: u64, // todo: chrono
    walltime_start: u64, // todo: chrono
    walltime_end: u64, // todo: chrono
}

enum TimeoutOwner {
    Team(Team),
    r#None,
    OTO,
}

struct ScoreAdjustment {
    id: String,
    read_only: bool,
    amount: usize,
    jam_recorded: Arc<Jam>,
    period_number_recorded: usize,
    jam_number_recorded: usize,
    recorded_during_jam: bool,
    last_two_minutes: bool,
    open: bool,
    applied_to: Arc<ScoringTrip>,
}

struct Skater {
    id: String,
    read_only: bool,
    name: String,
    roster_number: String,
    current_fielding: Arc<Fielding>,
    current_box_symbols: String,
    current_penalties: String,
    penalty_count: usize,
    position: Arc<Position>,
    role: SkaterRole,
    base_role: SkaterRole,
    penalty_box: bool,
    has_unserved: bool,
    flags: String,
    pronouns: String,
    color: String, // [sic]
    penalty_details: String,
    extra_penalty_time: u64, // todo: chrono
    fielding: HashSet<Fielding>,
    sub_penalties: HashSet<Penalty>,
    penalty: HashMap<usize, Penalty>,
}

struct Jam {
    id: String,
    read_only: bool,
    number: usize,
    period_number: usize,
    star_pass: bool, // "true, if either team had an SP"
    overtime: bool,
    injury_continuation: bool,
    duration: u64, // todo: chrono
    period_clock_elapsed_start: u64, // todo: chrono
    period_clock_elapsed_end: u64, // todo: chrono
    period_clock_display_end: u64, // todo: chrono
    walltime_start: u64, // todo: chrono
    walltime_end: u64, // todo: chrono
    team_jam: HashSet<TeamJam>,
    penalty: HashSet<Penalty>,
    timeouts_after: HashSet<Timeout>,
}

// chutten: No, I also don't understand why Media* is in a 1:n multitree.
struct Media {
    id: String,
    read_only: bool,
    format: HashSet<MediaFormat>,
}

struct MediaFormat {
    id: String,
    read_only: bool,
    //r#type: HashSet<MediaType>, // or
    media_type: HashSet<MediaType>,
}

struct MediaType {
    id: String,
    read_only: bool,
    file: HashSet<MediaFile>,
}

struct MediaFile {
    id: String,
    read_only: bool,
    src: String,
    name: String,
}

struct Penalty {
    id: String,
    read_only: bool,
    number: usize,
    time: u64, // todo: chrono
    jam: Arc<Jam>,
    period_number: usize,
    jam_number: usize, // because jam.number is too hard to type?
    code: String,
    serving: bool,
    served: bool,
    force_served: bool,
    box_trip: Arc<BoxTrip>,
    annotation: String,
    calling_official: Arc<Official>,
    calling_position: Arc<OfficialPosition>, // because calling_official.position is too hard to type?
}

struct Period {
    id: String,
    read_only: bool,
    number: usize,
    current_jam: Arc<Jam>,
    current_jam_number: usize, // because current_jam.number is too hard to type?
    sudden_scoring: bool,
    duration: u64, // todo: chrono
    walltime_start: u64, // todo: chrono
    walltime_end: u64, // todo: chrono
    local_time_start: u64, // todo: chrono
    team_1_penalty_count: usize,
    team_2_penalty_count: usize,
    team_1_points: usize,
    team_2_points: usize,
    timeout: Arc<Timeout>,
    jam: HashMap<usize, Jam>,
}

struct Position {
    id: String,
    read_only: bool,
    current_fielding: Arc<Fielding>,
    current_box_symbols: String,
    current_penalties: String,
    annotation: String,
    skater: Arc<Skater>,
    name: String,
    roster_number: String,
    flags: String,
    penalty_box: bool,
    has_unserved: bool,
    penalty_time: u64, // todo: chrono
    penalty_count: usize,
    penalty_details: String,
    extra_penalty_time: u64, // todo: trigger
}

struct BoxTrip {
    id: String,
    read_only: bool,
    is_current: bool,
    current_fielding: Arc<Fielding>,
    start_fielding: Arc<Fielding>,
    start_jam_number: usize,
    start_between_jams: bool,
    start_after_s_p: bool,
    end_fielding: Arc<Fielding>,
    end_jam_number: usize,
    end_between_jams: bool,
    end_after_s_p: bool,
    // todo chrono
    walltime_start: u64,
    walltime_end: u64,
    jam_clock_start: u64,
    jam_clock_end: u64,
    duration: u64,
    current_skater: Arc<Skater>,
    roster_number: String,
    penalty_codes: String,
    total_penalties: usize,
    timing_stopped: bool,
    time: u64,
    shortened: usize,
    penalty_details: String,
    jammer: bool,
    fielding: HashSet<Fielding>,
    penalty: HashSet<Penalty>,
    clock: HashSet<Clock>,
}

struct Expulsion {
    id: String,
    read_only: bool,
    info: String,
    extra_info: String,
    suspension: bool,
}

struct Fielding {
    id: String,
    read_only: bool,
    number: usize,
    skater: Arc<Skater>,
    skater_number: String,
    not_fielded: bool,
    position: usize,
    sit_for_3: bool,
    penalty_box: bool,
    has_unserved: bool,
    current_box_trip: Arc<BoxTrip>,
    box_trip_symbols: String,
    box_trip_symbols_before_s_p: String,
    box_trip_symbols_after_s_p: String,
    penalty_time: u64, // todo: chrono
    annotation: String,
    box_trip: HashSet<BoxTrip>,
}

enum FloorPosition {
    Jammer,
    Pivot,
    Blocker1,
    Blocker2,
    Blocker3,
}

struct Official {
    id: String,
    read_only: bool,
    role: OfficialRole,
    name: String,
    league: String,
    cert: String,
    p1_team: String,
    swap: bool,
    current_position: Arc<OfficialPosition>,
    prepared_official: Arc<Official>, // PreparedX is "X when it came from disk"{
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

struct OfficialPosition {
    id: String,
    read_only: bool,
    name: String,
    team: Arc<Team>,
    removable: bool,
    current_official: Arc<Official>,
    current_official_name: String, // Because current_official.name is too hard to type?
    penalties: HashSet<Penalty>,
}

struct OfficialsCrew {
    id: String,
    read_only: bool,
    name: String,
    head_nso: Arc<Member>,
    head_ref: Arc<Member>,
    nso: HashSet<Member>,
    //r#ref: HashSet<Member>, // or,
    referee: HashSet<Member>,
}

struct Member {
    id: String,
    read_only: bool,
    role: String, // todo: is this also a `Role`?
    name: String,
    league: String,
    cert: String,
    prepared_official: Arc<Official>, // PreparedX means "X when it's on disk"
}

enum SkaterRole { // from core.interfaces.Role
    Jammer,
    Pivot,
    Blocker,
    Bench,
    NotInGame,
    Ineligible,
}

struct Clients {
    id: String,
    read_only: bool,
    new_device_write: bool,
    all_local_devices_write: bool,
    device: HashSet<Device>,
}

struct Client {
    id: String,
    read_only: bool,
    remote_addr: String,
    platform: String,
    source: String,
    created: u64, // todo: chrono,
    wrote: u64,
}

struct Device {
    id: String,
    read_only: bool,
    session_id: String, // todo: This is "Secret"
    name: String,
    remote_addr: String,
    platform: String,
    comment: String,
    created: u64, // todo: chrono
    wrote: u64,
    accessed: u64,
    may_write: bool,
    num_clients: usize,
    client: HashSet<Client>,
}
