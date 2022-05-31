use personnel::{AstronautJob, Candidate};

fn job_score(a: &AstronautJob) -> u32 {
    match &a {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn secondary_job_score(a: &Option<AstronautJob>) -> u32 {
    match &a {
        Some(j) => job_score(j),
        _ => 0,
    }
}

fn calculate_job_score(c: &Candidate) -> u32 {
    let ps = job_score(&c.primary_job);
    let ss = secondary_job_score(&c.secondary_job);
    if ss == 0 {
        (ps * ps) % 577
    } else {
        (ps * ss) % 577
    }
}

fn calculate_candidate_score(c: &Candidate) -> u32 {
    let js = calculate_job_score(c);
    let hs: u32 = c.health as u32;
    let ags: u32 = c.age as u32;
    ((js + hs) * ags) % 3928
}

fn candidate_rank(cands: &mut Vec<Candidate>) {
    cands.sort_by(|x, y| (&calculate_candidate_score(y)).cmp(&calculate_candidate_score(x)))
}

fn main() {
    let mut candidates: Vec<Candidate> = Candidate::load_candidate_file();
    candidate_rank(&mut candidates);
    for n in 0..20 {
        let c = candidates.get(n);
        match c {
            Some(ci) => {
                println!(
                    "{} Job Score: {} Candidate Score: {}",
                    ci,
                    calculate_job_score(ci),
                    calculate_candidate_score(ci)
                );
            }
            _ => break,
        }
    }
}

#[cfg(test)]
mod test;
