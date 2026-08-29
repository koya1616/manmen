use crate::models::{CommandDefinition, CommandCategory};
use std::collections::HashMap;
use std::path::Path;
use log::{info, warn};

pub struct CommandRegistry {
    commands: HashMap<String, CommandDefinition>,
    categories: Vec<CommandCategory>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            categories: Vec::new(),
        }
    }

    pub fn load_from_dir(&mut self, commands_dir: &Path) -> Result<(), String> {
        if !commands_dir.exists() {
            warn!("Commands directory not found: {:?}", commands_dir);
            return Ok(());
        }

        let mut categories: HashMap<String, CommandCategory> = HashMap::new();

        for entry in std::fs::read_dir(commands_dir)
            .map_err(|e| format!("Failed to read commands directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                let category_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                categories.entry(category_name.clone()).or_insert_with(|| CommandCategory {
                    id: category_name.clone(),
                    name: category_name
                        .chars()
                        .next()
                        .map(|c| c.to_uppercase().to_string() + &category_name[1..])
                        .unwrap_or_default(),
                    description: None,
                });

                self.load_commands_from_dir(&path, &category_name)?;
            }
        }

        self.categories = categories.into_values().collect();
        self.categories.sort_by(|a, b| a.name.cmp(&b.name));

        info!(
            "Loaded {} commands in {} categories",
            self.commands.len(),
            self.categories.len()
        );

        Ok(())
    }

    fn load_commands_from_dir(
        &mut self,
        dir: &Path,
        category: &str,
    ) -> Result<(), String> {
        for entry in std::fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                match self.load_command_file(&path, category) {
                    Ok(cmd) => {
                        info!("Loaded command: {}", cmd.id);
                        self.commands.insert(cmd.id.clone(), cmd);
                    }
                    Err(e) => {
                        warn!("Failed to load command from {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    fn load_command_file(&self, path: &Path, category: &str) -> Result<CommandDefinition, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let mut cmd: CommandDefinition = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        if cmd.category.is_empty() {
            cmd.category = category.to_string();
        }

        Ok(cmd)
    }

    pub fn get_all_commands(&self) -> Vec<&CommandDefinition> {
        self.commands.values().collect()
    }

    pub fn get_command(&self, id: &str) -> Option<&CommandDefinition> {
        self.commands.get(id)
    }

    pub fn get_categories(&self) -> &[CommandCategory] {
        &self.categories
    }

    pub fn search(&self, query: &str) -> Vec<&CommandDefinition> {
        let query_lower = query.to_lowercase();

        let mut results: Vec<(&CommandDefinition, u32)> = self
            .commands
            .values()
            .filter_map(|cmd| {
                let score = self.calculate_relevance(cmd, &query_lower);
                if score > 0 {
                    Some((cmd, score))
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.1.cmp(&a.1));
        results.into_iter().map(|(cmd, _)| cmd).collect()
    }

    fn calculate_relevance(&self, cmd: &CommandDefinition, query: &str) -> u32 {
        let mut score = 0;

        if cmd.command.to_lowercase() == query {
            score += 100;
        } else if cmd.command.to_lowercase().starts_with(query) {
            score += 80;
        }

        if cmd.name.to_lowercase() == query {
            score += 60;
        } else if cmd.name.to_lowercase().contains(query) {
            score += 40;
        }

        if cmd.description.to_lowercase().contains(query) {
            score += 20;
        }

        if cmd.id.to_lowercase().contains(query) {
            score += 15;
        }

        for tag in &cmd.tags {
            if tag.to_lowercase().contains(query) {
                score += 10;
            }
        }

        for arg in &cmd.arguments {
            if arg.id.to_lowercase().contains(query)
                || arg.name.to_lowercase().contains(query)
            {
                score += 5;
            }
        }

        score
    }
}
