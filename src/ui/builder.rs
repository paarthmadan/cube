use super::*;

pub struct SolveInfo {
    pub scramble: String,
    pub time: String,
    pub cube_type: String,
}

impl From<&App> for SolveInfo {
    fn from(app: &App) -> Self {
        let time = match app.state {
            State::Idle | State::Timing => match app.active_timer {
                Some(timer) => format!("{:.4}", timer.time().as_secs_f64()),
                None => match app.times.last() {
                    Some(time) => format!("{:.4}", time.as_secs_f64()),
                    None => format!("{:.4}", 0.0),
                },
            },
            State::Inspection(time) => time.to_string(),
        };

        let cube_type = String::from("3x3");
        let scramble = app.scramble.to_string();

        SolveInfo {
            time,
            scramble,
            cube_type,
        }
    }
}

pub struct Stats {
    pub recent_solves: String,
    pub stats: String,
    pub graph: GraphInfo,
}

impl From<&App> for Stats {
    fn from(app: &App) -> Self {
        let recent_solves = app
            .times
            .iter()
            .rev()
            .map(|s| format!("{:.4}", s.as_secs_f64()) + "\n")
            .collect();

        let mut worst = 0.0;

        let stats = app
            .compute_statistics()
            .iter()
            .map(|(label, value)| {
                let value = match value {
                    Some(v) => {
                        if label == "worst" {
                            worst = v.as_secs_f64().ceil();
                        }
                        format!("{:.4}", v.as_secs_f64())
                    }
                    None => String::from("NA"),
                };
                format!("{}: {}\n", label, value)
            })
            .collect();

        // TODO: Move this logic elsewhere
        let n = match app.times.len() & 1 {
            0 => app.times.len(),
            1 => app.times.len() + 1,
            _ => unreachable! {},
        };

        let points: Vec<(f64, f64)> = app
            .times
            .iter()
            .enumerate()
            .map(|(i, time)| (i as f64, time.as_secs_f64()))
            .collect();

        let x_axis = Axis {
            bounds: (0.0, n as f64),
            labels: vec![String::from("0.0"), n.to_string()],
        };

        let y_axis = Axis {
            bounds: (0.0, worst * 1.5), //TODO(magic-number)
            labels: vec![String::from("0.0"), (worst * 1.5).to_string()],
        };

        let graph = GraphInfo {
            x_axis,
            y_axis,
            points,
        };

        Stats {
            recent_solves,
            stats,
            graph,
        }
    }
}

pub struct GraphInfo {
    pub points: Vec<(f64, f64)>,
    pub x_axis: Axis,
    pub y_axis: Axis,
}

pub struct Axis {
    pub bounds: (f64, f64),
    pub labels: Vec<String>,
}
