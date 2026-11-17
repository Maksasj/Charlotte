"""Write the seed predicate registry (Design 2.0, milestone 13):
crates/core/assets/lexicon/registry.md, copied into vault/lexicon/ on first open.

The registry is data, not code: one entry per predicate with its argument
types, a plain-English definition, sentence frames, synonyms and flags, plus
the word lists the validator uses (units, function words, number words).
The verb list is taken from the pattern parser's lexicon so that every verb
it can produce is a known predicate.

Run from the repo root:  python tools/make_registry.py
"""

import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = ROOT / "crates" / "core" / "assets" / "lexicon" / "registry.md"

# name, types, definition, frame, flags, synonyms
PREDS = [
    # Primes (v1 core)
    ("thing", "entity", "X is a thing", "{1} is a thing", "", ""),
    ("same", "entity entity", "X and Y are the same thing", "{1} is the same as {2}", "", ""),
    ("is_a", "entity category", "X is a kind of Y, or an instance of Y", "{1} is a {2}", "askable rule_head", "kind_of type_of"),
    ("part_of", "entity entity", "X is part of Y", "{1} is part of {2}", "askable rule_head", ""),
    ("has", "entity entity", "X has Y", "{1} has {2}", "askable rule_head open_world", "own possess"),
    ("property", "entity symbol value", "X has property P with value V", "the {2} of {1} is {3}", "askable rule_head", ""),
    ("before", "any any", "A happens before B", "{1} comes before {2}", "askable rule_head", ""),
    ("during", "any any", "A happens during T", "{1} happens during {2}", "askable", ""),
    ("event", "event", "E is an event", "{1} is an event", "", ""),
    ("agent_of", "event entity", "X is the one who acts in event E", "{2} does {1}", "", ""),
    ("does", "entity symbol", "X does action A", "{1} does {2}", "askable", ""),
    ("causes", "any any", "A causes B", "{1} causes {2}", "askable rule_head", "lead_to"),
    ("wants", "entity any", "X wants Y", "{1} wants {2}", "askable rule_head open_world", "want"),
    ("knows", "entity any", "X knows fact or thing F", "{1} knows {2}", "askable open_world", ""),
    ("believes", "entity any", "X believes fact F", "{1} believes {2}", "askable open_world", ""),
    ("feels", "entity symbol", "X feels F", "{1} feels {2}", "askable", ""),
    ("likes", "entity any", "X likes Y", "{1} likes {2}", "askable rule_head", "enjoy love"),
    ("person", "entity", "X is a person", "{1} is a person", "rule_head", ""),
    ("place", "entity", "X is a place", "{1} is a place", "rule_head", ""),
    ("located_in", "entity entity", "X is in place Y", "{1} is in {2}", "askable rule_head single_valued", "live_in"),
    ("says", "entity any", "X says fact F", "{1} says {2}", "", ""),
    ("proper_name", "entity", "X is written as a name", "", "", ""),
    ("work_at", "entity entity", "X works at Y", "{1} works at {2}", "askable rule_head", ""),
    # Procedural layer
    ("task", "task", "T is a task someone can do", "{1} is something you can do", "rule_head", ""),
    ("step", "task step", "S is a step of task T", "{2} is a step of {1}", "", ""),
    ("action", "any symbol", "the action of step or event E is A", "", "", ""),
    ("object_of", "any entity", "the thing acted on in E is X", "", "", ""),
    ("recipient", "event entity", "the one who receives in event E is X", "", "", ""),
    ("needs", "task entity", "task T needs item I", "{1} needs {2}", "askable rule_head", "require"),
    ("uses", "step entity", "step S uses item I", "{1} uses {2}", "", ""),
    ("takes_time", "any number number unit", "X takes Lo to Hi of unit U", "{1} takes {2} to {3} {4}s", "askable single_valued", "last"),
    ("until", "step symbol", "step S goes on until state C", "until {2}", "", ""),
    ("overlaps", "step step", "step S runs while step R runs", "while {2}", "", ""),
    ("method", "task task", "task T can be done by doing task M", "", "", ""),
    ("subtask", "task task", "task T includes task U", "{1} includes {2}", "", ""),
    ("done_when", "task symbol", "task T is done when state C holds", "{1} is done when {2}", "", ""),
    # Choice layer
    ("solves", "task symbol", "task T is a way to reach goal G", "{1} is a way to {2}", "askable", ""),
    ("recommended_for", "task any", "option T is good for situation X", "{1} is good for {2}", "askable", ""),
    ("available", "entity", "the user has item I", "you have {1}", "", ""),
    ("time_available", "number unit", "the user has N units of time", "you have {1} {2}", "single_valued", ""),
    ("staple", "entity", "item I is in almost every home", "{1} is in almost every home", "", ""),
    ("substitute", "entity entity", "item J can replace item I", "{2} can replace {1}", "askable", ""),
    ("portion", "entity number unit", "one person needs N of item I", "one person needs {2} {3} of {1}", "single_valued", ""),
    # Session layer
    ("active_task", "task", "the conversation is about task T", "", "", ""),
    ("at_step", "step", "the user is at step S", "", "", ""),
    ("slot", "symbol any", "slot N of the active task has value V", "", "", ""),
    ("open_question", "any", "the agent is waiting for an answer to Q", "", "", ""),
    ("grounded", "fact", "fact F was confirmed in this conversation", "", "", ""),
    ("said", "fact", "the agent already said fact F", "", "", ""),
    ("done", "step", "the user has done step S", "", "", ""),
    # Lexical layer
    ("same_pred", "symbol symbol", "predicates P and Q mean the same", "", "", ""),
    ("surface", "entity text", "X is written Text", "", "", ""),
    ("plural_of", "entity text", "the plural of X is written Text", "", "", ""),
    ("frame", "symbol text", "predicate P is said with sentence frame Text", "", "", ""),
    # Self layer
    ("can", "symbol", "the agent can do X", "I can {1}", "", ""),
    ("cannot", "symbol", "the agent cannot do X", "I cannot {1}", "", ""),
    ("knows_task", "task", "the agent knows how to do task T", "I know how to do {1}", "askable", ""),
    # Derived by core rules (registered so questions about them type-check)
    ("needs_all", "task entity", "everything task T needs, through steps and sub-tasks", "{1} needs {2}", "askable", ""),
    ("missing", "task entity", "item I is needed for T and the user lacks it", "you are missing {2} for {1}", "askable", ""),
    ("covered", "entity", "a substitute for I is at hand", "something can replace {1}", "", ""),
    ("lacking", "task", "something needed for T is missing", "something for {1} is missing", "", ""),
    ("feasible", "task", "the user can do T with what they have", "you can make {1}", "askable", ""),
    ("option_for", "symbol task", "a feasible way to reach goal G", "{2} is a way to {1}", "askable", ""),
    ("waiting", "step", "a step that must wait for an earlier one", "{1} must wait", "", ""),
    ("next_step", "task step", "the next step of T to do", "", "askable", ""),
    ("minutes", "any number number", "X takes Lo to Hi minutes", "{1} takes {2} to {3} minutes", "askable", ""),
    ("adds_time", "task step", "step S adds to the time of T", "{2} adds to the time of {1}", "", ""),
    ("side_by_side", "step", "step S runs during another step", "{1} runs while another step runs", "", ""),
    ("total_minutes", "task number number", "task T takes Lo to Hi minutes in all", "{1} takes {2} to {3} minutes in all", "askable", ""),
    ("step_count", "task number", "task T has N steps", "{1} has {2} steps", "askable", ""),
    # Task schemas (vault/tasks/)
    ("task_slot", "task symbol symbol", "task T has slot S, required or optional", "", "", ""),
    ("slot_question", "task symbol text", "the agent asks Text to fill slot S of task T", "", "", ""),
    ("slot_type", "task symbol symbol", "slot S of task T holds things of kind K", "", "", ""),
    ("closing", "task text", "the line that closes task T", "", "", ""),
    ("slot_fills", "task symbol symbol", "slot S of task T is filled by facts of predicate P", "", "", ""),
    # Dialogue policy (crates/core/assets/core/policy.md): facts of the moment and moves
    ("heard", "symbol", "the message held act A", "", "", ""),
    ("requested", "task", "the person asked for help with G", "", "", ""),
    ("open_frame", "task", "the conversation has an open task frame for G", "", "", ""),
    ("filled", "task symbol", "slot S of frame G is filled", "", "", ""),
    ("active", "task", "procedure T is under way", "", "", ""),
    ("chosen", "task", "option T was already offered", "", "", ""),
    ("known_goal", "task", "the agent knows goal or task G", "", "", ""),
    ("missing_slot", "task symbol", "slot S of frame G is still open", "", "", ""),
    ("asking", "task", "frame G still has open slots", "", "", ""),
    ("budget", "number", "the person has N minutes", "", "", ""),
    ("too_long", "task", "T takes longer than the time the person has", "", "", ""),
    ("candidate", "task task", "T is an option for frame G", "", "", ""),
    ("good_option", "task task", "T is a feasible option for G that fits the time", "", "", ""),
    ("has_good", "task", "frame G has a good option", "", "", ""),
    ("offered", "task", "an option for G was already offered", "", "", ""),
    ("has_next", "task", "procedure T has a next step", "", "", ""),
    ("any_active", "", "some procedure is under way", "", "", ""),
    ("any_frame", "", "some frame is open", "", "", ""),
    ("dm_move", "symbol number", "the dialogue move M with priority P", "", "", ""),
    # Registry and bookkeeping predicates used by the agent itself
    ("fact_id", "fact any any", "fact F is about these values", "", "", ""),
]

UNITS = [
    ("second", "time", "1"), ("minute", "time", "60"), ("hour", "time", "3600"), ("day", "time", "86400"), ("week", "time", "604800"),
    ("gram", "mass", "1"), ("kilogram", "mass", "1000"), ("millilitre", "volume", "1"), ("litre", "volume", "1000"),
    ("cup", "volume", "240"), ("spoon", "volume", "15"), ("teaspoon", "volume", "5"), ("tablespoon", "volume", "15"),
    ("piece", "count", "1"), ("person", "count", "1"), ("slice", "count", "1"), ("degree", "temperature", "1"),
    ("metre", "length", "1"), ("kilometre", "length", "1000"), ("centimetre", "length", "0.01"),
]

# Words that are never entities: deictics, pronouns, determiners, connectives.
FUNCTION_WORDS = """here there this that these those it its they them he she him her we us you i me my your our their
a an the and or but so because if then than as of to for from with without in on at by into onto about over under
between through during before after while until when where which who whom whose what how why not no yes all some any
each every many much more most few less least very too also just only even still
first second next finally afterwards meanwhile""".split()

NUMBER_WORDS = """zero one two three four five six seven eight nine ten eleven twelve thirteen fourteen fifteen sixteen seventeen
eighteen nineteen twenty thirty forty fifty sixty seventy eighty ninety hundred thousand million half quarter dozen""".split()

# Words that glue phrases into one constant (good_for_eggs_and_frying, part_that_stops_car).
GLUE_WORDS = """and or but to of for from with that which who where when than as by about into onto like
is are was were be been being it this these those here there""".split()

# Adjectives and the named property each describes (v1's `quality` catch-all is gone).
ADJECTIVES = {
    "temperature": "hot cold warm cool freezing boiling icy chilly",
    "weight": "heavy light",
    "speed": "fast slow quick",
    "age": "old young new ancient modern",
    "texture": "soft hard smooth rough sticky crunchy creamy fluffy",
    "taste": "sweet sour bitter salty spicy tasty delicious bland",
    "state": "broken ripe fresh rotten clean dirty empty full open closed wet dry ready cooked raw burnt flat busy",
    "difficulty": "easy difficult simple tricky",
    "depth": "deep shallow",
    "length": "long",
    "power": "electric solar manual",
    "diet": "vegan vegetarian",
    "character": "lazy kind friendly shy brave funny smart clever polite rude calm patient honest",
    "appeal": "fun boring exciting interesting popular",
    "clarity": "clear cloudy",
    "loudness": "loud quiet noisy",
    "brightness": "bright dark",
    "price": "cheap expensive free",
    "health": "healthy sick ill",
    "shape": "round square",
    "beauty": "beautiful pretty ugly",
    "safety": "safe dangerous",
    "strength": "strong weak",
    "wealth": "rich poor",
    "height": "high low",
}

# Property names a translation may use without the word in the message ("The car is red" -> colour).
PROPERTY_NAMES = """colour size age job name taste weight height price temperature shape material smell sound speed
difficulty texture state length width capacity flavour role value order hobby favourite_food nationality language""".split() + sorted(ADJECTIVES)

FORBID = [("property", 2, "quality", "a quality needs a named property (colour, size, taste, difficulty) or its own predicate")]


def verbs():
    src = (ROOT / "crates" / "parser" / "src" / "lexicon.rs").read_text(encoding="utf-8")
    m = re.search(r"pub const VERBS: &\[&str\] = &\[(.*?)\];", src, re.S)
    words = re.findall(r'"([a-z]+)"', m.group(1))
    named = {p[0] for p in PREDS}
    # Verbs that start everyday task names but that the v1 parser never needed.
    words += ["check", "invite", "pack", "hang", "replace", "defrost", "unclog", "tighten", "reset", "wrap", "sew", "iron", "vacuum"]
    out = []
    for w in words:
        if w not in out and w not in named:
            out.append(w)
    return out


def q(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def main():
    lines = []
    for name, types, definition, frame, flags, syns in PREDS:
        lines.append(f"pred({name}, {q(types)}, {q(definition)}).")
        if frame:
            lines.append(f"frame({name}, {q(frame)}).")
        for f in flags.split():
            lines.append(f"flag({name}, {f}).")
        for s in syns.split():
            lines.append(f"synonym({s}, {name}).")
    lines.append("% Verbs from the pattern parser: verb(V) means V(X) or V(X, Y) with an entity first.")
    for v in verbs():
        lines.append(f"verb({v}).")
    lines.append("% Prepositions: a verb and a preposition make a predicate (talk_to, sleep_on, study_at).")
    lines.extend(f"preposition({w})." for w in "in at on with for to from about into near by under over after before through around across along behind".split())
    lines.append("% Units: unit(Name, Dimension, size in the dimension's base unit).")
    for u, d, n in UNITS:
        lines.append(f"unit({u}, {d}, {n}).")
    lines.append("% Words the validator never accepts as an entity, and words that glue a phrase into one constant.")
    lines.extend(f"function_word({w})." for w in FUNCTION_WORDS)
    lines.extend(f"number_word({w})." for w in NUMBER_WORDS)
    lines.extend(f"glue_word({w})." for w in GLUE_WORDS)
    lines.extend(f"property_name({w})." for w in dict.fromkeys(PROPERTY_NAMES))
    lines.append("% Adjectives: adjective(Word, Property) - \"Anna is lazy\" is property(anna, character, lazy).")
    for prop, words in ADJECTIVES.items():
        lines.extend(f"adjective({w}, {prop})." for w in words.split())
    for p, pos, v, why in FORBID:
        lines.append(f"forbid({p}, {pos}, {v}, {q(why)}).")
    text = (
        "---\nid: registry\nstatus: confirmed\nconf: 1\nevidence: 0\ncounter: 0\nmodule: lexicon\nsource: builtin\n---\n"
        "# Predicate registry\n\n"
        "Every predicate the agent may store or ask about, with its argument types,\n"
        "a definition, sentence frames (`{1}` is the first argument), synonyms and flags.\n"
        "Argument types: entity, category, event, task, step, number, unit, fact (a fact id\n"
        "such as `f12`), text (a quoted string), symbol (a plain word), value (entity-like\n"
        "word or number), any. Flags: askable, rule_head, single_valued, open_world.\n"
        "New predicates come in through the review queue, never silently. Edit freely;\n"
        "the agent reloads it like any vault file.\n\n"
        "```datalog\n" + "\n".join(lines) + "\n```\n"
    )
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(text, encoding="utf-8", newline="\n")
    print(f"wrote {OUT.relative_to(ROOT)}: {len(PREDS)} predicates, {len(verbs())} verbs, {len(UNITS)} units")


if __name__ == "__main__":
    main()
