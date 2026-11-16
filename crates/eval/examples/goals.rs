//! Debug helper: how many curiosity goals each teaching dialogue leaves open.
use sla_agent::curiosity::ScriptedTeacher;
use sla_eval::{run_dialogue, Dialogue};

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/synthetic/teaching/dev");
    for d in Dialogue::load_dir(&root) {
        let mut t = ScriptedTeacher { answers: vec![], default: "I don't know.".into() };
        let r = run_dialogue(&d, Some(&mut t), 3);
        println!("{:30} teacher questions: {}", d.name, r.teacher_questions);
        for (q, a) in r.transcript.iter().filter(|(q, _)| q.starts_with("[agent asks]")) {
            println!("    {q} | {a}");
        }
    }
}
