use clap::{Parser, Subcommand};
use rusqlite::{params, Connection, Result, Row};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
struct Problem {
    id: Option<i64>,
    description: String,
    link: Option<String>,
    category: Option<String>,
    pattern: Option<String>,
    difficulty: Option<String>,
    time_to_solve_1st: Option<i64>,
    time_to_solve_2nd: Option<i64>,
    time_to_solve_3rd: Option<i64>,
    comments: Option<String>,
    should_solve_again: bool,
}

impl Problem {
    fn new(description: &str) -> Self {
        Problem {
            id: None,
            description: description.to_string(),
            link: None,
            category: None,
            pattern: None,
            difficulty: None,
            time_to_solve_1st: None,
            time_to_solve_2nd: None,
            time_to_solve_3rd: None,
            comments: None,
            should_solve_again: false,
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Problem #{}: {} ({})",
            self.id.unwrap_or(0),
            self.description,
            self.difficulty.as_deref().unwrap_or("Unknown")
        )?;

        if let Some(category) = &self.category {
            write!(f, " - Category: {}", category)?;
        }

        if let Some(pattern) = &self.pattern {
            write!(f, " - Pattern: {}", pattern)?;
        }

        write!(f, "\n  ")?;

        if let Some(link) = &self.link {
            write!(f, "Link: {}", link)?;
        }

        write!(f, "\n  Solve times: ")?;
        match (
            self.time_to_solve_1st,
            self.time_to_solve_2nd,
            self.time_to_solve_3rd,
        ) {
            (Some(t1), Some(t2), Some(t3)) => write!(f, "{}min, {}min, {}min", t1, t2, t3)?,
            (Some(t1), Some(t2), None) => write!(f, "{}min, {}min, -", t1, t2)?,
            (Some(t1), None, None) => write!(f, "{}min, -, -", t1)?,
            _ => write!(f, "Not attempted")?,
        }

        if let Some(comments) = &self.comments {
            write!(f, "\n  Comments: {}", comments)?;
        }

        if self.should_solve_again {
            write!(f, "\n  [REVIEW NEEDED]")?;
        }

        Ok(())
    }
}

fn from_row(row: &Row) -> Result<Problem> {
    Ok(Problem {
        id: row.get(0)?,
        description: row.get(1)?,
        link: row.get(2)?,
        category: row.get(3)?,
        pattern: row.get(4)?,
        difficulty: row.get(5)?,
        time_to_solve_1st: row.get(6)?,
        time_to_solve_2nd: row.get(7)?,
        time_to_solve_3rd: row.get(8)?,
        comments: row.get(9)?,
        should_solve_again: row.get::<_, i64>(10)? != 0,
    })
}

struct ProblemTracker {
    conn: Connection,
}

impl ProblemTracker {
    fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS problems (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                link TEXT,
                category TEXT,
                pattern TEXT,
                difficulty TEXT,
                time_to_solve_1st INTEGER,
                time_to_solve_2nd INTEGER,
                time_to_solve_3rd INTEGER,
                comments TEXT,
                should_solve_again INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;

        Ok(ProblemTracker { conn })
    }

    fn add_problem(&self, problem: Problem) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO problems (
                description, link, category, pattern, difficulty,
                time_to_solve_1st, time_to_solve_2nd, time_to_solve_3rd,
                comments, should_solve_again
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                problem.description,
                problem.link,
                problem.category,
                problem.pattern,
                problem.difficulty,
                problem.time_to_solve_1st,
                problem.time_to_solve_2nd,
                problem.time_to_solve_3rd,
                problem.comments,
                problem.should_solve_again as i64
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    fn update_solve_time(&self, id: i64, attempt: usize, minutes: i64) -> Result<()> {
        let column = match attempt {
            1 => "time_to_solve_1st",
            2 => "time_to_solve_2nd",
            3 => "time_to_solve_3rd",
            _ => {
                return Err(rusqlite::Error::InvalidParameterName(
                    "Attempt must be 1, 2, or 3".to_string(),
                ))
            }
        };

        let query = format!("UPDATE problems SET {} = ? WHERE id = ?", column);
        self.conn.execute(&query, params![minutes, id])?;

        Ok(())
    }

    fn toggle_review_flag(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE problems SET should_solve_again = NOT should_solve_again WHERE id = ?",
            params![id],
        )?;

        Ok(())
    }

    fn get_problem(&self, id: i64) -> Result<Problem> {
        self.conn
            .query_row("SELECT * FROM problems WHERE id = ?", params![id], |row| {
                from_row(row)
            })
    }

    fn get_all_problems(&self) -> Result<Vec<Problem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM problems ORDER BY id")?;
        let problem_iter = stmt.query_map([], |row| from_row(row))?;

        let mut problems = Vec::new();
        for problem_result in problem_iter {
            problems.push(problem_result?);
        }

        Ok(problems)
    }

    fn get_problems_to_review(&self) -> Result<Vec<Problem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM problems WHERE should_solve_again = 1")?;
        let problem_iter = stmt.query_map([], |row| from_row(row))?;

        let mut problems = Vec::new();
        for problem_result in problem_iter {
            problems.push(problem_result?);
        }

        Ok(problems)
    }

    fn get_problems_by_category(&self, category: &str) -> Result<Vec<Problem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM problems WHERE category = ?")?;
        let problem_iter = stmt.query_map(params![category], |row| from_row(row))?;

        let mut problems = Vec::new();
        for problem_result in problem_iter {
            problems.push(problem_result?);
        }

        Ok(problems)
    }

    fn get_problems_by_pattern(&self, pattern: &str) -> Result<Vec<Problem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM problems WHERE pattern = ?")?;
        let problem_iter = stmt.query_map(params![pattern], |row| from_row(row))?;

        let mut problems = Vec::new();
        for problem_result in problem_iter {
            problems.push(problem_result?);
        }

        Ok(problems)
    }

    fn get_problems_by_difficulty(&self, difficulty: &str) -> Result<Vec<Problem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM problems WHERE difficulty = ?")?;
        let problem_iter = stmt.query_map(params![difficulty], |row| from_row(row))?;

        let mut problems = Vec::new();
        for problem_result in problem_iter {
            problems.push(problem_result?);
        }

        Ok(problems)
    }

    fn search_problems(&self, keyword: &str) -> Result<Vec<Problem>> {
        let search_pattern = format!("%{}%", keyword);
        let mut stmt = self.conn.prepare(
            "SELECT * FROM problems WHERE 
            description LIKE ? OR 
            category LIKE ? OR 
            pattern LIKE ? OR 
            comments LIKE ?",
        )?;

        let problem_iter = stmt.query_map(
            params![
                search_pattern,
                search_pattern,
                search_pattern,
                search_pattern
            ],
            |row| from_row(row),
        )?;

        let mut problems = Vec::new();
        for problem_result in problem_iter {
            problems.push(problem_result?);
        }

        Ok(problems)
    }

    fn delete_problem(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM problems WHERE id = ?", params![id])?;
        Ok(())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Path to the SQLite database file
    #[arg(short, long, default_value = "problems.db")]
    database: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new problem
    Add {
        /// Problem description
        #[arg(required = true)]
        description: String,

        /// Problem link
        #[arg(short, long)]
        link: Option<String>,

        /// Problem category
        #[arg(short = 'C', long)]
        category: Option<String>,

        /// Problem pattern
        #[arg(short, long)]
        pattern: Option<String>,

        /// Problem difficulty
        #[arg(short, long)]
        difficulty: Option<String>,

        /// Time to solve (first attempt) in minutes
        #[arg(short, long)]
        time: Option<i64>,

        /// Comments about the problem
        #[arg(short, long)]
        comments: Option<String>,

        /// Should solve again
        #[arg(short, long)]
        review: bool,
    },
    /// Show a specific problem by ID
    Show {
        /// Problem ID
        id: i64,
    },
    /// List all problems
    List,
    /// List problems that need review
    Review,
    /// List problems by category
    ByCategory {
        /// Category name
        category: String,
    },
    /// List problems by pattern
    ByPattern {
        /// Pattern name
        pattern: String,
    },
    /// List problems by difficulty
    ByDifficulty {
        /// Difficulty level
        difficulty: String,
    },
    /// Search problems by keyword
    Search {
        /// Search keyword
        keyword: String,
    },
    /// Update a problem's solve time
    UpdateTime {
        /// Problem ID
        id: i64,

        /// Attempt number (1, 2, or 3)
        attempt: usize,

        /// Time to solve in minutes
        minutes: i64,
    },
    /// Toggle a problem's review flag
    ToggleReview {
        /// Problem ID
        id: i64,
    },
    /// Delete a problem
    Delete {
        /// Problem ID
        id: i64,

        /// Skip confirmation
        #[arg(short, long)]
        force: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let db_path = cli.database.to_string_lossy();
    let tracker = ProblemTracker::new(&db_path)?;

    match &cli.command {
        Commands::Add {
            description,
            link,
            category,
            pattern,
            difficulty,
            time,
            comments,
            review,
        } => {
            let mut problem = Problem::new(description);
            problem.link = link.clone();
            problem.category = category.clone();
            problem.pattern = pattern.clone();
            problem.difficulty = difficulty.clone();
            problem.time_to_solve_1st = *time;
            problem.comments = comments.clone();
            problem.should_solve_again = *review;

            let id = tracker.add_problem(problem)?;
            println!("Added problem with ID: {}", id);
        }
        Commands::Show { id } => match tracker.get_problem(*id) {
            Ok(problem) => println!("{}", problem),
            Err(_) => println!("Problem with ID {} not found", id),
        },
        Commands::List => {
            let problems = tracker.get_all_problems()?;
            if problems.is_empty() {
                println!("No problems found");
            } else {
                println!("All Problems ({})", problems.len());
                for problem in problems {
                    println!("\n{}", problem);
                }
            }
        }
        Commands::Review => {
            let problems = tracker.get_problems_to_review()?;
            if problems.is_empty() {
                println!("No problems to review");
            } else {
                println!("Problems to Review ({})", problems.len());
                for problem in problems {
                    println!("\n{}", problem);
                }
            }
        }
        Commands::ByCategory { category } => {
            let problems = tracker.get_problems_by_category(category)?;
            if problems.is_empty() {
                println!("No problems found in category '{}'", category);
            } else {
                println!("Problems in Category '{}' ({})", category, problems.len());
                for problem in problems {
                    println!("\n{}", problem);
                }
            }
        }
        Commands::ByPattern { pattern } => {
            let problems = tracker.get_problems_by_pattern(pattern)?;
            if problems.is_empty() {
                println!("No problems found with pattern '{}'", pattern);
            } else {
                println!("Problems with Pattern '{}' ({})", pattern, problems.len());
                for problem in problems {
                    println!("\n{}", problem);
                }
            }
        }
        Commands::ByDifficulty { difficulty } => {
            let problems = tracker.get_problems_by_difficulty(difficulty)?;
            if problems.is_empty() {
                println!("No problems found with difficulty '{}'", difficulty);
            } else {
                println!(
                    "Problems with Difficulty '{}' ({})",
                    difficulty,
                    problems.len()
                );
                for problem in problems {
                    println!("\n{}", problem);
                }
            }
        }
        Commands::Search { keyword } => {
            let problems = tracker.search_problems(keyword)?;
            if problems.is_empty() {
                println!("No problems found matching '{}'", keyword);
            } else {
                println!("Problems matching '{}' ({})", keyword, problems.len());
                for problem in problems {
                    println!("\n{}", problem);
                }
            }
        }
        Commands::UpdateTime {
            id,
            attempt,
            minutes,
        } => {
            if *attempt < 1 || *attempt > 3 {
                println!("Attempt must be 1, 2, or 3");
                return Ok(());
            }

            match tracker.update_solve_time(*id, *attempt, *minutes) {
                Ok(_) => println!(
                    "Updated problem #{} with attempt {} time: {} minutes",
                    id, attempt, minutes
                ),
                Err(_) => println!("Problem with ID {} not found", id),
            }
        }
        Commands::ToggleReview { id } => match tracker.toggle_review_flag(*id) {
            Ok(_) => match tracker.get_problem(*id) {
                Ok(problem) => println!(
                    "Problem #{} review flag set to: {}",
                    id,
                    if problem.should_solve_again {
                        "Yes"
                    } else {
                        "No"
                    }
                ),
                Err(_) => println!("Problem with ID {} not found", id),
            },
            Err(_) => println!("Problem with ID {} not found", id),
        },
        Commands::Delete { id, force } => {
            if !*force {
                println!("Are you sure you want to delete problem #{}? [y/N]", id);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Deletion cancelled");
                    return Ok(());
                }
            }

            match tracker.delete_problem(*id) {
                Ok(_) => println!("Deleted problem #{}", id),
                Err(_) => println!("Problem with ID {} not found", id),
            }
        }
    }

    Ok(())
}
