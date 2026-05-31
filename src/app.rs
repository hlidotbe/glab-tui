use crate::utils::ui::StatefulTable;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tab {
    #[default]
    Issues,
    MergeRequests,
    Pipelines,
    Runners,
    Releases,
}

impl Tab {
    pub const ALL: [Tab; 5] = [
        Tab::Issues,
        Tab::MergeRequests,
        Tab::Pipelines,
        Tab::Runners,
        Tab::Releases,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            Tab::Issues => "Issues",
            Tab::MergeRequests => "MRs",
            Tab::Pipelines => "Pipelines",
            Tab::Runners => "Runners",
            Tab::Releases => "Releases",
        }
    }
}

pub struct App {
    pub active_tab: Tab,
    pub running: bool,
    pub project_context: String,
    pub gitlab_client: Option<crate::gitlab::client::GitlabClient>,
    pub issues: StatefulTable<crate::gitlab::issues::Issue>,
    pub mrs: StatefulTable<crate::gitlab::mr::MergeRequest>,
    pub pipelines: StatefulTable<crate::gitlab::pipelines::Pipeline>,
    pub search_query: String,
    pub is_typing_search: bool,
    pub selected_pipeline_jobs: Option<Vec<crate::gitlab::pipelines::Job>>,
    pub selected_job_index: Option<usize>,
    pub job_trace: Option<String>,
    pub error_message: Option<String>,
    pub runners: StatefulTable<crate::gitlab::runners::Runner>,
    pub releases: StatefulTable<crate::gitlab::releases::Release>,
    pub pipeline_jobs: std::collections::HashMap<u64, Vec<crate::gitlab::pipelines::Job>>,
    pub fetching_pipelines: std::collections::HashSet<u64>,
    pub loading_tabs: std::collections::HashSet<Tab>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_tab: Tab::default(),
            running: true,
            project_context: "group/repository".to_string(),
            gitlab_client: None,
            issues: StatefulTable::with_items(vec![]),
            mrs: StatefulTable::with_items(vec![]),
            pipelines: StatefulTable::with_items(vec![]),
            search_query: String::new(),
            is_typing_search: false,
            selected_pipeline_jobs: None,
            selected_job_index: None,
            job_trace: None,
            error_message: None,
            runners: StatefulTable::with_items(vec![]),
            releases: StatefulTable::with_items(vec![]),
            pipeline_jobs: std::collections::HashMap::new(),
            fetching_pipelines: std::collections::HashSet::new(),
            loading_tabs: std::collections::HashSet::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_tab(&mut self) {
        let current_index = Tab::ALL.iter().position(|t| t == &self.active_tab).unwrap_or(0);
        let next_index = (current_index + 1) % Tab::ALL.len();
        self.active_tab = Tab::ALL[next_index];
    }

    pub fn previous_tab(&mut self) {
        let current_index = Tab::ALL.iter().position(|t| t == &self.active_tab).unwrap_or(0);
        let prev_index = if current_index == 0 {
            Tab::ALL.len() - 1
        } else {
            current_index - 1
        };
        self.active_tab = Tab::ALL[prev_index];
    }
}
