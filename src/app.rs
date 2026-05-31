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

#[derive(Clone, Debug)]
pub struct EditMenu {
    pub title: String,
    pub fields: Vec<(String, String)>, // (Label, Value)
    pub selected_idx: usize,
    pub entity_iid: u64,
    pub entity_type: String, // "issue", "mr"
}

#[derive(Clone, Debug)]
pub struct Selector {
    pub title: String,
    pub all_items: Vec<String>,
    pub selected_items: std::collections::HashSet<String>,
    pub cursor_idx: usize,
    pub search_query: String,
    pub is_filtering: bool,
    pub is_loading: bool,
    pub entity_iid: u64,
    pub entity_type: String, // "issue", "mr"
    pub field_type: String,  // "labels", "assignees", "reviewers", "milestone"
    pub multi_select: bool,
}

impl Selector {
    pub fn get_filtered_items(&self) -> Vec<String> {
        let query = self.search_query.to_lowercase();
        let mut items: Vec<String> = self.all_items.iter()
            .filter(|item| item.to_lowercase().contains(&query))
            .cloned()
            .collect();
            
        if !query.trim().is_empty() {
            let exact_match = self.all_items.iter().any(|item| item.to_lowercase() == query.trim());
            if !exact_match {
                items.insert(0, format!("+ Create \"{}\"", self.search_query.trim()));
            }
        }
        items
    }
}

#[derive(Clone, Debug)]
pub enum TextInputAction {
    EditField {
        entity_iid: u64,
        entity_type: String,
        field_type: String,
    },
    CreateIssue,
    CreateMr,
}

#[derive(Clone, Debug)]
pub struct TextInput {
    pub title: String,
    pub value: String,
    pub cursor_idx: usize,
    pub action: TextInputAction,
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
    pub loaded_tabs: std::collections::HashSet<Tab>,
    pub edit_menu: Option<EditMenu>,
    pub selector: Option<Selector>,
    pub text_input: Option<TextInput>,
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
            loaded_tabs: std::collections::HashSet::new(),
            edit_menu: None,
            selector: None,
            text_input: None,
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
