use std::collections::HashMap;

const POINTS_PER_WIN: u32 = 3;
const POINTS_PER_DRAW: u32 = 1;
const POINTS_PER_LOSS: u32 = 0;

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

#[derive(Default)]
struct TeamInfo {
    name: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl TeamInfo {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn add_win(&mut self) {
        self.matches_played += 1;
        self.wins += 1;
        self.points += POINTS_PER_WIN;
    }

    fn add_draw(&mut self) {
        self.matches_played += 1;
        self.draws += 1;
        self.points += POINTS_PER_DRAW;
    }

    fn add_loss(&mut self) {
        self.matches_played += 1;
        self.losses += 1;
        self.points += POINTS_PER_LOSS;
    }
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

fn process_team_results(name: &str, results: &[TeamResult]) -> TeamInfo {
    let mut team_info = TeamInfo::new(name.to_string());
    for result in results {
        match result {
            TeamResult::Win => team_info.add_win(),
            TeamResult::Draw => team_info.add_draw(),
            TeamResult::Loss => team_info.add_loss(),
        };
    }
    team_info
}

fn create_table(infos: &[TeamInfo]) -> String {
    let header = format!("{:<30} | MP |  W |  D |  L |  P", "Team");
    let mut all_infos = vec![header];
    for info in infos {
        all_infos.push(String::from(info));
    }
    all_infos.join("\n")
}
