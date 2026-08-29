use crate::models::CommandDefinition;

pub struct Security;

impl Security {
    pub fn check_permission_required(cmd: &CommandDefinition) -> bool {
        cmd.requires_admin
    }

    pub fn check_dangerous(cmd: &CommandDefinition) -> bool {
        cmd.dangerous
    }

    pub fn validate_command_safety(cmd: &CommandDefinition) -> Result<(), String> {
        if cmd.dangerous {
            return Err(format!(
                "Command '{}' is marked as dangerous and requires explicit confirmation",
                cmd.id
            ));
        }
        Ok(())
    }
}
