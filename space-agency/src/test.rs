use crate::{calculate_candidate_score, calculate_job_score, candidate_rank, secondary_job_score};
use personnel::{AstronautJob, Candidate};

#[test]
fn test_can_calculate_secondary_job_score_when_exists() {
    let secondary_job: Option<AstronautJob> = Some(AstronautJob::Biologist);
    assert_eq!(secondary_job_score(&secondary_job), 257);
}

#[test]
fn test_can_calculate_secondary_job_score_when_not_exists() {
    let secondary_job: Option<AstronautJob> = None;
    assert_eq!(secondary_job_score(&secondary_job), 0);
}

#[test]
fn test_can_calculate_job_score_with_secondary_job() {
    let c = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biogeochemist),
        health: 10,
        age: 20,
    };
    assert_eq!(calculate_job_score(&c), 108);
}

#[test]
fn test_can_calculate_job_score_without_secondary_job() {
    let c = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: None,
        health: 10,
        age: 33,
    };
    assert_eq!(calculate_job_score(&c), 108);
}

#[test]
fn test_can_calculate_candidate_score() {
    let c = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: Some(AstronautJob::Biogeochemist),
        health: 2,
        age: 10,
    };
    assert_eq!(calculate_candidate_score(&c), 1100)
}

#[test]
fn test_can_rank_two_candidates() {
    let mut cs = vec![
        Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: Some(AstronautJob::Biogeochemist),
            health: 2,
            age: 5,
        },
        Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: None,
            health: 2,
            age: 10,
        },
    ];
    candidate_rank(&mut cs);
    let c1 = cs.get(0).unwrap();
    let c2 = cs.get(1).unwrap();
    assert_eq!(calculate_candidate_score(c1), 1100);
    assert_eq!(calculate_candidate_score(c2), 550);
}
#[test]
fn test_can_rank_many_candidates() {
    let mut cs = vec![
        Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: Some(AstronautJob::Biogeochemist),
            health: 2,
            age: 10,
        },
        Candidate {
            primary_job: AstronautJob::Biogeochemist,
            secondary_job: None,
            health: 2,
            age: 3,
        },
        Candidate {
            primary_job: AstronautJob::Biologist,
            secondary_job: None,
            health: 9,
            age: 5,
        },
        Candidate {
            primary_job: AstronautJob::Biologist,
            secondary_job: Some(AstronautJob::Biogeochemist),
            health: 5,
            age: 8,
        },
    ];
    candidate_rank(&mut cs);
    let expected_ranks = vec![3720, 1400, 1100, 330];
    let mut actual_ranks = Vec::new();
    for c in cs {
        actual_ranks.push(calculate_candidate_score(&c));
    }
    assert_eq!(expected_ranks, actual_ranks);
}
