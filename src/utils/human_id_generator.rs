use rand;
use rand::Rng;
use rand::rngs::SmallRng;

const TERMS: [&str; 122] = [
    "skater",
    "jammer",
    "pivot",
    "blocker",
    "alternate",
    "captain",
    "jam",
    "period",
    "timeout",
    "lineup",
    "team",
    "review",
    "start",
    "stop",
    "seconds",
    "whistle",
    "rolling",
    "stoppage",
    "clock",
    "tweet",
    "illegal",
    "violation",
    "target",
    "blocking",
    "zone",
    "position",
    "multiplayer",
    "pass",
    "penalty",
    "score",
    "trip",
    "point",
    "initial",
    "interference",
    "delay",
    "procedure",
    "expulsion",
    "gross",
    "foulout",
    "warning",
    "block",
    "gaining",
    "report",
    "return",
    "impact",
    "high",
    "low",
    "contact",
    "direction",
    "clockwise",
    "impenetrable",
    "pack",
    "split",
    "play",
    "out",
    "in",
    "skating",
    "destruction",
    "bounds",
    "failure",
    "yield",
    "miscounduct",
    "false",
    "line",
    "stay",
    "lead",
    "lost",
    "call",
    "engagement",
    "complete",
    "incomplete",
    "stand",
    "done",
    "overtime",
    "reentry",
    "insubordination",
    "unsporting",
    "cut",
    "swap",
    "spectrum",
    "head",
    "back",
    "shoulder",
    "knee",
    "toe",
    "torso",
    "finger",
    "leg",
    "chin",
    "thigh",
    "pads",
    "mouth",
    "guard",
    "wrist",
    "elbow",
    "forearm",
    "hand",
    "shin",
    "wheel",
    "truck",
    "star",
    "stripe",
    "helmet",
    "cover",
    "toestop",
    "face",
    "nose",
    "uniform",
    "number",
    "bridge",
    "goat",
    "wall",
    "tripod",
    "recycle",
    "runback",
    "lane",
    "power",
    "short",
    "flat",
    "banked",
    "minor",
    "major",
];
// If there's a duplicate, we take from this list.
const OVERFLOW: [&str; 6] = ["ball", "offside", "touchdown", "goalie", "racket", "grass"];

pub struct Generator {
    rng: SmallRng,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            rng: rand::make_rng()
        }
    }

    pub fn generate(&mut self) -> String {
        let i1 = self.rng.next_u32() as usize % TERMS.len();
        let i2 = self.rng.next_u32() as usize % TERMS.len();
        if i1 != i2 {
            format!("{}-{}", TERMS[i1], TERMS[i2])
        } else {
            format!(
                "{}-{}",
                TERMS[i1],
                OVERFLOW[self.rng.next_u32() as usize % OVERFLOW.len()]
            )
        }
    }
}
