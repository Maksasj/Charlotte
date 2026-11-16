//! Word lists for the pattern parser.

pub const GREET: &[&str] = &["hi", "hello", "hey", "hi there", "hello there", "hey there", "good morning", "good evening", "good afternoon", "greetings", "howdy", "hiya"];
pub const BYE: &[&str] = &["bye", "goodbye", "bye bye", "see you", "see you later", "good night", "farewell", "later", "see ya", "quit", "exit"];
pub const YES: &[&str] = &[
    "yes", "yeah", "yep", "yup", "right", "correct", "exactly", "sure", "that is right", "that is correct", "true", "yes that is right",
    "yes it is", "indeed", "of course", "yes correct", "yes exactly", "absolutely", "that is true", "you are right", "yes please",
];
pub const NO: &[&str] = &["no", "nope", "nah", "no it is not", "not really", "no thanks", "no thank you"];
pub const WRONG: &[&str] = &[
    "wrong", "that is wrong", "that is not right", "that is incorrect", "incorrect", "not true", "that is not true", "you are wrong",
    "that is false", "no that is wrong", "no that is not right", "that is not correct", "you are mistaken", "no wrong",
];

/// Removed wherever they occur.
pub const DROP_ANYWHERE: &[&str] = &[
    "please", "really", "also", "always", "usually", "often", "sometimes", "still", "very", "just", "too", "anymore", "now", "today",
    "definitely", "truly", "totally", "quite", "currently", "probably", "certainly", "kinda", "honestly",
];
/// Removed at the start.
pub const LEAD_FILLERS: &[&str] = &["well", "um", "umm", "uhm", "uh", "ok", "okay", "so", "oh", "hmm", "and", "but", "alright", "anyway", "hey", "listen", "look", ","];

pub const DETERMINERS: &[&str] = &["a", "an", "the", "some", "every", "all", "each", "any", "this", "these", "those"];
pub const SPEAKER: &[&str] = &["i", "me", "my", "myself", "mine"];
pub const VAR_PRONOUNS: &[&str] = &["someone", "something", "somebody", "anyone", "anything", "anybody", "they", "it", "he", "she", "them", "him", "her"];

pub const AUX: &[&str] = &["is", "are", "am", "was", "were", "does", "do", "did", "has", "have", "had", "never", "used", "will", "can", "would"];
pub const AUX_SINGULAR: &[&str] = &["is", "was", "has", "does"];
pub const BE: &[&str] = &["is", "are", "am", "was", "were", "be"];
pub const Q_AUX: &[&str] = &["is", "are", "am", "was", "were", "does", "do", "did", "has", "have", "can"];
pub const QWORDS: &[&str] = &["what", "who", "where", "which"];

pub const EMOTIONS: &[&str] = &["happy", "sad", "tired", "angry", "excited", "bored", "hungry", "scared", "calm", "nervous"];
pub const COLOURS: &[&str] = &["red", "blue", "green", "yellow", "black", "white", "brown", "orange", "purple", "pink", "grey", "gray"];
pub const SIZES: &[&str] = &["big", "small", "large", "tiny", "huge", "tall", "short"];
pub const COLOUR_WORDS: &[&str] = &["colour", "color"];
pub const PREPS: &[&str] = &["in", "at", "on", "with", "for", "to", "from", "about", "into", "near", "by", "under", "over", "after", "before", "through"];

/// Adjectives that end in -s but are not plurals.
pub const ADJ_LIKE_S: &[&str] = &["delicious", "famous", "dangerous", "nervous", "curious", "serious", "various", "generous", "precious", "gorgeous", "enormous", "anxious", "jealous", "glorious", "obvious"];

/// Words that cannot be part of an entity name.
pub const BAD_IN_ENTITY: &[&str] = &[
    "i", "you", "we", "think", "guess", "maybe", "not", "is", "are", "was", "were", "do", "does", "did", "what", "who", "why", "how", "where",
    "if", "then", "went", "okay", "ok", "because", "so", "but", "lol", "kind", "sort",
];

/// Words that cannot be verbs (guards the fallback subject/verb split).
pub const NOT_VERBS: &[&str] = &[
    "the", "a", "an", "and", "or", "of", "to", "in", "on", "at", "with", "very", "not", "that", "this", "what", "who", "if", "then", "it", "he", "she",
    "they", "you", "we", "i", "my", "your", "be", "do",
];

/// Base forms of verbs the subject/verb split recognizes. Unknown verbs
/// still work when they carry a third-person -s.
pub const VERBS: &[&str] = &[
    "like", "love", "enjoy", "adore", "dislike", "hate", "detest", "have", "own", "possess", "want", "need", "know", "feel", "live", "cause",
    "drink", "eat", "play", "work", "read", "write", "drive", "cook", "fly", "swim", "run", "walk", "speak", "teach", "study", "visit", "use",
    "wear", "grow", "build", "sell", "buy", "make", "watch", "sing", "dance", "paint", "fix", "repair", "carry", "contain", "produce", "bark",
    "sleep", "chase", "hunt", "bite", "climb", "ride", "travel", "draw", "collect", "prefer", "bake", "brew", "catch", "code", "design",
    "manage", "lead", "learn", "plant", "water", "feed", "wash", "clean", "open", "close", "attend", "host", "meet", "help", "support",
    "follow", "lose", "win", "kick", "throw", "hit", "require", "include", "serve", "deliver", "smell", "taste", "hear", "see", "go", "come",
    "stay", "sit", "stand", "jump", "fall", "rain", "snow", "shine", "bloom", "melt", "boil", "freeze", "burn", "break", "bring", "take",
    "give", "send", "keep", "hold", "belong", "happen", "start", "end", "begin", "finish", "call", "visit", "fish", "hike", "ski", "surf",
    "jog", "knit", "sew", "fight", "cry", "laugh", "smile", "talk", "listen", "think", "believe", "remember", "forget", "understand", "speak",
    "translate", "program", "type", "print", "ship", "store", "power", "heat", "cool", "light", "connect", "protect", "cover", "fill",
    "pull", "push", "lift", "cut", "chop", "slice", "grill", "fry", "roast", "steam", "mix", "pour", "order", "pay", "cost", "earn", "save",
    "spend", "borrow", "lend", "rent", "share", "join", "leave", "enter", "exit", "move", "stop", "wait", "explore", "admire", "trust",
    "fear", "miss", "invent", "discover", "repair", "inspect", "test", "measure", "weigh", "count", "teach", "coach", "train", "guard",
    "graze", "hop", "crawl", "dig", "nest", "migrate", "purr", "howl", "roar", "chirp", "squeak", "hum", "glow", "float", "sink", "orbit",
    "flow", "grow", "shrink", "rise", "set", "attract", "repel", "dissolve", "conduct", "absorb", "reflect", "emit", "smoke", "vote",
    "drown", "mow", "brush", "comb", "shave", "dress", "iron", "fold", "pack", "unpack", "deliver", "volunteer", "practice", "practise",
    "perform", "compose", "record", "film", "edit", "review", "publish", "report", "research", "analyze", "analyse", "operate", "maintain",
    "farm", "harvest", "milk", "herd", "sail", "row", "paddle", "skate", "box", "wrestle", "golf", "bowl", "dive", "climb", "camp",
];
