use std::collections::HashMap;

enum TeamResult {
    Win,
    Loss,
    Draw,
}

pub fn tally(match_results: &str) -> String {
    let mut team_to_results = HashMap::new();
    match_results.lines().for_each(|line| {
        let information = line.split(';').take(3).collect::<Vec<&str>>();
        let team1 = information[0];
        let team2 = information[1];
        let result = information[2];

        match result {
            "win" => {
                let team = team1;
                let results = team_to_results.entry(team).or_insert(Vec::new());
                results.push(TeamResult::Win);

                let team = team2;
                let results = team_to_results.entry(team).or_insert(Vec::new());
                results.push(TeamResult::Loss);
            }
            "loss" => {
                let team = team2;
                let results = team_to_results.entry(team).or_insert(Vec::new());
                results.push(TeamResult::Win);

                let team = team1;
                let results = team_to_results.entry(team).or_insert(Vec::new());
                results.push(TeamResult::Loss);
            }
            "draw" => {
                let team = team1;
                let results = team_to_results.entry(team).or_insert(Vec::new());
                results.push(TeamResult::Draw);

                let team = team2;
                let results = team_to_results.entry(team).or_insert(Vec::new());
                results.push(TeamResult::Draw);
            }
            _ => {
                panic!()
            }
        }
    });

    let mut team_informations = Vec::new();
    for (team, results) in team_to_results {
        let team_info = process_team_results(team, &results);
        team_informations.push(team_info);
    }

    team_informations.sort_by(|lhs, rhs| {
        rhs.points
            .cmp(&lhs.points)
            .then_with(|| lhs.name.cmp(&rhs.name))
    });

    create_table(&team_informations)
}

fn create_table(infos: &Vec<TeamInfo>) -> String {
    let mut text = format!("{:<30} | MP |  W |  D |  L |  P", "Team");
    if !infos.is_empty() {
        text += "\n";
    }
    let mut it = infos.iter().peekable();
    while let Some(info) = it.next() {
        text += String::from(info).as_str();
        if it.peek().is_some() {
            text += "\n";
        }
    }
    text
}

struct TeamInfo {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl From<&TeamInfo> for String {
    fn from(origin: &TeamInfo) -> Self {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            origin.name,
            origin.matches_played,
            origin.wins,
            origin.draws,
            origin.losses,
            origin.points
        )
    }
}

const POINTS_PER_WIN: u32 = 3;
const POINTS_PER_DRAW: u32 = 1;
const POINTS_PER_LOSS: u32 = 0;
fn process_team_results(name: &str, results: &Vec<TeamResult>) -> TeamInfo {
    let matches_played = results.len() as u32;
    let mut wins = 0;
    let mut draws = 0;
    let mut losses = 0;
    let mut points = 0;
    for result in results {
        match result {
            TeamResult::Win => {
                wins += 1;
                points += POINTS_PER_WIN;
            }
            TeamResult::Draw => {
                draws += 1;
                points += POINTS_PER_DRAW;
            }
            TeamResult::Loss => {
                losses += 1;
                points += POINTS_PER_LOSS;
            }
        };
    }

    TeamInfo {
        name: name.to_string(),
        matches_played,
        wins,
        draws,
        losses,
        points,
    }
}
