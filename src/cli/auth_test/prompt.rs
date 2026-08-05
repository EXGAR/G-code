const DEFAULT_AUTH_TEST_PROVIDER_PROMPT: &str =
    "Reply with exactly AUTH_TEST_OK and nothing else. Do not call tools.";
const AUTH_TEST_TOOL_NAME: &str = "bash";
const AUTH_TEST_TOOL_COMMAND: &str = "echo JCODE_TOOL_OK";
const AUTH_TEST_TOOL_OUTPUT_MARKER: &str = "JCODE_TOOL_OK";
const DEFAULT_AUTH_TEST_TOOL_PROMPT: &str = "Use exactly one bash tool call with command exactly `echo JCODE_TOOL_OK`. After you see the tool result, reply with exactly AUTH_TEST_OK and nothing else.";

#[derive(Debug, PartialEq, Eq)]
struct AuthTestPromptPlan<'a> {
    provider_prompt: &'a str,
    tool_prompt: &'a str,
}

fn auth_test_prompt_plan(custom_prompt: Option<&str>) -> AuthTestPromptPlan<'_> {
    let provider_prompt = custom_prompt.unwrap_or(DEFAULT_AUTH_TEST_PROVIDER_PROMPT);
    AuthTestPromptPlan {
        provider_prompt,
        tool_prompt: DEFAULT_AUTH_TEST_TOOL_PROMPT,
    }
}

fn auth_test_expected_output(prompt: &str) -> &str {
    prompt
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .rev()
        .find(|token| {
            token.ends_with("_OK")
                && token.chars().all(|character| {
                    character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
                })
        })
        .unwrap_or("AUTH_TEST_OK")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_provider_prompt_sets_its_marker_without_replacing_tool_smoke_prompt() {
        let plan = auth_test_prompt_plan(Some("Reply exactly JCODE_PROVIDER_SETUP_OK"));

        assert_eq!(
            plan.provider_prompt,
            "Reply exactly JCODE_PROVIDER_SETUP_OK"
        );
        assert_eq!(
            auth_test_expected_output(plan.provider_prompt),
            "JCODE_PROVIDER_SETUP_OK"
        );
        assert_eq!(plan.tool_prompt, DEFAULT_AUTH_TEST_TOOL_PROMPT);
    }

    #[test]
    fn default_prompt_plan_keeps_auth_test_marker_and_strict_tool_prompt() {
        let plan = auth_test_prompt_plan(None);

        assert_eq!(plan.provider_prompt, DEFAULT_AUTH_TEST_PROVIDER_PROMPT);
        assert_eq!(
            auth_test_expected_output(plan.provider_prompt),
            "AUTH_TEST_OK"
        );
        assert_eq!(plan.tool_prompt, DEFAULT_AUTH_TEST_TOOL_PROMPT);
    }
}
